use surrealml_tokenizers::{encode, load_local_tokenizer};
use wasmtime::AsContextMut;
use wasmtime::{Caller, Linker, Memory};
use surrealml_llms::interface::load_model::load_model;