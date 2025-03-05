//! Defines the session module for the execution module.
use ort::session::Session;
use crate::errors::error::{SurrealError, SurrealErrorStatus};
use crate::safe_eject;

#[cfg(feature = "dynamic")]
use once_cell::sync::Lazy;
#[cfg(feature = "dynamic")]
use ort::environment::{EnvironmentBuilder, Environment};
#[cfg(feature = "dynamic")]
use std::sync::{Arc, Mutex};

use std::sync::LazyLock;


/// Creates a session for a model.
/// 
/// # Arguments
/// * `model_bytes` - The model bytes (usually extracted fromt the surml file)
/// 
/// # Returns
/// A session object.
pub fn get_session(model_bytes: Vec<u8>) -> Result<Session, SurrealError> {
    let builder = safe_eject!(Session::builder(), SurrealErrorStatus::Unknown);

    #[cfg(feature = "gpu")]
    {
        let cuda = CUDAExecutionProvider::default();
        if let Err(e) = cuda.register(&builder) {
            eprintln!("Failed to register CUDA: {:?}. Falling back to CPU.", e);
        } else {
            println!("CUDA registered successfully");
        }
    }
    let session: Session = safe_eject!(builder
        .commit_from_memory(&model_bytes), SurrealErrorStatus::Unknown);
    Ok(session)
}


// #[cfg(feature = "dynamic")]
// pub static ORT_ENV: LazyLock<Arc<Mutex<Option<Arc<Environment>>>>> = LazyLock::new(|| Arc::new(Mutex::new(None)));


#[cfg(feature = "dynamic")]
pub fn set_environment(dylib_path: String) -> Result<(), SurrealError> {

    let outcome: EnvironmentBuilder = ort::init_from(dylib_path);
    match outcome.commit() {
        Ok(env) => {
            // ORT_ENV.lock().unwrap().replace(env);
        },
        Err(e) => {
            return Err(SurrealError::new(e.to_string(), SurrealErrorStatus::Unknown));
        }
    }
    Ok(())
}
