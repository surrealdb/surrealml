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


#[pyfunction]
pub fn load_model(file_path: String) -> String {
    let file_id = generate_unique_id();
    let file = SurMlFile::from_file(&file_path).unwrap();
    let mut python_state = PYTHON_STATE.lock().unwrap();
    python_state.insert(file_id.clone(), file);
    file_id
}


#[pyfunction]
pub fn save_model(file_path: String, file_id: String) {
    let mut python_state = PYTHON_STATE.lock().unwrap();
    let wrapped_file = python_state.get_mut(&file_id).unwrap();
    wrapped_file.write(&file_path).unwrap();
    python_state.remove(&file_id);
}


#[pyfunction]
pub fn load_cached_raw_model(file_path: String) -> String {
    let file_id = generate_unique_id();
    let model = CModule::load(file_path).unwrap();
    let file = SurMlFile::fresh(model);
    let mut python_state = PYTHON_STATE.lock().unwrap();
    python_state.insert(file_id.clone(), file);
    file_id
}


#[pyfunction]
pub fn add_column(file_id: String, column_name: String) {
    let mut python_state = PYTHON_STATE.lock().unwrap();
    let wrapped_file = python_state.get_mut(&file_id).unwrap();
    wrapped_file.header.add_column(column_name);
}


#[pyfunction]
pub fn add_output(file_id: String, output_name: String, normaliser_label: Option<String>, one: Option<f32>, two: Option<f32>) {
    let mut python_state = PYTHON_STATE.lock().unwrap();
    let wrapped_file = python_state.get_mut(&file_id).unwrap();
    if let Some(normaliser_label) = normaliser_label {
        let normaliser = NormaliserType::new(normaliser_label, one.unwrap(), two.unwrap());
        wrapped_file.header.add_output(output_name, Some(normaliser));
    }
    else {
        wrapped_file.header.add_output(output_name, None);
    }
}


#[pyfunction]
pub fn add_output_normaliser(file_id: String, output_name: String) {
    let mut python_state = PYTHON_STATE.lock().unwrap();
    let wrapped_file = python_state.get_mut(&file_id).unwrap();
    wrapped_file.header.add_output(output_name, None);
}


#[pyfunction]
pub fn add_normaliser(file_id: String, column_name: String, normaliser_label: String, one: f32, two: f32) {
    let normaliser = NormaliserType::new(normaliser_label, one, two);
    let mut python_state = PYTHON_STATE.lock().unwrap();
    let wrapped_file = python_state.get_mut(&file_id).unwrap();
    wrapped_file.header.normalisers.add_normaliser(normaliser, column_name, &wrapped_file.header.keys);
}

