//! Defines the operations around performing computations on a loaded model.
use crate::storage::surml_file::SurMlFile;
use tch::Tensor;
use std::collections::HashMap;


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
    pub fn input_tensor_from_key_bindings(&self, input_values: HashMap<String, f32>) -> Tensor {
        let buffer = self.input_vector_from_key_bindings(input_values);
        Tensor::f_from_slice::<f32>(buffer.as_slice()).unwrap()
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
    pub fn raw_compute(&self, tensor: Tensor) -> Tensor {
        self.surml_file.model.forward_ts(&[tensor]).unwrap()
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
    pub fn buffered_compute(&self, input_values: &mut HashMap<String, f32>) -> Tensor {
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
        
        // perform the calculation
        let output  = self.surml_file.model.forward_ts(&[tensor]).unwrap();
        // if no normaliser is present, return the output
        if self.surml_file.header.output.normaliser == None {
            return output
        }

        // apply the normaliser to the output
        let output_normaliser = self.surml_file.header.output.normaliser.as_ref().unwrap();
        let mut buffer = Vec::with_capacity(output.size()[0] as usize);

        for i in 0..output.size()[0] {
            let value = output.double_value(&[i]) as f32;
            buffer.push(output_normaliser.inverse_normalise(value));
        }
        Tensor::f_from_slice::<f32>(buffer.as_slice()).unwrap()
    }

}
