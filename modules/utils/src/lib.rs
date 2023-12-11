//! An embedded ONNX runtime directly in the Rust binary when compiling result in no need for installing ONNX runtime separately
//! or worrying about version clashes with other runtimes.
//!
//! This crate is just the Rust implementation of the Surml API. It is advised that you just use this crate directly if you are running
//! a Rust server. It must be noted that the version of ONNX needs to be the same as the client when using this crate. For this current
//! version of Surml, the ONNX version is `1.16.0`.
//!
//! ## Usage
//! Surml can be used to store, load, and execute ONNX models.
//!
//! ### Storing and accessing models
//! We can store models and meta data around the models with the following code:
//! ```rust
//! use std::fs::File;
//! use std::io::{self, Read, Write};
//!
//! use surrealml_core::storage::surml_file::SurMlFile;
//! use surrealml_core::storage::header::Header;
//! use surrealml_core::storage::header::normalisers::{
//!     wrapper::NormaliserType,
//!     linear_scaling::LinearScaling
//! };
//!
//!
//! // load your own model here (surrealml python package can be used to convert PyTorch,
//! // and Sklearn models to ONNX or package them as surml files)
//! let mut file = File::open("./stash/linear_test.onnx").unwrap();
//! let mut model_bytes = Vec::new();
//! file.read_to_end(&mut model_bytes).unwrap();
//!
//! // create a header for the model
//! let mut header = Header::fresh();
//! header.add_column(String::from("squarefoot"));
//! header.add_column(String::from("num_floors"));
//! header.add_output(String::from("house_price"), None);
//!
//! // add normalisers if needed
//! header.add_normaliser(
//!     "squarefoot".to_string(),
//!     NormaliserType::LinearScaling(LinearScaling { min: 0.0, max: 1.0 })
//! );
//! header.add_normaliser(
//!     "num_floors".to_string(),
//!     NormaliserType::LinearScaling(LinearScaling { min: 0.0, max: 1.0 })
//! );
//!
//! // create a surml file
//! let surml_file = SurMlFile::new(header, model_bytes);
//!
//! // read and write surml files
//! surml_file.write("./stash/test.surml").unwrap();
//! let new_file = SurMlFile::from_file("./stash/test.surml").unwrap();
//! let file_from_bytes = SurMlFile::from_bytes(surml_file.to_bytes()).unwrap();
//! ```
//!
//! ### Executing models
//! We you load a `surml` file, you can execute the model with the following code:
//! ```rust
//! use surrealml_core::storage::surml_file::SurMlFile;
//! use surrealml_core::execution::compute::ModelComputation;
//! use ndarray::ArrayD;
//! use std::collections::HashMap;
//!
//!
//! let mut file = SurMlFile::from_file("./stash/test.surml").unwrap();
//!
//! let compute_unit = ModelComputation {
//!     surml_file: &mut file,
//! };
//!
//! // automatically map inputs and apply normalisers to the compute if this data was put in the header
//! let mut input_values = HashMap::new();
//! input_values.insert(String::from("squarefoot"), 1000.0);
//! input_values.insert(String::from("num_floors"), 2.0);
//!
//! let output = compute_unit.buffered_compute(&mut input_values).unwrap();
//!
//! // feed a raw ndarray into the model if no header was provided or if you want to bypass the header
//! let x = vec![1000.0, 2.0];
//! let data: ArrayD<f32> = ndarray::arr1(&x).into_dyn();
//!
//! // None input can be a tuple of dimensions of the input data
//! let output = compute_unit.raw_compute(data, None).unwrap();
//! ```
pub mod storage;
pub mod execution;


/// Returns the version of the ONNX runtime that is used.
pub fn onnx_runtime() -> &'static str {
    "1.16.0"
}
