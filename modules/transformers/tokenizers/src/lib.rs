mod error;
#[cfg(feature = "http-access")]
mod fetch_tokenizer;
mod preset_tokenizers;
mod tokenizer;

pub use crate::error::SurrealError;
pub use tokenizer::{decode, encode, load_tokenizer};
pub use tokenizers::Tokenizer;
