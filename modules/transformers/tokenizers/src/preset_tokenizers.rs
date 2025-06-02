//! Utilities for working with **preset** tokenizers.
use tokenizers::Tokenizer;
use std::str::FromStr;
use crate::error::{SurrealError, SurrealErrorStatus};


/// Tokenizer JSON specifications embedded in the binary.
const GPT2_JSON: &'static str = include_str!("../tokenizers/gpt2-tokenizer.json");
const DISTILGPT2_JSON: &'static str = include_str!("../tokenizers/distilgpt2-tokenizer.json");
const GPT_NEO_125M_JSON: &'static str = include_str!("../tokenizers/EleutherAI-gpt-neo-125M-tokenizer.json");
const BERT_UNCASED_JSON: &'static str = include_str!("../tokenizers/bert-base-uncased-tokenizer.json");


/// Identifiers for the built‑in tokenizers bundled with Surreal.
///
/// # Variants
/// * `Gpt2` — OpenAI GPT‑2 vocabulary.
/// * `DistilGpt2` — DistilGPT‑2 vocabulary.
/// * `GptNeo125M` — EleutherAI GPT‑Neo‑125M vocabulary.
/// * `BertBaseUncased` — BERT‑Base‑Uncased WordPiece vocabulary.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PresetTokenizers {
    Gpt2,
    DistilGpt2,
    GptNeo125M,
    BertBaseUncased,
}

impl PresetTokenizers {
    /// Convert a canonical model string to a [`PresetTokenizers`] variant.
    ///
    /// # Arguments
    /// * `model` – Model identifier used on the model hub (e.g. `"gpt2"`).
    ///
    /// # Returns
    /// * `Some(variant)` when the identifier is recognised.
    /// * `None` for unknown identifiers.
    pub fn from_str(model: &str) -> Option<Self> {
        match model {
            "gpt2" => Some(PresetTokenizers::Gpt2),
            "distilgpt2" => Some(PresetTokenizers::DistilGpt2),
            "gpt-neo-125m" => Some(PresetTokenizers::GptNeo125M),
            "bert-base-uncased" => Some(PresetTokenizers::BertBaseUncased),
            _ => None,
        }
    }

    /// Build a [`Tokenizer`] from the embedded JSON specification.
    ///
    /// # Returns
    /// * `Result<Tokenizer, SurrealError> - A fully initialised [`Tokenizer`] ready for encoding and decoding.
    pub fn retrieve_tokenizer(&self) -> Result<Tokenizer, SurrealError> {
        let data: &'static str = match self {
            PresetTokenizers::Gpt2 => GPT2_JSON,
            PresetTokenizers::DistilGpt2 => DISTILGPT2_JSON,
            PresetTokenizers::GptNeo125M => GPT_NEO_125M_JSON,
            PresetTokenizers::BertBaseUncased => BERT_UNCASED_JSON,
        };

        Ok(Tokenizer::from_str(data).map_err(|e| {
            SurrealError::new(
                format!("Failed to parse preset tokenizer: {}", e),
                SurrealErrorStatus::BadRequest,
            )
        })?)
    }
}


#[cfg(test)]
mod tests {
    use super::PresetTokenizers;
    use crate::error::{SurrealError, SurrealErrorStatus};
    use std::str::FromStr;     
    
    #[test]
    fn from_str_recognises_valid_model_names() {
        // Each known model string should map to the correct enum variant.
        assert_eq!(PresetTokenizers::from_str("gpt2"), Some(PresetTokenizers::Gpt2));
        assert_eq!(
            PresetTokenizers::from_str("distilgpt2"),
            Some(PresetTokenizers::DistilGpt2)
        );
        assert_eq!(
            PresetTokenizers::from_str("gpt-neo-125m"),
            Some(PresetTokenizers::GptNeo125M)
        );
        assert_eq!(
            PresetTokenizers::from_str("bert-base-uncased"),
            Some(PresetTokenizers::BertBaseUncased)
        );
    }

    #[test]
    fn from_str_unknown_model_returns_none() {
        // An unsupported model string should yield None, not panic.
        assert_eq!(PresetTokenizers::from_str("some-random-model"), None);
    }

    #[test]
    fn presets_load_successfully() {
        let presets = [
            PresetTokenizers::Gpt2,
            PresetTokenizers::DistilGpt2,
            PresetTokenizers::GptNeo125M,
            PresetTokenizers::BertBaseUncased,
        ];

        for preset in presets {
            // Should produce Ok(Tokenizer)
            let tok = preset
                .retrieve_tokenizer()
                .expect("preset tokenizer must load");

            // Sanity: tokenizer should yield at least one token for a short input
            let enc = tok.encode("test", true).unwrap();
            assert!(
                !enc.get_ids().is_empty(),
                "{preset:?} produced empty token sequence"
            );
        }
    }
}

