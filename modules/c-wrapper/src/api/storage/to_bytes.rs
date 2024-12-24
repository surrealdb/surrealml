use crate::state::STATE;
use std::os::raw::c_char;
use crate::utils::VecU8Return;
use crate::process_string_for_vec_u8_return;
use std::ffi::CString;
use std::ffi::CStr;

/// Converts the entire file to bytes.
/// 
/// # Arguments
/// * `file_id` - The unique identifier for the SurMlFile struct.
/// 
/// # Returns
/// A vector of bytes representing the entire file.
#[no_mangle]
pub extern "C" fn to_bytes(file_id_ptr: *const c_char) -> VecU8Return {
    let file_id = process_string_for_vec_u8_return!(file_id_ptr, "file id");
    let mut state = STATE.lock().unwrap();
    let file = state.get_mut(&file_id).unwrap();
    let raw_bytes = file.to_bytes();
    VecU8Return::success(raw_bytes)
}
