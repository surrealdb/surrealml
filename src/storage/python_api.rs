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
use super::surml_file::SurMlFile;
use super::header::normalisers::wrapper::NormaliserType;
use tch::CModule;

use crate::python_state::{PYTHON_STATE, generate_unique_id};


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


/// Saves a model to a file.
/// 
/// # Arguments
/// * `file_path` - The path to the file to save to.
/// * `file_id` - The unique identifier for the loaded model.
#[pyfunction]
pub fn save_model(file_path: String, file_id: String) {
    let mut python_state = PYTHON_STATE.lock().unwrap();
    let file = python_state.get_mut(&file_id).unwrap();
    file.write(&file_path).unwrap();
}


/// Loads a PyTorch C model from a file wrapping it in a SurMlFile struct 
/// which is stored in memory and referenced by a unique ID.
/// 
/// # Arguments
/// * `file_path` - The path to the file to load.
#[pyfunction]
pub fn load_cached_raw_model(file_path: String) -> String {
    let file_id = generate_unique_id();
    let model = CModule::load(file_path).unwrap();
    let file = SurMlFile::fresh(model);
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
    wrapped_file.header.add_version(version);
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
    file.header.normalisers.add_normaliser(normaliser, column_name, &file.header.keys);
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

