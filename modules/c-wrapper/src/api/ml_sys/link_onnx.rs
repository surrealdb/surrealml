use surrealml_core::execution::session::set_environment;
use std::ffi::{c_float, CStr, CString, c_int, c_char};
use crate::utils::EmptyReturn;


/// Links the onnx file to the environment.
/// 
/// # Arguments
/// * `onnx_path` - The path to the onnx file.
/// 
/// # Returns
/// An EmptyReturn object containing the outcome of the operation.
#[no_mangle]
pub extern "C" fn link_onnx(onnx_path: *const c_char) -> EmptyReturn {
    if onnx_path.is_null() {
        return EmptyReturn {
            is_error: 1,
            error_message: CString::new("Onnx path is null").unwrap().into_raw()
        }
    }
    let onnx_path = match unsafe { CStr::from_ptr(onnx_path) }.to_str() {
        Ok(onnx_path) => onnx_path.to_owned(),
        Err(error) => return EmptyReturn {
            is_error: 1,
            error_message: CString::new(format!("Error getting onnx path: {}", error)).unwrap().into_raw()
        }
    };
    match set_environment(onnx_path) {
        Ok(_) => {
            EmptyReturn {
                is_error: 0,
                error_message: std::ptr::null_mut()
            }
        },
        Err(e) => {
            println!("Error linking onnx file to environment: {}", e);
            EmptyReturn {
                is_error: 1,
                error_message: CString::new(e.to_string()).unwrap().into_raw()
            }
        }
    }
}
