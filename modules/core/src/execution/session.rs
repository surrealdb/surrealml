use ort::{InMemorySession, Session};
use nanoservices_utils::errors::{NanoServiceError, NanoServiceErrorStatus};
use nanoservices_utils::safe_eject;


pub fn session(model_data: &Vec<u8>) -> Result<InMemorySession, NanoServiceError> {
    let builder = Session::builder().unwrap();
    safe_eject!(builder.commit_from_memory_directly(model_data), NanoServiceErrorStatus::Unknown)
}
