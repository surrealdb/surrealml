//! # Surml Utils
//! This crate is just the Rust implementation of the Surml API. It is advised that you just use this crate directly if you are running
//! a Rust server. It must be noted that the version of ONNX needs to be the same as the client when using this crate. For this current
//! version of Surml, the ONNX version is `1.16.0`.
pub mod storage;
pub mod execution;


/// Returns the version of the ONNX runtime that is used.
pub fn onnx_runtime() -> &'static str {
    "1.16.0"
}
