
#[macro_export]
macro_rules! safe_eject_option {
    ($check:expr) => {
        match $check {Some(x) => x, None => {let file_track = format!("{}:{}", file!(), line!());let message = format!("{}=>The value is not found", file_track);return Err(NanoServiceError::new(message, NanoServiceErrorStatus::NotFound))}}
    };
}


#[macro_export]
macro_rules! safe_eject_internal {
    // Match when the optional string is provided
    ($e:expr, $err_status:expr, $msg:expr) => {
        $e.map_err(|x| {let file_track = format!("{}:{}", file!(), line!()); let formatted_error = format!("{} => {}", file_track, x.to_string()); NanoServiceError::new(formatted_error, NanoServiceErrorStatus::Unknown)})?
    };
    // Match when the optional string is not provided
    ($e:expr) => {
        $e.map_err(|x| {let file_track = format!("{}:{}", file!(), line!()); let formatted_error = format!("{} => {}", file_track, x.to_string()); NanoServiceError::new(formatted_error, NanoServiceErrorStatus::Unknown)})?
    };
}