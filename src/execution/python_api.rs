//! Defines the python API for execution of models.
use pyo3::prelude::*;
use crate::execution::compute::ModelComputation;
use tch::Tensor;
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
pub fn raw_compute(file_id: String, input_vector: Vec<f32>) -> Vec<f32> {
    let mut python_state = PYTHON_STATE.lock().unwrap();
    let mut file = python_state.get_mut(&file_id).unwrap();
    let tensor = Tensor::f_from_slice::<f32>(input_vector.as_slice()).unwrap();
    file.model.set_eval();
    let compute_unit = ModelComputation {
        surml_file: &mut file
    };
    let output_tensor = compute_unit.raw_compute(tensor);
    let mut buffer = Vec::with_capacity(output_tensor.size()[0] as usize);

    for i in 0..output_tensor.size()[0] {
        println!("{}", output_tensor.double_value(&[i]));
        buffer.push(output_tensor.double_value(&[i]) as f32);
    }
    buffer
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

    let output_tensor = compute_unit.buffered_compute(&mut input_values_map);
    let mut buffer: Vec<f32> = Vec::with_capacity(output_tensor.size()[0] as usize);
    println!("{:?}", output_tensor);
    for i in 0..output_tensor.size()[0] {
        buffer.push(output_tensor.double_value(&[i]) as f32);
    }
    buffer
}
