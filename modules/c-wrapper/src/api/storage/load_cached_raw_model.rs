use crate::state::{STATE, generate_unique_id};
use surrealml_core::storage::surml_file::SurMlFile;
use std::fs::File;
use std::io::Read;
use std::os::raw::c_char;
use std::ffi::CString;
use std::ffi::CStr;
use crate::utils::StringReturn;
use crate::process_string_for_string_return;


/// Loads a PyTorch C model from a file wrapping it in a SurMlFile struct 
/// which is stored in memory and referenced by a unique ID.
/// 
/// # Arguments
/// * `file_path` - The path to the file to load.
/// 
/// # Returns
/// A unique identifier for the loaded model.
#[no_mangle]
pub extern "C" fn load_cached_raw_model(file_path_ptr: *const c_char) -> StringReturn {
    let file_path_str = process_string_for_string_return!(file_path_ptr, "file path");
    let file_id = generate_unique_id();
    let mut model = File::open(file_path_str).unwrap();
    let mut data = vec![];
    model.read_to_end(&mut data).unwrap();
    let file = SurMlFile::fresh(data);
    let mut python_state = STATE.lock().unwrap();
    python_state.insert(file_id.clone(), file);
    StringReturn::success(file_id)
}