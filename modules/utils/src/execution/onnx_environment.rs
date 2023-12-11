//! This module defines the ONNX environment for the execution of ONNX models.
use ort::{Environment, ExecutionProvider};
use once_cell::sync::Lazy;
use std::sync::Arc;


// Compiles the ONNX module into the rust binary.
#[cfg(all(target_os = "macos", not(doc)))]
pub static LIB_BYTES: &'static [u8] = include_bytes!("../../onnx_driver/target/debug/libonnxruntime.dylib");

#[cfg(all(any(target_os = "linux", target_os = "android"), not(doc)))]
pub static LIB_BYTES: &'static [u8] = include_bytes!("../../onnx_driver/target/debug/libonnxruntime.so");

#[cfg(all(target_os = "windows", not(doc)))]
pub static LIB_BYTES: &'static [u8] = include_bytes!("../../onnx_driver/target/debug/libonnxruntime.dll");

// Fallback for documentation and other targets
#[cfg(any(doc, not(any(target_os = "macos", target_os = "linux", target_os = "android", target_os = "windows"))))]
pub static LIB_BYTES: &'static [u8] = &[];


// the ONNX environment which loads the library
pub static ENVIRONMENT: Lazy<Arc<Environment>> = Lazy::new(|| {
    let _ = std::fs::write("./libonnxruntime.dylib", LIB_BYTES);
    std::env::set_var("ORT_DYLIB_PATH", "./libonnxruntime.dylib");
    let environment = Arc::new(
        Environment::builder()
            .with_execution_providers([ExecutionProvider::CPU(Default::default())])
            .build().unwrap()
    );
    let _ = std::fs::remove_file("./libonnxruntime.dylib");
    environment
});



