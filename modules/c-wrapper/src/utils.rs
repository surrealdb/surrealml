use std::os::raw::{c_char, c_int};
use std::ffi::CString;


#[macro_export]
macro_rules! process_string_for_empty_return {
    ($str_ptr:expr, $var_name:expr) => {
        match $str_ptr.is_null() {
            true => {
                return EmptyReturn {
                    is_error: 1,
                    error_message: CString::new(format!("Received a null pointer for {}", $var_name)).unwrap().into_raw()
                };
            },
            false => {
                let c_str = unsafe { CStr::from_ptr($str_ptr) };
                match c_str.to_str() {
                    Ok(s) => s.to_owned(),
                    Err(_) => {
                        return EmptyReturn {
                            is_error: 1,
                            error_message: CString::new(format!("Invalid UTF-8 string received for {}", $var_name)).unwrap().into_raw()
                        };
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! process_string_for_string_return {
    ($str_ptr:expr, $var_name:expr) => {
        match $str_ptr.is_null() {
            true => {
                return StringReturn {
                    is_error: 1,
                    error_message: CString::new(format!("Received a null pointer for {}", $var_name)).unwrap().into_raw(),
                    string: std::ptr::null_mut()
                };
            },
            false => {
                let c_str = unsafe { CStr::from_ptr($str_ptr) };
                match c_str.to_str() {
                    Ok(s) => s.to_owned(),
                    Err(_) => {
                        return StringReturn {
                            is_error: 1,
                            error_message: CString::new(format!("Invalid UTF-8 string received for {}", $var_name)).unwrap().into_raw(),
                            string: std::ptr::null_mut()
                        };
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! process_string_for_vec_u8_return {
    ($str_ptr:expr, $var_name:expr) => {
        match $str_ptr.is_null() {
            true => {
                return VecU8Return {
                    data: std::ptr::null_mut(),
                    length: 0,
                    capacity: 0,
                    is_error: 1,
                    error_message: CString::new(format!("Received a null pointer for {}", $var_name)).unwrap().into_raw()
                };
            },
            false => {
                let c_str = unsafe { CStr::from_ptr($str_ptr) };
                match c_str.to_str() {
                    Ok(s) => s.to_owned(),
                    Err(_) => {
                        return VecU8Return {
                            data: std::ptr::null_mut(),
                            length: 0,
                            capacity: 0,
                            is_error: 1,
                            error_message: CString::new(format!("Invalid UTF-8 string received for {}", $var_name)).unwrap().into_raw()
                        };
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! string_return_safe_eject {
    ($execution:expr) => {
        match $execution {
            Ok(s) => s,
            Err(e) => {
                return StringReturn {
                    string: std::ptr::null_mut(),
                    is_error: 1,
                    error_message: CString::new(e.to_string()).unwrap().into_raw()
                }
            }
        }
    };
}


#[macro_export]
macro_rules! empty_return_safe_eject {
    ($execution:expr, $var:expr, Option) => {
        match $execution {
            Some(s) => s,
            None => {
                return EmptyReturn {
                    is_error: 1,
                    error_message: CString::new($var).unwrap().into_raw()
                }
            }
        }
    };
    ($execution:expr) => {
        match $execution {
            Ok(s) => s,
            Err(e) => {
                return EmptyReturn {
                    is_error: 1,
                    error_message: CString::new(e.to_string()).unwrap().into_raw()
                }
            }
        }
    };
}


#[repr(C)]
pub struct StringReturn {
    pub string: *mut c_char,
    pub is_error: c_int,
    pub error_message: *mut c_char
}


impl StringReturn {
    pub fn success(string: String) -> Self {
        StringReturn {
            string: CString::new(string).unwrap().into_raw(),
            is_error: 0,
            error_message: std::ptr::null_mut()
        }
    }
}


#[no_mangle]
pub extern "C" fn free_string_return(string_return: StringReturn) {
    // Free the string if it is not null
    if !string_return.string.is_null() {
        unsafe { drop(CString::from_raw(string_return.string)) };
    }
    // Free the error message if it is not null
    if !string_return.error_message.is_null() {
        unsafe { drop(CString::from_raw(string_return.error_message)) };
    }
}


#[repr(C)]
pub struct EmptyReturn {
    pub is_error: c_int,           // 0 for success, 1 for error
    pub error_message: *mut c_char, // Optional error message
}

impl EmptyReturn {
    pub fn success() -> Self {
        EmptyReturn {
            is_error: 0,
            error_message: std::ptr::null_mut()
        }
    }
}


#[no_mangle]
pub extern "C" fn free_empty_return(empty_return: EmptyReturn) {
    // Free the error message if it is not null
    if !empty_return.error_message.is_null() {
        unsafe { drop(CString::from_raw(empty_return.error_message)) };
    }
}


#[repr(C)]
pub struct VecU8Return {
    pub data: *mut u8,
    pub length: usize,
    pub capacity: usize, // Optional if you want to include capacity for clarity
    pub is_error: c_int,
    pub error_message: *mut c_char
}


impl VecU8Return {
    pub fn success(data: Vec<u8>) -> Self {
        let mut data = data;
        let data_ptr = data.as_mut_ptr();
        let length = data.len();
        let capacity = data.capacity();
        std::mem::forget(data);
        VecU8Return {
            data: data_ptr,
            length,
            capacity,
            is_error: 0,
            error_message: std::ptr::null_mut()
        }
    }
}


#[no_mangle]
pub extern "C" fn free_vec_u8(vec_u8: VecU8Return) {
    // Free the data if it is not null
    if !vec_u8.data.is_null() {
        unsafe { drop(Vec::from_raw_parts(vec_u8.data, vec_u8.length, vec_u8.capacity)) };
    }
}