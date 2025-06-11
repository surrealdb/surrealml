use crate::utils::{StringReturn, TokenizerReturn, VecU32Return};
use crate::{
    process_opt_string_for_tokenizer_return, process_string_for_tokenizer_return,
    process_string_for_vecu32_return,
};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::slice;
use surrealml_tokenizers::{
    decode as decode_surrealml, encode as encode_surrealml,
    load_tokenizer as load_tokenizer_surrealml, Tokenizer,
};

#[repr(C)]
pub struct TokenizerHandle {
    tokenizer: Tokenizer,
}

#[no_mangle]
pub extern "C" fn load_tokenizer(model: *const c_char, hf_token: *const c_char) -> TokenizerReturn {
    let model = process_string_for_tokenizer_return!(model, "model");
    let hf_token = process_opt_string_for_tokenizer_return!(hf_token, "hf_token");

    match load_tokenizer_surrealml(model, hf_token) {
        Ok(tok) => {
            let handle = Box::new(TokenizerHandle { tokenizer: tok });
            return TokenizerReturn::success(Box::into_raw(handle));
        }
        Err(_) => return TokenizerReturn::error(format!("Invalid UTF-8 for tokenizer")),
    }
}

#[no_mangle]
pub extern "C" fn encode(
    tokenizer_handle: *mut TokenizerHandle,
    text: *const c_char,
) -> VecU32Return {
    if tokenizer_handle.is_null() {
        return VecU32Return::error(format!(
            "Received null pointer for tokenizer handle in encode fn"
        ));
    }
    let text = process_string_for_vecu32_return!(text, "text");
    let tokenizer = unsafe { &(*tokenizer_handle).tokenizer };

    match encode_surrealml(tokenizer, &text) {
        Ok(ids) => VecU32Return::success(ids),
        Err(e) => VecU32Return::error(format!("Failed to encode text '{}': {}", text, e)),
    }
}

#[no_mangle]
pub extern "C" fn decode(
    tokenizer_handle: *mut TokenizerHandle,
    data_ptr: *const u32,
    length: usize,
) -> StringReturn {
    if tokenizer_handle.is_null() {
        return StringReturn {
            string: std::ptr::null_mut(),
            is_error: 1,
            error_message: CString::new("Received null pointer for tokenizer handle")
                .unwrap()
                .into_raw(),
        };
    }
    let tokenizer = unsafe { &(*tokenizer_handle).tokenizer };

    if data_ptr.is_null() {
        return StringReturn {
            string: std::ptr::null_mut(),
            is_error: 1,
            error_message: CString::new("Received null pointer for data")
                .unwrap()
                .into_raw(),
        };
    };
    let slice: &[u32] = unsafe { slice::from_raw_parts(data_ptr, length) };

    match decode_surrealml(tokenizer, slice) {
        Ok(decoded_string) => match CString::new(decoded_string) {
            Ok(c_string) => StringReturn {
                string: c_string.into_raw(),
                is_error: 0,
                error_message: std::ptr::null_mut(),
            },
            Err(_) => {
                return StringReturn {
                    string: std::ptr::null_mut(),
                    is_error: 1,
                    error_message: CString::new("Failed to create CString from decoded string")
                        .unwrap()
                        .into_raw(),
                }
            }
        },
        Err(_) => {
            return StringReturn {
                string: std::ptr::null_mut(),
                is_error: 1,
                error_message: CString::new("Failed to decode data").unwrap().into_raw(),
            }
        }
    }
}
