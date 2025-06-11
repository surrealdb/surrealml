//! Utilities for working with **preset** Hugging Face model identifiers.
use crate::error::{SurrealError, SurrealErrorStatus};
use std::str::FromStr;
use tokenizers::Tokenizer;

/// Model identifier files embedded in the binary.
const MIXTRAL_8X7B_V01: &'static str =
    include_str!("../tokenizers/mistralai-Mixtral-8x7B-v0.1-tokenizer.json");
const MISTRAL_7B_V01: &'static str =
    include_str!("../tokenizers/mistralai-Mistral-7B-v0.1-tokenizer.json");
const MISTRALLITE: &'static str = include_str!("../tokenizers/amazon-MistralLite-tokenizer.json");
const GEMMA_7B: &'static str = include_str!("../tokenizers/google-gemma-7b-tokenizer.json");
const GEMMA_2B: &'static str = include_str!("../tokenizers/google-gemma-2b-tokenizer.json");
const GEMMA_3_4B_IT: &'static str =
    include_str!("../tokenizers/google-gemma-3-4b-it-tokenizer.json");
const FALCON_7B: &'static str = include_str!("../tokenizers/tiiuae-falcon-7b-tokenizer.json");

/// Identifiers for the built-in models bundled with this crate.
///
/// # Variants
/// * `Mixtral8x7Bv01` — `mistralai/Mixtral-8x7B-v0.1`  
/// * `Mistral7Bv01` — `mistralai/Mistral-7B-v0.1`  
/// * `MistralLite` — `amazon/MistralLite`  
/// * `Gemma7B` — `google/gemma-7b`  
/// * `Gemma2B` — `google/gemma-2b`  
/// * `Gemma3_4BIt` — `google/gemma-3-4b-it`  
/// * `Falcon7B` — `tiiuae/falcon-7b`
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PresetTokenizers {
    Mixtral8x7Bv01,
    Mistral7Bv01,
    MistralLite,
    Gemma7B,
    Gemma2B,
    Gemma3_4BIt,
    Falcon7B,
}

impl PresetTokenizers {
    /// Convert a canonical model string to a [`PresetTokenizers`] variant.
    ///
    /// # Arguments
    /// * `model` – Model identifier used on the model hub (e.g. `"mistralai/Mixtral-8x7B-v0.1"`).
    ///
    /// # Returns
    /// * `Some(variant)` when the identifier is recognised.
    /// * `None` for unknown identifiers.
    pub fn from_str(model: &str) -> Option<Self> {
        match model {
            "mistralai/Mixtral-8x7B-v0.1" => Some(PresetTokenizers::Mixtral8x7Bv01),
            "mistralai/Mistral-7B-v0.1" => Some(PresetTokenizers::Mistral7Bv01),
            "amazon/MistralLite" => Some(PresetTokenizers::MistralLite),
            "google/gemma-7b" => Some(PresetTokenizers::Gemma7B),
            "google/gemma-2b" => Some(PresetTokenizers::Gemma2B),
            "google/gemma-3-4b-it" => Some(PresetTokenizers::Gemma3_4BIt),
            "tiiuae/falcon-7b" => Some(PresetTokenizers::Falcon7B),
            _ => None,
        }
    }

    /// Retrieve the embedded tokenizer identifier for this variant.
    ///
    /// # Returns
    /// * `Result<Tokenizer, SurrealError> - A fully initialised [`Tokenizer`] ready for encoding and decoding.
    pub fn retrieve_tokenizer(&self) -> Result<Tokenizer, SurrealError> {
        let data: &'static str = match self {
            PresetTokenizers::Mixtral8x7Bv01 => MIXTRAL_8X7B_V01,
            PresetTokenizers::Mistral7Bv01 => MISTRAL_7B_V01,
            PresetTokenizers::MistralLite => MISTRALLITE,
            PresetTokenizers::Gemma7B => GEMMA_7B,
            PresetTokenizers::Gemma2B => GEMMA_2B,
            PresetTokenizers::Gemma3_4BIt => GEMMA_3_4B_IT,
            PresetTokenizers::Falcon7B => FALCON_7B,
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
        assert_eq!(
            PresetTokenizers::from_str("mistralai/Mixtral-8x7B-v0.1"),
            Some(PresetTokenizers::Mixtral8x7Bv01)
        );
        assert_eq!(
            PresetTokenizers::from_str("mistralai/Mistral-7B-v0.1"),
            Some(PresetTokenizers::Mistral7Bv01)
        );
        assert_eq!(
            PresetTokenizers::from_str("amazon/MistralLite"),
            Some(PresetTokenizers::MistralLite)
        );
        assert_eq!(
            PresetTokenizers::from_str("google/gemma-7b"),
            Some(PresetTokenizers::Gemma7B)
        );
        assert_eq!(
            PresetTokenizers::from_str("google/gemma-2b"),
            Some(PresetTokenizers::Gemma2B)
        );
        assert_eq!(
            PresetTokenizers::from_str("google/gemma-3-4b-it"),
            Some(PresetTokenizers::Gemma3_4BIt)
        );
        assert_eq!(
            PresetTokenizers::from_str("tiiuae/falcon-7b"),
            Some(PresetTokenizers::Falcon7B)
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
            PresetTokenizers::Mixtral8x7Bv01,
            PresetTokenizers::Mistral7Bv01,
            PresetTokenizers::MistralLite,
            PresetTokenizers::Gemma7B,
            PresetTokenizers::Gemma2B,
            PresetTokenizers::Gemma3_4BIt,
            PresetTokenizers::Falcon7B,
        ];

        for preset in presets {
            println!("Testing preset: {:?}", preset);
            // Should produce Ok(Tokenizer)
            let tok = preset
                .retrieve_tokenizer()
                .expect("preset tokenizer must load");

            // Sanity: tokenizer should yield at least one token for a short input
            let enc = tok.encode("test", true).unwrap();
            assert!(!enc.get_ids().is_empty(), "produced empty token sequence");
        }
    }
}
