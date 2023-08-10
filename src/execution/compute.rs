use crate::storage::surml_file::SurMlFile;
use tch::Tensor;
use std::collections::HashMap;


pub struct ModelComputation<'a> {
    pub surml_file: &'a mut SurMlFile,
}


impl <'a>ModelComputation<'a> {

    pub fn input_tensor_from_key_bindings(&self, mut input_values: HashMap<String, f32>) -> Tensor {
        let mut buffer = Vec::with_capacity(self.surml_file.header.keys.store.len());
        for key in &self.surml_file.header.keys.store {
            let value = input_values.get_mut(key).unwrap();
            buffer.push(std::mem::take(value));
        }
        Tensor::f_from_slice::<f32>(buffer.as_slice()).unwrap()
    }

    pub fn raw_compute(&self, tensor: Tensor) -> Tensor {
        self.surml_file.model.forward_ts(&[tensor]).unwrap()
    }

    pub fn buffered_compute(&self, mut input_values: HashMap<String, f32>) -> Tensor {
        let tensor = self.input_tensor_from_key_bindings(input_values);
        self.surml_file.model.forward_ts(&[tensor]).unwrap()
    }

}
