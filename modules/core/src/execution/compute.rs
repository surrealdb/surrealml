//! Defines the operations around performing computations on a loaded model.
use crate::storage::surml_file::SurMlFile;
use std::collections::HashMap;
use ndarray::{ArrayD, CowArray};
use ort::{SessionBuilder, Value, session::Input};
use super::onnx_environment::ENVIRONMENT;


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
    pub fn input_tensor_from_key_bindings(&self, input_values: HashMap<String, f32>) -> ArrayD<f32> {
        let buffer = self.input_vector_from_key_bindings(input_values);
        ndarray::arr1::<f32>(&buffer).into_dyn()
    }

    /// Creates a vector of dimensions for the input tensor from the loaded model.
    /// 
    /// # Arguments
    /// * `input_dims` - The input dimensions from the loaded model.
    /// 
    /// # Returns
    /// A vector of dimensions for the input tensor to be reshaped into from the loaded model.
    fn process_input_dims(input_dims: &Input) -> Vec<usize> {
        let mut buffer = Vec::new();
        for dim in input_dims.dimensions() {
            match dim {
                Some(dim) => buffer.push(dim as usize),
                None => buffer.push(1)
            }
        }
        buffer
    }

    /// Creates a Vector that can be used manipulated with other operations such as normalisation from a hashmap of keys and values.
    /// 
    /// # Arguments
    /// * `input_values` - A hashmap of keys and values that will be used to create the input vector.
    /// 
    /// # Returns
    /// A Vector that can be used manipulated with other operations such as normalisation.
    pub fn input_vector_from_key_bindings(&self, mut input_values: HashMap<String, f32>) -> Vec<f32> {
        let mut buffer = Vec::with_capacity(self.surml_file.header.keys.store.len());
        for key in &self.surml_file.header.keys.store {
            let value = input_values.get_mut(key).unwrap();
            buffer.push(std::mem::take(value));
        }
        buffer
    }

    /// Performs a raw computation on the loaded model.
    /// 
    /// # Arguments
    /// * `tensor` - The input tensor to the loaded model.
    /// 
    /// # Returns
    /// The computed output tensor from the loaded model.
    pub fn raw_compute(&self, tensor: ArrayD<f32>, _dims: Option<(i32, i32)>) -> Result<Vec<f32>, String> {
        let session = SessionBuilder::new(&ENVIRONMENT).map_err(|e| e.to_string())?
                                                       .with_model_from_memory(&self.surml_file.model)
                                                       .map_err(|e| e.to_string())?;
        let unwrapped_dims = ModelComputation::process_input_dims(&session.inputs[0]);
        let tensor = tensor.into_shape(unwrapped_dims).map_err(|e| e.to_string())?;

        let x = CowArray::from(tensor).into_dyn();
        let outputs = session.run(vec![Value::from_array(session.allocator(), &x).unwrap()]).map_err(|e| e.to_string())?;

        let mut buffer: Vec<f32> = Vec::new();

        // extract the output tensor converting the values to f32 if they are i64
        match outputs[0].try_extract::<f32>() {
            Ok(y) => {
                for i in y.view().clone().into_iter() {
                    buffer.push(*i);
                }
            },
            Err(_) => {
                match outputs[0].try_extract::<i64>() {
                    Ok(y) => {
                        for i in y.view().clone().into_iter() {
                            buffer.push(*i as f32);
                        }
                    },
                    Err(e) => return Err(e.to_string())
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
    pub fn buffered_compute(&self, input_values: &mut HashMap<String, f32>) -> Result<Vec<f32>, String> {
        // applying normalisers if present
        for (key, value) in &mut *input_values {
            let value_ref = value.clone();
            match self.surml_file.header.get_normaliser(&key.to_string()) {
                Some(normaliser) => {
                    *value = normaliser.normalise(value_ref);
                },
                None => {}
            }
        }
        let tensor = self.input_tensor_from_key_bindings(input_values.clone());
        let output = self.raw_compute(tensor, None)?;
        
        // if no normaliser is present, return the output
        if self.surml_file.header.output.normaliser == None {
            return Ok(output)
        }

        // apply the normaliser to the output
        let output_normaliser = self.surml_file.header.output.normaliser.as_ref().unwrap();
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

    #[test]
    fn test_raw_compute_linear_sklearn() {
        let mut file = SurMlFile::from_file("./model_stash/sklearn/surml/linear.surml").unwrap();
        let model_computation = ModelComputation {
            surml_file: &mut file,
        };

        let mut input_values = HashMap::new();
        input_values.insert(String::from("squarefoot"), 1000.0);
        input_values.insert(String::from("num_floors"), 2.0);

        let raw_input = model_computation.input_tensor_from_key_bindings(input_values);

        let output = model_computation.raw_compute(raw_input, Some((1, 2))).unwrap();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], 985.57745);
    }


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

    #[test]
    fn test_raw_compute_linear_torch() {
        let mut file = SurMlFile::from_file("./model_stash/torch/surml/linear.surml").unwrap();
        let model_computation = ModelComputation {
            surml_file: &mut file,
        };

        let mut input_values = HashMap::new();
        input_values.insert(String::from("squarefoot"), 1000.0);
        input_values.insert(String::from("num_floors"), 2.0);

        let raw_input = model_computation.input_tensor_from_key_bindings(input_values);

        let output = model_computation.raw_compute(raw_input, None).unwrap();
        assert_eq!(output.len(), 1);
    }
}
