//! Defines the session module for the execution module.
use ort::session::Session;
use tempfile::NamedTempFile;
use std::path::PathBuf;
use std::io::Write;
use crate::errors::error::{SurrealError, SurrealErrorStatus};
use crate::safe_eject;

#[cfg(feature = "dynamic")]
use onnx_embedding::embed_onnx;

#[cfg(feature = "dynamic")]
use ort::environment::EnvironmentBuilder;

#[cfg(feature = "dynamic")]
pub static ONNX_BYTES: &'static [u8] = embed_onnx!("1.20.0");


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

/// Unpacks the embedded libonnxruntime and loads it into ort.
#[cfg(feature = "dynamic")]
pub fn set_environment() -> Result<(), SurrealError> {
    // Generate the ort environment from the dynamically pulled onnx runtime bytes
    let mut temp_file = NamedTempFile::new().map_err(|e| {
        SurrealError::new(e.to_string(), SurrealErrorStatus::Unknown)
    })?;
    temp_file.write_all(ONNX_BYTES).map_err(|e| {
        SurrealError::new(e.to_string(), SurrealErrorStatus::Unknown)
    })?;
    let path: PathBuf = temp_file.path().to_path_buf();
    let path_str = match path.to_str() {
        Some(unwrapped_string) => unwrapped_string,
        None => return Err(SurrealError::new("cannot convert ONNX temp file to string".to_string(), SurrealErrorStatus::Unknown))
    };
    let environment: EnvironmentBuilder = ort::init_from(path_str);

    match environment.commit() {
        Ok(_env) => {
            // TODO => might look into wrapping the session in a lock but for now it seems to be
            // working in tests. Below is what the lock can look like:
            //  pub static ORT_ENV: LazyLock<Arc<Mutex<Option<Arc<Environment>>>>> = LazyLock::new(|| Arc::new(Mutex::new(None)));
        },
        Err(e) => {
            return Err(SurrealError::new(e.to_string(), SurrealErrorStatus::Unknown));
        }
    }
    Ok(())
}