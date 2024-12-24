use crate::state::STATE;
use std::ffi::{c_float, CStr, CString, c_int, c_char};
use surrealml_core::execution::compute::ModelComputation;


#[repr(C)]
pub struct Vecf32Return {
    pub data: *mut f32,
    pub length: usize,
    pub capacity: usize, // Optional if you want to include capacity for clarity
    pub is_error: c_int,
    pub error_message: *mut c_char
}


#[no_mangle]
pub extern "C" fn free_vecf32_return(vecf32_return: Vecf32Return) {
    // Free the data if it is not null
    if !vecf32_return.data.is_null() {
        unsafe { drop(Vec::from_raw_parts(vecf32_return.data, vecf32_return.length, vecf32_return.capacity)) };
    }
    // Free the error message if it is not null
    if !vecf32_return.error_message.is_null() {
        unsafe { drop(CString::from_raw(vecf32_return.error_message)) };
    }
}



#[no_mangle]
pub extern "C" fn raw_compute(file_id_ptr: *const c_char, data_ptr: *const c_float, length: usize) -> Vecf32Return {

    if file_id_ptr.is_null() {
        return Vecf32Return {
            data: std::ptr::null_mut(),
            length: 0,
            capacity: 0,
            is_error: 1,
            error_message: CString::new("File id is null").unwrap().into_raw()
        }
    }
    if data_ptr.is_null() {
        return Vecf32Return {
            data: std::ptr::null_mut(),
            length: 0,
            capacity: 0,
            is_error: 1,
            error_message: CString::new("Data is null").unwrap().into_raw()
        }
    }

    let file_id = match unsafe { CStr::from_ptr(file_id_ptr) }.to_str() {
        Ok(file_id) => file_id.to_owned(),
        Err(error) => return Vecf32Return {
            data: std::ptr::null_mut(),
            length: 0,
            capacity: 0,
            is_error: 1,
            error_message: CString::new(format!("Error getting file id: {}", error)).unwrap().into_raw()
        }
    };

    let mut state = match STATE.lock() {
        Ok(state) => state,
        Err(error) => {
            return Vecf32Return {
                data: std::ptr::null_mut(),
                length: 0,
                capacity: 0,
                is_error: 1,
                error_message: CString::new(format!("Error getting state: {}", error)).unwrap().into_raw()
            }
        }
    };
    let mut file = match state.get_mut(&file_id) {
        Some(file) => file,
        None => {
            {
                return Vecf32Return {
                    data: std::ptr::null_mut(),
                    length: 0,
                    capacity: 0,
                    is_error: 1,
                    error_message: CString::new(format!("File not found for id: {}, here is the state: {:?}", file_id, state.keys())).unwrap().into_raw()
                }
            }
        }
    };

    let slice = unsafe { std::slice::from_raw_parts(data_ptr, length) };
    let tensor = ndarray::arr1(slice).into_dyn();
    let compute_unit = ModelComputation {
        surml_file: &mut file
    };

    // perform the computation
    let mut outcome = match compute_unit.raw_compute(tensor, None) {
        Ok(outcome) => outcome,
        Err(error) => {
            return Vecf32Return {
                data: std::ptr::null_mut(),
                length: 0,
                capacity: 0,
                is_error: 1,
                error_message: CString::new(format!("Error computing model: {}", error.message)).unwrap().into_raw()
            }
        }
    };
    let outcome_ptr = outcome.as_mut_ptr();
    let outcome_len = outcome.len();
    let outcome_capacity = outcome.capacity();
    std::mem::forget(outcome);
    Vecf32Return {
        data: outcome_ptr,
        length: outcome_len,
        capacity: outcome_capacity,
        is_error: 0,
        error_message: std::ptr::null_mut()
    }
}