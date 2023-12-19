//! This module defines the ONNX environment for the execution of ONNX models.
use ort::{Environment, ExecutionProvider};
use once_cell::sync::Lazy;
use std::sync::Arc;


// Compiles the ONNX module into the rust binary.
#[cfg(all(target_os = "macos", not(doc), not(onnx_runtime_env_var_set)))]
pub static LIB_BYTES: &'static [u8] = include_bytes!("../../onnx_driver/target/debug/libonnxruntime.dylib");

#[cfg(all(any(target_os = "linux", target_os = "android"), not(doc), not(onnx_runtime_env_var_set)))]
pub static LIB_BYTES: &'static [u8] = include_bytes!("../../onnx_driver/target/debug/libonnxruntime.so");

#[cfg(all(target_os = "windows", not(doc), not(onnx_runtime_env_var_set)))]
pub static LIB_BYTES: &'static [u8] = include_bytes!("../../onnx_driver/target/debug/onnxruntime.dll");

// Fallback for documentation and other targets
#[cfg(any(doc, onnx_runtime_env_var_set, not(any(target_os = "macos", target_os = "linux", target_os = "android", target_os = "windows"))))]
pub static LIB_BYTES: &'static [u8] = &[];


// the ONNX environment which loads the library
pub static ENVIRONMENT: Lazy<Arc<Environment>> = Lazy::new(|| {

    // if the "ONNXRUNTIME_LIB_PATH" is provided we do not need to compile the ONNX library, instead we just point to the library
    // in the "ONNXRUNTIME_LIB_PATH" and load that.
    match std::env::var("ONNXRUNTIME_LIB_PATH") {
        Ok(path) => {
            std::env::set_var("ORT_DYLIB_PATH", path);
            return Arc::new(
                Environment::builder()
                    .with_execution_providers([ExecutionProvider::CPU(Default::default())])
                    .build().unwrap()
            );
        },
        // if the "ONNXRUNTIME_LIB_PATH" is not provided we use the `LIB_BYTES` that is the ONNX library compiled into the binary.
        // we write the `LIB_BYTES` to a temporary file and then load that file.
        Err(_) => {

            #[cfg(any(not(doc), not(onnx_runtime_env_var_set)))]
            let _ = std::fs::write("./libonnxruntime.dylib", LIB_BYTES);
            
            std::env::set_var("ORT_DYLIB_PATH", "./libonnxruntime.dylib");
            let environment = Arc::new(
                Environment::builder()
                    .with_execution_providers([ExecutionProvider::CPU(Default::default())])
                    .build().unwrap()
            );
            std::env::remove_var("ORT_DYLIB_PATH");

            #[cfg(any(not(doc), not(onnx_runtime_env_var_set)))]
            let _ = std::fs::remove_file("./libonnxruntime.dylib");
            
            return environment
        }
    }
});



