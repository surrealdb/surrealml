//! Defines the python API for execution of models.
use pyo3::prelude::*;
use surrealml_core::execution::compute::ModelComputation;
use std::collections::HashMap;

use crate::python_state::PYTHON_STATE;


/// Runs a computation based off raw data and returns the output.
/// 
/// # Arguments
/// * `file_id` - The unique identifier for the loaded model.
/// * `input_vector` - The input vector to the loaded model.
/// 
/// # Returns
/// The computed output vector from the loaded model.
#[pyfunction]
pub fn raw_compute(file_id: String, input_vector: Vec<f32>, dims: Option<(i32, i32)>) -> Vec<f32> {
    let mut python_state = match PYTHON_STATE.lock() {
        Ok(state) => state,
        Err(error) => panic!("{}", format!("Error getting python state: {}", error))
    };
    let mut file = match python_state.get_mut(&file_id) {
        Some(file) => file,
        None => panic!("File not found for id: {}, here is the state: {:?}", file_id, python_state.keys())
    };
    let tensor = ndarray::arr1(&input_vector).into_dyn();
    let compute_unit = ModelComputation {
        surml_file: &mut file
    };
    compute_unit.raw_compute(tensor, dims).unwrap()
}


/// Runs a computation based off key bindings and returns the output and applies normalisers if they are present.
/// 
/// # Arguments
/// * `file_id` - The unique identifier for the loaded model.
/// * `input_values_map` - The input values to the loaded model.
/// 
/// # Returns
/// The computed output vector from the loaded model.
#[pyfunction]
pub fn buffered_compute(file_id: String, mut input_values_map: HashMap<String, f32>) -> Vec<f32> {
    let mut python_state = match PYTHON_STATE.lock() {
        Ok(state) => state,
        Err(error) => panic!("{}", format!("Error getting python state: {}", error))
    };
    let mut file = match python_state.get_mut(&file_id) {
        Some(file) => file,
        None => panic!("File not found for id: {}, here is the state: {:?}", file_id, python_state.keys())
    };

    let compute_unit = ModelComputation {
        surml_file: &mut file
    };
    compute_unit.buffered_compute(&mut input_values_map).unwrap()
}
