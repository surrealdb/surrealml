//! Defines the operations around performing computations on a loaded model with wonnx.
use crate::storage::surml_file::SurMlFile;
use std::collections::HashMap;
use ndarray::{ArrayD, CowArray};
use std::sync::Arc;
use wonnx::Session;
use wonnx::utils::{InputTensor, OutputTensor, tensor};
use wonnx::SessionConfig;

use std::fs::File;
use std::io::{Read, Result};


// below is the tract imports
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};
use tokenizers::tokenizer::{Result, Tokenizer};
use tract_onnx::prelude::*;


pub async fn load_model() {
    // let file = SurMlFile::from_file("./stash/test.surml").unwrap();
    let mut file = File::open("./stash/linear_test.onnx").unwrap();

    // Create a buffer to hold the file's contents
    let mut buffer = Vec::new();

    // Read the file's contents into the buffer
    file.read_to_end(&mut buffer).unwrap();
    let config = SessionConfig::new().with_outputs(Some(vec!["5".to_string()]));
    let session = Session::from_bytes_with_config(&buffer, &config).await.unwrap();
    let mut inputs = HashMap::new();
    inputs.insert("onnx::MatMul_0".to_string(), InputTensor::F32(vec![1000.0, 2.0].into()));
    let outputs = session.run(&inputs).await.unwrap();
    println!("file: {:?}", outputs);
}


pub fn load_tract_model() {
    
}


#[cfg(test)]
mod tests {

    use tokio::runtime;

    use super::*;

    #[test]
    fn test_raw_compute() {
        let runtime = runtime::Runtime::new().unwrap();
        runtime.block_on(load_model());
        // let mut file = SurMlFile::from_file("./stash/test.surml").unwrap();
        // let model_computation = ModelComputation {
        //     surml_file: &mut file,
        // };

        // let mut input_values = HashMap::new();
        // input_values.insert(String::from("squarefoot"), 1000.0);
        // input_values.insert(String::from("num_floors"), 2.0);

        // let output = model_computation.raw_compute(model_computation.input_tensor_from_key_bindings(input_values)).unwrap();
        // assert_eq!(output.len(), 1);
        // assert_eq!(output[0], 725.42053);
    }

}