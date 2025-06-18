use crate::utils::EmptyReturn;
use std::ffi::CString;
use surrealml_core::execution::session::set_environment;

/// Links the onnx file to the environment.
///
/// # Returns
/// An EmptyReturn object containing the outcome of the operation.
#[no_mangle]
pub extern "C" fn link_onnx() -> EmptyReturn {
    match set_environment() {
        Ok(_) => EmptyReturn {
            is_error: 0,
            error_message: std::ptr::null_mut(),
        },
        Err(e) => {
            println!("Error linking onnx file to environment: {}", e);
            EmptyReturn {
                is_error: 1,
                error_message: CString::new(e.to_string()).unwrap().into_raw(),
            }
        }
    }
}
