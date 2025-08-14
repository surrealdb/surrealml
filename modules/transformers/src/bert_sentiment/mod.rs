//! Sentiment‑analysis **utility layer** wrapping the low‑level BERT classifier
//! in [`crate::bert_sentiment::model`].  The goal is to provide a _turn‑key_
//! helper: *embed the weights in the binary, load them lazily, and expose a
//! few ergonomic functions for preprocessing and post‑processing*.
//!
//! ```no_run
//! use surrealml_tokenizers::{load_local_tokenizer, PresetTokenizers};
//! use crate::bert_sentiment::{encoding_to_tensors, get_sentiment_model};
//! use candle_core::Device;
//!
//! // 1 Build model & tokenizer.
//! let model     = get_sentiment_model()?;                 // lazy‑loaded weights
//! let tokenizer = load_local_tokenizer(
//!     PresetTokenizers::BertBaseUncased.to_string(),
//! )?;
//!
//! // 2 Encode text → tensors.
//! let enc  = tokenizer.encode("I love this movie!", true).unwrap();
//! let (ids, _type_ids, mask) = encoding_to_tensors(&enc, &Device::Cpu)?;
//!
//! // 3 Forward pass.
//! let logits = model.predict(&ids, &mask)?;               // (1, 2)
//! ```
//!
//! ## Embedded assets
//! * `BERT_MODEL`   – JSON config identical to HuggingFace's `config.json`.
//! * `BERT_TENSORS` – Binary **safe‑tensors** weights (`.safetensors`).
//!
//! They live in `transformers/sent_two_*` and are pulled in at compile‑time via
//! `include_str!` / `include_bytes!`.  This makes the crate **self‑contained**
//! and eliminates any runtime file I/O.
//!
//! ## Public helpers
//! | Function                 | Purpose                                                |
//! |------------------------- |------------------------------------------------------- |
//! | `get_sentiment_model`    | Build a ready‑to‑run [`BertForSequenceClassification`] |
//! | `load_label_map`         | Extract `{id → label}` mapping from the JSON config    |
//! | `encoding_to_tensors`    | Convert *tokenizers‑rs* [`Encoding`] into Candle input |
//!
//! ---

pub mod pooler;
pub mod model;

use std::collections::HashMap;

use candle_core::{Device, DType, Result as CandleResult, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::Config;
use serde_json;
use tokenizers::Encoding;

pub use candle_nn::ops::softmax; // re‑export for convenience in demos/tests

/// BERT configuration (`config.json`) embedded at compile‑time.
const BERT_MODEL: &str = include_str!("../../transformers/sent_two_config.json");
/// Pre‑trained weights (`model.safetensors`) embedded at compile‑time.
const BERT_TENSORS: &[u8] = include_bytes!("../../transformers/sent_two_model.safetensors");

/// Build a **ready‑to‑inference** [`model::BertForSequenceClassification`].
///
/// The procedure mirrors the HuggingFace loader:
/// 1. Read the JSON config (`BERT_MODEL`).
/// 2. Load the tensors from the in‑memory safetensors buffer.
/// 3. Duplicate LayerNorm `{weight,bias}` → `{gamma,beta}` for Candle.
/// 4. Ensure embedding LayerNorm exists (older checkpoints omit it).
/// 5. Feed everything into a [`VarBuilder`] and instantiate the model.
pub fn get_sentiment_model() -> anyhow::Result<model::BertForSequenceClassification> {
    // 1 Device – CPU by default; swap for `Device::cuda_if_available()` etc.
    let device = Device::Cpu;

    // 2 Parse BERT config.
    let cfg: Config = serde_json::from_str(BERT_MODEL)?;

    // 3 Load tensors from the embedded weights.
    let mut tensors = candle_core::safetensors::load_buffer(BERT_TENSORS, &device)?;

    // 4 Duplicate LayerNorm parameters so both naming schemes are present.
    let mut extra = Vec::new();
    for (name, t) in tensors.iter() {
        if name.ends_with(".LayerNorm.weight") {
            extra.push((name.replace(".weight", ".gamma"), t.clone()));
        } else if name.ends_with(".LayerNorm.bias") {
            extra.push((name.replace(".bias", ".beta"), t.clone()));
        }
    }
    tensors.extend(extra);

    // 5 Ensure embedding‑norm tensors exist – some BERT variants omit them.
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

    // 6 Wrap everything in a VarBuilder and instantiate the classifier.
    let vb = VarBuilder::from_tensors(tensors, DType::F32, &device);
    let model = model::BertForSequenceClassification::load(vb, &cfg, /*num_labels=*/ 2)?;

    Ok(model)
}

/// Return `{class_id → human‑readable label}` as defined in the JSON config.
///
/// When fine‑tuning your own checkpoint simply ensure the `id2label` field is
/// present and this helper will pick it up automatically.
pub fn load_label_map() -> HashMap<usize, String> {
    let meta: serde_json::Value = serde_json::from_str(BERT_MODEL).unwrap_or_default();
    meta.get("id2label")
        .and_then(|m| m.as_object())
        .map(|m| {
            m.iter()
                .filter_map(|(k, v)| {
                    k.parse::<usize>().ok().zip(v.as_str().map(|s| s.to_owned()))
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Convert a *tokenizers‑rs* [`Encoding`] into Candle tensors:
/// * **`ids`** – token IDs `(1, seq_len)`  – `DType::U32`
/// * **`type_ids`** – segment IDs `(1, seq_len)` – always zero for single‑sentence tasks
/// * **`mask`** – attention mask `(1, seq_len)` – `DType::F32`
///
/// Returns a tuple `(ids, type_ids, mask)` ready for [`model::BertForSequenceClassification::forward`].
pub fn encoding_to_tensors(
    enc: &Encoding,
    device: &Device,
) -> CandleResult<(Tensor, Tensor, Tensor)> {
    let ids = enc.get_ids();
    let mask = enc.get_attention_mask();
    let len = ids.len();

    // IDs & type‑ids stay in their original integer dtype (U32).
    let ids_tensor = Tensor::from_slice(ids, (1, len), device)?;          // U32
    let type_ids = Tensor::zeros((1, len), DType::U32, device)?;          // U32

    // Attention mask is expected as F32 by Candle's BERT implementation.
    let mask_tensor = Tensor::from_slice(mask, (1, len), device)?.to_dtype(DType::F32)?;

    Ok((ids_tensor, type_ids, mask_tensor))
}


#[cfg(test)]
mod tests {
    use super::*;
    use surrealml_tokenizers::{load_local_tokenizer, PresetTokenizers};

    /// The model should classify clear‑cut positive/negative phrases correctly
    /// with high confidence (>95 %).
    #[test]
    fn test_extreme_sentiment() -> anyhow::Result<()> {
        let model = get_sentiment_model()?;
        let tokenizer = load_local_tokenizer(PresetTokenizers::BertBaseUncased.to_string())?;

        let samples = [
            ("I absolutely love this fantastic wonderful amazing incredible movie", "positive"),
            ("I completely hate this terrible awful horrible disgusting worst movie", "negative"),
            ("This movie is good", "positive"),
            ("This movie is bad", "negative"),
            ("okay", "positive"),
            ("great", "positive"),
            ("terrible", "negative"),
        ];

        for (text, expected_label) in samples.iter() {

            // here is where the host code runs with the text from the WASM module
            let enc = tokenizer.encode(*text, true).unwrap();
            let (ids, _, mask) = encoding_to_tensors(&enc, &Device::Cpu)?;
            let logits = model.predict(&ids, &mask)?;

            // Convert logits → probabilities.
            let probs = softmax(&logits, 1)?.squeeze(0)?.to_vec1::<f32>()?;
            let (pred_idx, confidence) = probs
                .iter()
                .copied()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();

            // this is what you return to the user's WASM module
            let predicted_label = if pred_idx == 0 { "negative" } else { "positive" };

            // --- Assertions --------------------------------------------------
            assert_eq!(predicted_label, *expected_label, "{}", text);
            assert!(confidence > 0.95, "low confidence {:.2} for \"{}\"", confidence, text);
        }

        Ok(())
    }
}
