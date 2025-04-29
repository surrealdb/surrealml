//! Defines the session module for the execution module.
use ort::session::Session;
use std::path::PathBuf;
use crate::errors::error::{SurrealError, SurrealErrorStatus};
use crate::safe_eject;
use onnx_embedding::embed_onnx;

#[cfg(feature = "dynamic")]
use ort::environment::EnvironmentBuilder;
use tempfile::TempDir;

use tempfile::tempdir;
use std::io::Cursor;
use zip::ZipArchive;


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


fn unzip_to_temp_dir(zip_bytes: &[u8]) -> std::io::Result<(PathBuf, TempDir)> {
    // 1. Create a temp dir
    let temp_dir = tempdir()?;
    let temp_path = temp_dir.path().to_path_buf(); // clone path before move

    // 2. Open a ZipArchive from the embedded bytes
    let reader = Cursor::new(zip_bytes);
    let mut archive = ZipArchive::new(reader)?;

    // 3. Extract files
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = temp_path.join(file.mangled_name());

        if (*file.name()).ends_with('/') {
            // It's a directory
            std::fs::create_dir_all(&outpath)?;
        } else {
            // It's a file
            if let Some(parent) = outpath.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut outfile = std::fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    // 4. IMPORTANT: TempDir will be deleted when dropped
    // You need to keep the TempDir alive if you want to use the path
    // -> So you must return both the TempDir and the PathBuf!
    Ok((temp_path, temp_dir))
}


#[cfg(feature = "dynamic")]
pub fn set_environment() -> Result<(), SurrealError> {

    #[cfg(doc)]
    const ONNX_BYTES: &[u8] = &[];

    #[cfg(not(doc))]
    const ONNX_BYTES: &[u8] = embed_onnx!("1.20.0");

    let (extracted_lib_dir, _temp_dir) = match unzip_to_temp_dir(onnx_bytes) {
        Ok(package) => package,
        Err(e) => return Err(SurrealError::new(e.to_string(), SurrealErrorStatus::Unknown))
    };

    let onnx_lib_path = if cfg!(target_os = "windows") {
        extracted_lib_dir.join("onnxruntime.dll")
    } else if cfg!(target_os = "macos") {
        extracted_lib_dir.join("libonnxruntime.dylib")
    } else {
        extracted_lib_dir.join("libonnxruntime.so")
    };

    let outcome: EnvironmentBuilder = ort::init_from(onnx_lib_path.to_str().unwrap());
    match outcome.commit() {
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