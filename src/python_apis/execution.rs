//! Defines the python API for execution of models.
use pyo3::prelude::*;
use surrealml_utils::execution::compute::ModelComputation;
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
    let mut python_state = PYTHON_STATE.lock().unwrap();
    let mut file = python_state.get_mut(&file_id).unwrap();
    let mut tensor = ndarray::arr1(&input_vector).into_dyn();
    match dims {
        Some(unwrapped_dims) => {
            let test = (1, 28);
            tensor = tensor.into_shape(test).unwrap().into_dyn();
        },
        None => {}
    }
    let compute_unit = ModelComputation {
        surml_file: &mut file
    };
    compute_unit.raw_compute(tensor).unwrap()
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
    let mut python_state = PYTHON_STATE.lock().unwrap();
    let mut file = python_state.get_mut(&file_id).unwrap();

    let compute_unit = ModelComputation {
        surml_file: &mut file
    };
    compute_unit.buffered_compute(&mut input_values_map).unwrap()

    // let output_tensor = compute_unit.buffered_compute(&mut input_values_map);
    // let mut buffer: Vec<f32> = Vec::with_capacity(output_tensor.size()[0] as usize);
    // for i in 0..output_tensor.size()[0] {
    //     buffer.push(output_tensor.double_value(&[i]) as f32);
    // }
    // buffer
}
