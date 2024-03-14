//! Defines the entry point for the storage of models for python.
//! 
//! # Path to saving file
//! we can save a model from python using the following steps:
//! * train the model
//! * add data around the model in terms of key bindings, normalisers, and outputs
//! * trace the model using torch.jit.trace
//! * save the C model using torch.jit.save
//! * load the model using the load_cached_raw_model function
use pyo3::prelude::*;
use surrealml_core::storage::surml_file::SurMlFile;
use surrealml_core::storage::header::normalisers::wrapper::NormaliserType;
use std::fs::File;
use std::io::Read;
use hyper::{Body, Request, Method};
use hyper::header::CONTENT_TYPE;
use hyper::{Client, Uri};
use hyper::header::AUTHORIZATION;
use hyper::header::HeaderValue;
use base64::encode;

use crate::python_state::{PYTHON_STATE, generate_unique_id};
use surrealml_core::storage::stream_adapter::StreamAdapter;


/// Loads a model from a file and returns a unique identifier for the loaded model.
/// 
/// # Arguments
/// * `file_path` - The path to the file to load.
/// 
/// # Returns
/// A unique identifier for the loaded model.
#[pyfunction]
pub fn load_model(file_path: String) -> String {
    let file_id = generate_unique_id();
    let file = SurMlFile::from_file(&file_path).unwrap();
    let mut python_state = PYTHON_STATE.lock().unwrap();
    python_state.insert(file_id.clone(), file);
    file_id
}


/// Saves a model to a file, deleting the file from the `PYTHON_STATE` in the process.
/// 
/// # Arguments
/// * `file_path` - The path to the file to save to.
/// * `file_id` - The unique identifier for the loaded model.
#[pyfunction]
pub fn save_model(file_path: String, file_id: String) {
    let mut python_state = PYTHON_STATE.lock().unwrap();
    let file = python_state.get_mut(&file_id).unwrap();
    file.write(&file_path).unwrap();
    python_state.remove(&file_id);
}


/// Loads a PyTorch C model from a file wrapping it in a SurMlFile struct 
/// which is stored in memory and referenced by a unique ID.
/// 
/// # Arguments
/// * `file_path` - The path to the file to load.
#[pyfunction]
pub fn load_cached_raw_model(file_path: String) -> String {
    let file_id = generate_unique_id();
    let mut model = File::open(file_path).unwrap();
    let mut data = vec![];
    model.read_to_end(&mut data).unwrap();
    let file = SurMlFile::fresh(data);
    let mut python_state = PYTHON_STATE.lock().unwrap();
    python_state.insert(file_id.clone(), file);
    file_id
}


/// Converts the entire file to bytes.
/// 
/// # Arguments
/// * `file_id` - The unique identifier for the SurMlFile struct.
/// 
/// # Returns
/// A vector of bytes representing the entire file.
#[pyfunction]
pub fn to_bytes(file_id: String) -> Vec<u8> {
    let mut python_state = PYTHON_STATE.lock().unwrap();
    let file = python_state.get_mut(&file_id).unwrap();
    file.to_bytes()
}


/// Adds a name to the SurMlFile struct.
/// 
/// # Arguments
/// * `file_id` - The unique identifier for the SurMlFile struct.
/// * `model_name` - The name of the model to be added.
#[pyfunction]
pub fn add_name(file_id: String, model_name: String) {
    let mut python_state = PYTHON_STATE.lock().unwrap();
    let wrapped_file = python_state.get_mut(&file_id).unwrap();
    wrapped_file.header.add_name(model_name);
}


/// Adds a description to the SurMlFile struct.
/// 
/// # Arguments
/// * `file_id` - The unique identifier for the SurMlFile struct.
/// * `description` - The description of the model to be added.
#[pyfunction]
pub fn add_description(file_id: String, description: String) {
    let mut python_state = PYTHON_STATE.lock().unwrap();
    let wrapped_file = python_state.get_mut(&file_id).unwrap();
    wrapped_file.header.add_description(description);
}


/// Adds a version to the SurMlFile struct.
/// 
/// # Arguments
/// * `file_id` - The unique identifier for the SurMlFile struct.
/// * `version` - The version of the model to be added.
#[pyfunction]
pub fn add_version(file_id: String, version: String) {
    let mut python_state = PYTHON_STATE.lock().unwrap();
    let wrapped_file = python_state.get_mut(&file_id).unwrap();
    let _ = wrapped_file.header.add_version(version);
}


/// Adds a column to the SurMlFile struct.
/// 
/// # Arguments
/// * `file_id` - The unique identifier for the SurMlFile struct.
/// * `column_name` - The name of the column to be added.
#[pyfunction]
pub fn add_column(file_id: String, column_name: String) {
    let mut python_state = PYTHON_STATE.lock().unwrap();
    let wrapped_file = python_state.get_mut(&file_id).unwrap();
    wrapped_file.header.add_column(column_name);
}


/// adds an author to the SurMlFile struct.
/// 
/// # Arguments
/// * `file_id` - The unique identifier for the SurMlFile struct.
/// * `author` - The author to be added.
#[pyfunction]
pub fn add_author(file_id: String, author: String) {
    let mut python_state = PYTHON_STATE.lock().unwrap();
    let wrapped_file = python_state.get_mut(&file_id).unwrap();
    wrapped_file.header.add_author(author);
}


/// Adds an origin of where the model was trained to the SurMlFile struct.
/// 
/// # Arguments
/// * `file_id` - The unique identifier for the SurMlFile struct.
/// * `origin` - The origin to be added.
#[pyfunction]
pub fn add_origin(file_id: String, origin: String) {
    let mut python_state = PYTHON_STATE.lock().unwrap();
    let wrapped_file = python_state.get_mut(&file_id).unwrap();
    let _ = wrapped_file.header.add_origin(origin);
}


/// Adds an engine to the SurMlFile struct.
/// 
/// # Arguments
/// * `file_id` - The unique identifier for the SurMlFile struct.
/// * `engine` - The engine to be added.
#[pyfunction]
pub fn add_engine(file_id: String, engine: String) {
    let mut python_state = PYTHON_STATE.lock().unwrap();
    let wrapped_file = python_state.get_mut(&file_id).unwrap();
    wrapped_file.header.add_engine(engine);
}


/// Adds an output to the SurMlFile struct.
/// 
/// # Arguments
/// * `file_id` - The unique identifier for the SurMlFile struct.
/// * `output_name` - The name of the output to be added.
/// * `normaliser_label` - The label of the normaliser to be applied to the output.
/// * `one` - The first parameter of the normaliser.
/// * `two` - The second parameter of the normaliser.
#[pyfunction]
pub fn add_output(file_id: String, output_name: String, normaliser_label: Option<String>, one: Option<f32>, two: Option<f32>) {
    let mut python_state = PYTHON_STATE.lock().unwrap();
    let file = python_state.get_mut(&file_id).unwrap();
    if let Some(normaliser_label) = normaliser_label {
        let normaliser = NormaliserType::new(normaliser_label, one.unwrap(), two.unwrap());
        file.header.add_output(output_name, Some(normaliser));
    }
    else {
        file.header.add_output(output_name, None);
    }
}


/// Adds a normaliser to the SurMlFile struct.
/// 
/// # Arguments
/// * `file_id` - The unique identifier for the SurMlFile struct.
/// * `column_name` - The name of the column to which the normaliser will be applied.
/// * `normaliser_label` - The label of the normaliser to be applied to the column.
/// * `one` - The first parameter of the normaliser.
/// * `two` - The second parameter of the normaliser.
#[pyfunction]
pub fn add_normaliser(file_id: String, column_name: String, normaliser_label: String, one: f32, two: f32) {
    let normaliser = NormaliserType::new(normaliser_label, one, two);
    let mut python_state = PYTHON_STATE.lock().unwrap();
    let file = python_state.get_mut(&file_id).unwrap();
    let _ = file.header.normalisers.add_normaliser(normaliser, column_name, &file.header.keys);
}


/// Deletes a SurMlFile struct from memory.
/// 
/// # Arguments
/// * `file_id` - The unique identifier for the SurMlFile struct.
#[pyfunction]
pub fn delete_cached_model(file_id: String) {
    let mut python_state = PYTHON_STATE.lock().unwrap();
    python_state.remove(&file_id);
}


/// Uploads a file to a url.
/// 
/// # Arguments
/// * `file_path` - The path to the file to upload.
/// * `url` - The url to upload the file to.
/// * `chunk_size` - The size of the chunks to upload the file in.
/// * `ns` - The database namespace to upload the file to.
/// * `db` - The database to upload the file to.
/// * `username` - The username to use for authentication.
/// * `password` - The password to use for authentication.
#[pyfunction]
pub fn upload_model(
    file_path: String,
    url: String,
    chunk_size: usize,
    ns: String,
    db: String,
    username: Option<String>,
    password: Option<String>
) -> Result<(), std::io::Error> {
    let client = Client::new();
    let uri: Uri = url.parse().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let generator = StreamAdapter::new(chunk_size, file_path).unwrap();
    let body = Body::wrap_stream(generator);

    let part_req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header(CONTENT_TYPE, "application/octet-stream")
        .header("ns", HeaderValue::from_str(&ns).unwrap())
        .header("db", HeaderValue::from_str(&db).unwrap());

    let req;
    if username.is_none() == false && password.is_none() == false {
        let encoded_credentials = encode(format!("{}:{}", username.unwrap(), password.unwrap()));
        req = part_req.header(AUTHORIZATION, format!("Basic {}", encoded_credentials))
                      .body(body)
                      .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    }
    else {
        req = part_req.body(body).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    }

    let tokio_runtime = tokio::runtime::Builder::new_current_thread().enable_io().enable_time().build().unwrap();
    tokio_runtime.block_on( async move {
        let _response = client.request(req).await.unwrap();
    });
    Ok(())
}
