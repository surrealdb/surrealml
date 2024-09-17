//! Defines the operations around performing computations on a loaded model.
use crate::storage::surml_file::SurMlFile;
use super::session::session;
use std::collections::HashMap;
use ndarray::ArrayD;
use ort::{Input, ValueType};
use nanoservices_utils::safe_eject;
use nanoservices_utils::errors::{NanoServiceError, NanoServiceErrorStatus};


/// A wrapper for the loaded machine learning model so we can perform computations on the loaded model.
/// 
/// # Attributes
/// * `surml_file` - The loaded machine learning model using interior mutability to allow mutable access to the model
pub struct ModelComputation<'a> {
    pub surml_file: &'a mut SurMlFile,
}


impl <'a>ModelComputation<'a> {

    /// Creates a Tensor that can be used as input to the loaded model from a hashmap of keys and values.
    /// 
    /// # Arguments
    /// * `input_values` - A hashmap of keys and values that will be used to create the input tensor.
    /// 
    /// # Returns
    /// A Tensor that can be used as input to the loaded model.
    pub fn input_tensor_from_key_bindings(&self, input_values: HashMap<String, f32>) -> Result<ArrayD<f32>, NanoServiceError> {
        let buffer = self.input_vector_from_key_bindings(input_values)?;
        Ok(ndarray::arr1::<f32>(&buffer).into_dyn())
    }

    /// Creates a vector of dimensions for the input tensor from the loaded model.
    /// 
    /// # Arguments
    /// * `input_dims` - The input dimensions from the loaded model.
    /// 
    /// # Returns
    /// A vector of dimensions for the input tensor to be reshaped into from the loaded model.
    fn process_input_dims(input_dims: &Input) -> Result<Vec<usize>, NanoServiceError> {
        match &input_dims.input_type {
            ValueType::Tensor { dimensions, .. } => {
                let mut buffer = Vec::new();
                for dim in dimensions {
                    buffer.push(*dim as usize);
                }
                Ok(buffer)
            },
            _ => Err(NanoServiceError::new(
                String::from("compute => process_input_dims: Unknown input type for input dims"), 
                NanoServiceErrorStatus::Unknown
            ))
        }
    }

    /// Creates a Vector that can be used manipulated with other operations such as normalisation from a hashmap of keys and values.
    /// 
    /// # Arguments
    /// * `input_values` - A hashmap of keys and values that will be used to create the input vector.
    /// 
    /// # Returns
    /// A Vector that can be used manipulated with other operations such as normalisation.
    pub fn input_vector_from_key_bindings(&self, mut input_values: HashMap<String, f32>) -> Result<Vec<f32>, NanoServiceError> {
        let mut buffer = Vec::with_capacity(self.surml_file.header.keys.store.len());

        for key in &self.surml_file.header.keys.store {
            let value = match input_values.get_mut(key) {
                Some(value) => value,
                None => return Err(NanoServiceError::new(format!("src/execution/compute.rs 67: Key {} not found in input values", key), NanoServiceErrorStatus::NotFound))
            };
            buffer.push(std::mem::take(value));
        }

        Ok(buffer)
    }

    /// Performs a raw computation on the loaded model.
    /// 
    /// # Arguments
    /// * `tensor` - The input tensor to the loaded model.
    /// 
    /// # Returns
    /// The computed output tensor from the loaded model.
    pub fn raw_compute(&self, tensor: ArrayD<f32>, _dims: Option<(i32, i32)>) -> Result<Vec<f32>, NanoServiceError> {
        let session = session(&self.surml_file.model)?;
        let unwrapped_dims = ModelComputation::process_input_dims(&session.inputs[0])?;
        let tensor = safe_eject!(
            tensor.into_shape(unwrapped_dims.clone()), 
            NanoServiceErrorStatus::Unknown, 
            "problem with reshaping tensor for raw_compute"
        )?;

        // let x = CowArray::from(tensor).into_dyn();
        let mut buffer = Vec::with_capacity(tensor.len());
        for i in tensor.iter() {
            buffer.push(*i);
        }

        let buffer = ort::Tensor::from_array((
            unwrapped_dims, 
            buffer.into_boxed_slice()
        )).unwrap();

        let input_values = safe_eject!(
            ort::inputs![buffer], 
            NanoServiceErrorStatus::Unknown,
            "problem with creating input values in raw_compute"
        )?;
        let outputs = safe_eject!(
            session.run(input_values), 
            NanoServiceErrorStatus::Unknown,
            "problem with running session in raw_compute"
        )?;

        let mut buffer: Vec<f32> = Vec::with_capacity(outputs.len());

        // extract the output tensor converting the values to f32 if they are i64
        match outputs[0].try_extract_tensor::<f32>() {
            Ok(y) => {
                for i in y.view().clone().into_iter() {
                    buffer.push(*i);
                }
            },
            Err(_) => {
                for i in safe_eject!(
                    outputs[0].try_extract_tensor::<i64>(), 
                    NanoServiceErrorStatus::Unknown,
                    "problem with extracting output tensor in raw_compute"
                )?.view().into_iter() {
                    buffer.push(*i as f32);
                }
            }
        };
        return Ok(buffer)
    }

    /// Checks the header applying normalisers if present and then performs a raw computation on the loaded model. Will
    /// also apply inverse normalisers if present on the outputs.
    /// 
    /// # Notes
    /// This function is fairly coupled and will consider breaking out the functions later on if needed.
    /// 
    /// # Arguments
    /// * `input_values` - A hashmap of keys and values that will be used to create the input tensor.
    /// 
    /// # Returns
    /// The computed output tensor from the loaded model.
    pub fn buffered_compute(&self, input_values: &mut HashMap<String, f32>) -> Result<Vec<f32>, NanoServiceError> {
        // applying normalisers if present
        for (key, value) in &mut *input_values {
            let value_ref = value.clone();
            match self.surml_file.header.get_normaliser(&key.to_string())? {
                Some(normaliser) => {
                    *value = normaliser.normalise(value_ref);
                },
                None => {}
            }
        }
        let tensor = self.input_tensor_from_key_bindings(input_values.clone())?;
        let output = self.raw_compute(tensor, None)?;
        
        // if no normaliser is present, return the output
        if self.surml_file.header.output.normaliser == None {
            return Ok(output)
        }

        // apply the normaliser to the output
        let output_normaliser = match self.surml_file.header.output.normaliser.as_ref() {
            Some(normaliser) => normaliser,
            None => return Err(NanoServiceError::new(
                String::from("No normaliser present for output which shouldn't happen as passed initial check for").to_string(), 
                NanoServiceErrorStatus::Unknown
            ))
        };
        let mut buffer = Vec::with_capacity(output.len());

        for value in output {
            buffer.push(output_normaliser.inverse_normalise(value));
        }
        return Ok(buffer)
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[cfg(feature = "sklearn-tests")]
    #[test]
    fn test_raw_compute_linear_sklearn() {
        let mut file = SurMlFile::from_file("./model_stash/sklearn/surml/linear.surml").unwrap();
        let model_computation = ModelComputation {
            surml_file: &mut file,
        };

        let mut input_values = HashMap::new();
        input_values.insert(String::from("squarefoot"), 1000.0);
        input_values.insert(String::from("num_floors"), 2.0);

        let raw_input = model_computation.input_tensor_from_key_bindings(input_values).unwrap();

        let output = model_computation.raw_compute(raw_input, Some((1, 2))).unwrap();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], 985.57745);
    }

    #[cfg(feature = "sklearn-tests")]
    #[test]
    fn test_raw_compute_linear_sklearn_multiple() {
        let mut file = SurMlFile::from_file("./model_stash/sklearn/surml/multiple_linear.surml").unwrap();
        let model_computation = ModelComputation {
            surml_file: &mut file,
        };

        let mut input_values = HashMap::new();
        input_values.insert(String::from("squarefoot"), 1000.0);
        input_values.insert(String::from("num_floors"), 2.0);

        let inputs = vec![1000.0 as f32, 2.0, 3.0];
        let inputs = ndarray::arr1::<f32>(&inputs).into_dyn();

        let output = model_computation.raw_compute(inputs, Some((1, 2))).unwrap();
        assert_eq!(output.len(), 3);
    }

    #[cfg(feature = "sklearn-tests")]
    #[test]
    fn test_buffered_compute_linear_sklearn() {
        let mut file = SurMlFile::from_file("./model_stash/sklearn/surml/linear.surml").unwrap();
        let model_computation = ModelComputation {
            surml_file: &mut file,
        };

        let mut input_values = HashMap::new();
        input_values.insert(String::from("squarefoot"), 1000.0);
        input_values.insert(String::from("num_floors"), 2.0);

        let output = model_computation.buffered_compute(&mut input_values).unwrap();
        assert_eq!(output.len(), 1);
    }

    #[cfg(feature = "onnx-tests")]
    #[test]
    fn test_raw_compute_linear_onnx() {
        let mut file = SurMlFile::from_file("./model_stash/onnx/surml/linear.surml").unwrap();
        let model_computation = ModelComputation {
            surml_file: &mut file,
        };

        let mut input_values = HashMap::new();
        input_values.insert(String::from("squarefoot"), 1000.0);
        input_values.insert(String::from("num_floors"), 2.0);

        let raw_input = model_computation.input_tensor_from_key_bindings(input_values).unwrap();

        let output = model_computation.raw_compute(raw_input, Some((1, 2))).unwrap();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], 985.57745);
    }

    #[cfg(feature = "onnx-tests")]
    #[test]
    fn test_buffered_compute_linear_onnx() {
        let mut file = SurMlFile::from_file("./model_stash/onnx/surml/linear.surml").unwrap();
        let model_computation = ModelComputation {
            surml_file: &mut file,
        };

        let mut input_values = HashMap::new();
        input_values.insert(String::from("squarefoot"), 1000.0);
        input_values.insert(String::from("num_floors"), 2.0);

        let output = model_computation.buffered_compute(&mut input_values).unwrap();
        assert_eq!(output.len(), 1);
    }

    #[cfg(feature = "torch-tests")]
    #[test]
    fn test_raw_compute_linear_torch() {
        let mut file = SurMlFile::from_file("./model_stash/torch/surml/linear.surml").unwrap();
        let model_computation = ModelComputation {
            surml_file: &mut file,
        };

        let mut input_values = HashMap::new();
        input_values.insert(String::from("squarefoot"), 1000.0);
        input_values.insert(String::from("num_floors"), 2.0);

        let raw_input = model_computation.input_tensor_from_key_bindings(input_values).unwrap();

        let output = model_computation.raw_compute(raw_input, None).unwrap();
        assert_eq!(output.len(), 1);
    }

    #[cfg(feature = "torch-tests")]
    #[test]
    fn test_buffered_compute_linear_torch() {
        let mut file = SurMlFile::from_file("./model_stash/torch/surml/linear.surml").unwrap();
        let model_computation = ModelComputation {
            surml_file: &mut file,
        };

        let mut input_values = HashMap::new();
        input_values.insert(String::from("squarefoot"), 1000.0);
        input_values.insert(String::from("num_floors"), 2.0);

        let output = model_computation.buffered_compute(&mut input_values).unwrap();
        assert_eq!(output.len(), 1);
    }

    #[cfg(feature = "tensorflow-tests")]
    #[test]
    fn test_raw_compute_linear_tensorflow() {
        let mut file = SurMlFile::from_file("./model_stash/tensorflow/surml/linear.surml").unwrap();
        let model_computation = ModelComputation {
            surml_file: &mut file,
        };

        let mut input_values = HashMap::new();
        input_values.insert(String::from("squarefoot"), 1000.0);
        input_values.insert(String::from("num_floors"), 2.0);

        let raw_input = model_computation.input_tensor_from_key_bindings(input_values).unwrap();

        let output = model_computation.raw_compute(raw_input, None).unwrap();
        assert_eq!(output.len(), 1);
    }

    #[cfg(feature = "tensorflow-tests")]
    #[test]
    fn test_buffered_compute_linear_tensorflow() {
        let mut file = SurMlFile::from_file("./model_stash/tensorflow/surml/linear.surml").unwrap();
        let model_computation = ModelComputation {
            surml_file: &mut file,
        };

        let mut input_values = HashMap::new();
        input_values.insert(String::from("squarefoot"), 1000.0);
        input_values.insert(String::from("num_floors"), 2.0);

        let output = model_computation.buffered_compute(&mut input_values).unwrap();
        assert_eq!(output.len(), 1);
    }
}
