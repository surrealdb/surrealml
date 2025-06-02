mod error;
mod tokenizer;
mod preset_tokenizers;
#[cfg(feature = "http-access")]
mod fetch_tokenizer;

pub use tokenizers::Tokenizer;
pub use crate::error::SurrealError;
pub use tokenizer::{load_tokenizer, encode, decode};