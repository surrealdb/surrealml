
// const BERT_MODEL: &str = include_str!("../transformers/config_bert.json");
// const BERT_TENSORS: &[u8] = include_bytes!("../transformers/model_bert.safetensors");

const BERT_MODEL:   &str  = include_str!("../transformers/sent_two_config.json");
const BERT_TENSORS: &[u8] = include_bytes!("../transformers/sent_two_model.safetensors");

use anyhow::Result;
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config, HiddenAct};
use serde_json;
use candle_core::{Tensor, Device, DType, Result as CandleResult};
use candle_nn::{Linear, Dropout};
use candle_nn::Module;
use candle_nn::ModuleT;
use candle_nn::linear;
use tokenizers::Encoding;
use candle_core::IndexOp;
use std::collections::HashMap;
use safetensors::SafeTensors;

// Assuming your PresetTokenizers enum and retrieve_tokenizer are accessible
use surrealml_tokenizers::{Tokenizer, load_local_tokenizer, PresetTokenizers};


fn load_label_map() -> HashMap<usize, String> {
    let meta: serde_json::Value =
        serde_json::from_str(BERT_MODEL).unwrap_or_default();

    meta.get("id2label")
        .and_then(|m| m.as_object())
        .map(|m| {
            m.iter()
             .filter_map(|(k, v)| {
                 k.parse::<usize>().ok()
                  .zip(v.as_str().map(|s| s.to_owned()))
             })
             .collect()
        })
        .unwrap_or_default()
}

impl BertForSequenceClassification {
    pub fn load(
        vb: VarBuilder,
        cfg: &Config,
        num_labels: usize,
    ) -> CandleResult<Self> {
        // ❶ Encoder ── use the "bert" sub‑view so the prefixes match
        let bert = BertModel::load(vb.pp("bert"), cfg)?;      // ← change is here

        // ❷ Classifier head (unchanged)
        // let classifier = match linear(
        //     cfg.hidden_size as usize,
        //     num_labels,
        //     vb.pp("classifier"),          // weights live at the root as before
        // ) {
        //     Ok(layer) => layer,
        //     Err(_) => {
        //         let dev = vb.device();
        //         let w   = Tensor::zeros((num_labels, cfg.hidden_size as usize), DType::F32, dev)?;
        //         let b   = Tensor::zeros(num_labels, DType::F32, dev)?;
        //         Linear::new(w, Some(b))
        //     }
        // };

        let classifier = linear(
            cfg.hidden_size as usize,
            num_labels,
            vb.pp("classifier")
        ).expect("to load classifier");

        let dropout = Dropout::new(cfg.hidden_dropout_prob as f32);
        Ok(Self { bert, dropout, classifier })
    }

    /// logits `[batch, num_labels]`
    pub fn predict(&self, ids: &Tensor, mask: &Tensor) -> CandleResult<Tensor> {
        // single‑sentence input → all‑zero segment‑ids
        let type_ids = Tensor::zeros_like(ids)?;

        // last_hidden: [B, S, H]  (S = sequence length)
        let last_hidden = self.bert.forward(ids, &type_ids, Some(mask))?;

        // ── keep only the CLS vector (token 0) ──────────────────────────────
        // narrow dim‑1 to length 1, then squeeze that dim → [B, H]
        let cls = last_hidden        // [B, S, H]
            .narrow(1, 0, 1)?        // [B, 1, H]
            .squeeze(1)?;            // [B, H]

        // dropout (disabled at inference) + linear head
        let x = self.dropout.forward_t(&cls, /*train=*/false)?;
        self.classifier.forward(&x)          // logits [B, 2]
    }

    /// logits `[batch, num_labels]`
    pub fn forward(&self, ids: &Tensor, mask: &Tensor) -> CandleResult<Tensor> {
        let type_ids = Tensor::zeros_like(ids)?;                 // segment‑ids = 0
        let pooled   = self.bert.forward(ids, &type_ids, Some(mask))?; // [B, H] – CLS pool
        let x        = self.dropout.forward_t(&pooled, false)?;  // inference ⇒ train = false
        self.classifier.forward(&x)                              // [B, 2]
    }
}




pub struct BertForSequenceClassification {
    bert: BertModel,
    dropout: Dropout,
    classifier: Linear, // 768 → num_labels
}



pub fn get_preset_transformer(_tok: PresetTokenizers) -> anyhow::Result<BertModel> {
    let device = Device::Cpu;

    let cfg: Config = serde_json::from_str(BERT_MODEL)?;
    let weights = candle_core::safetensors::load_buffer(BERT_TENSORS, &device)?;

    let vb = VarBuilder::from_tensors(weights, DType::F32, &device)
    // The renames are to allow the weight names from BERT to be run by candel safe tensors so we loop through
    // the weights to handle this 
    .rename_f(|n: &str| {
        if n.contains(".LayerNorm.") {
            n.replace(".weight", ".gamma")
             .replace(".bias",  ".beta")
        } else {
            n.to_owned()          // leave everything else unchanged
        }
    })
    .pp("bert");
    let model = BertModel::load(vb, &cfg)?;   // <- should succeed now
    Ok(model)
}

pub fn get_sentiment_model() -> anyhow::Result<BertForSequenceClassification> {
    use candle_core::{DType, Tensor};

    // ── 0. set‑up ───────────────────────────────────────────────────────────
    let device = Device::Cpu;
    let cfg: Config = serde_json::from_str(BERT_MODEL)?;

    // ── 1. read every tensor in the embedded .safetensors file ──────────────
    let mut tensors = candle_core::safetensors::load_buffer(BERT_TENSORS, &device)?;
    // keys look like  "bert.embeddings.word_embeddings.weight", …, "classifier.bias"

    // ── 2. duplicate LayerNorm {weight,bias} → {gamma,beta} (keep prefix!) ──
    let mut extra = Vec::new();
    for (name, t) in tensors.iter() {
        if name.ends_with(".LayerNorm.weight") {
            extra.push((name.replace(".weight", ".gamma"), t.clone()));
        } else if name.ends_with(".LayerNorm.bias") {
            extra.push((name.replace(".bias",  ".beta"),  t.clone()));
        }
    }
    tensors.extend(extra);

    // ── 3. make sure the *embedding* LayerNorm tensors exist ────────────────
    let h = cfg.hidden_size as usize;
    if !tensors.contains_key("bert.embeddings.LayerNorm.gamma") {
        tensors.insert(
            "bert.embeddings.LayerNorm.gamma".into(),
            Tensor::ones(h, DType::F32, &device)?,
        );
    }
    if !tensors.contains_key("bert.embeddings.LayerNorm.beta") {
        tensors.insert(
            "bert.embeddings.LayerNorm.beta".into(),
            Tensor::zeros(h, DType::F32, &device)?,
        );
    }

    // ── 4. build the VarBuilder (root view, no extra .pp here) ──────────────
    let vb = VarBuilder::from_tensors(tensors, DType::F32, &device);

    // BertForSequenceClassification::load internally uses
    //     - `vb.pp("bert")`       → expects keys that start with "bert."
    //     - `vb.pp("classifier")` → expects "classifier.*"
    //
    // That matches the map we just built, so loading will now succeed.
    let model = BertForSequenceClassification::load(vb, &cfg, /*num_labels=*/ 2)?;

    Ok(model)
}



pub fn get_sent_sentiment_model() -> anyhow::Result<BertForSequenceClassification> {
    let device = Device::Cpu;
    let cfg: Config = serde_json::from_str(BERT_MODEL)?;
    let weights = candle_core::safetensors::load_buffer(BERT_TENSORS, &device)?;

    // Only rename LayerNorm {weight,bias} → {gamma,beta}
    let vb = VarBuilder::from_tensors(weights, DType::F32, &device)
        .rename_f(|n| {
            if n.contains(".LayerNorm.") {
                n.replace(".weight", ".gamma").replace(".bias", ".beta")
            } else { n.to_owned() }
        });

    // model.safetensors already contains `classifier.weight` & `bias`
    Ok(BertForSequenceClassification::load(vb, &cfg, 2)?)
}



fn encoding_to_tensors(
    enc: &Encoding,
    device: &Device,
) -> CandleResult<(Tensor, Tensor, Tensor)> {
    let ids   = enc.get_ids();
    let mask  = enc.get_attention_mask();
    let len   = ids.len();

    let ids  = Tensor::from_slice(ids,  (1, len), device)?;
    let mask = Tensor::from_slice(mask, (1, len), device)?;
    // single‑sentence input → segment‑ids are all‑zeros
    let type_ids = Tensor::zeros((1, len), DType::U32, device)?;

    Ok((ids, type_ids, mask))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_preset_transformer() {
        // println!("Embedded BERT_MODEL tensors content: '{}'", BERT_TENSORS);
        let outcome = get_preset_transformer(PresetTokenizers::BertBaseUncased).unwrap();
    }

    #[test]
    fn test_sentiment() -> anyhow::Result<()> {
        use candle_nn::ops::softmax;
        // https://chatgpt.com/c/687e0bff-2c68-8003-b32e-7f5af4f183a1
        // ❶ classifier
        let model = get_sentiment_model()?;             // BertForSequenceClassification
        let w         = model.classifier.weight();          // &Tensor
        let mean_abs  = w.abs()?.mean_all()?.to_scalar::<f32>()?;
        println!("mean |w| = {mean_abs:.4}");
        if let Some(b) = model.classifier.bias() {
            println!("bias mean: {:.4}", b.mean_all()?.to_scalar::<f32>()?);
        }

        // ❷ tokenize
        let tokenizer = load_local_tokenizer(PresetTokenizers::BertBaseUncased.to_string())?;
        let text = "everything is amazing";
        let enc  = tokenizer.encode(text, true).unwrap();

        // ❸ tensors
        let (ids, _, mask) = encoding_to_tensors(&enc, &Device::Cpu)?;

        // ❹ forward pass (wrapper returns logits [1, 2])
        let logits = model.predict(&ids, &mask)?;       // one call
        println!("logits: {:?}", logits.squeeze(0)?);

        let probs_vec = softmax(&logits, 1)?            // [1, 2]
            .squeeze(0)?                                // [2]
            .to_vec1::<f32>()?;                         // Vec<f32>

        let (pred, confidence) = probs_vec
            .iter()
            .copied()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();

        let label_map = load_label_map();
        let pretty = |id: usize| {
            match label_map.get(&id) {
                Some(lbl) if lbl.eq_ignore_ascii_case("positive") => "positive",
                Some(lbl) if lbl.eq_ignore_ascii_case("negative") => "negative",
                // fallback when labels are "LABEL_0"/"LABEL_1" or map is empty
                _ => if id == 0 { "positive" } else { "negative" },
            }
        };
        println!(
            "\"{text}\" → {} ({:.1} %)",
            pretty(pred),          // <-- call the helper
            confidence * 100.0
        );
        Ok(())
    }


    #[test]
    fn print_tensor_names() {
        use safetensors::SafeTensors;
        let st = SafeTensors::deserialize(BERT_TENSORS).unwrap();
        assert!(st.names().into_iter().any(|n| n.contains("embeddings.LayerNorm"))); // sanity
        for n in st.names().into_iter().take(10) { println!("{n}"); }
    }

    #[test]
    fn test_tensors() -> Result<(), Box<dyn std::error::Error>> {
        let st = SafeTensors::deserialize(BERT_TENSORS)?;
        assert!(
            st.names().iter().any(|n| n == &"classifier.weight"),
            "`classifier.weight` is missing – the head was re-initialised!"
        );
        assert!(
            st.names().iter().any(|n| n == &"classifier.bias"),
            "`classifier.bias` is missing - the head was re-initialised!"
        );
        Ok(())
    }
}

