//! Responsible for the saving and loading of the model including meta data around the model.
pub mod header;
pub mod surml_file;
pub mod cache;

#[cfg(feature = "python")]
pub mod python_api;
