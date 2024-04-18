//! Defines the placeholder for the type of model engine in the header.
use serde::{Serialize, Deserialize};


/// Defines the type of engine being used to run the model.
/// 
/// # Fields
/// * `Native` - The native engine which will be native rust and linfa.
/// * `PyTorch` - The PyTorch engine which will be PyTorch and tch-rs.
/// * `Undefined` - The undefined engine which will be used when the engine is not defined.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Engine {
    Native,
    PyTorch,
    Undefined
}


impl Engine {

    /// Creates a new `Engine` struct with the undefined engine.
    /// 
    /// # Returns
    /// A new `Engine` struct with the undefined engine.
    pub fn fresh() -> Self {
        Engine::Undefined
    }

    /// Creates a new `Engine` struct from a string.
    /// 
    /// # Arguments
    /// * `engine` - The engine as a string.
    /// 
    /// # Returns
    /// A new `Engine` struct.
    pub fn from_string(engine: String) -> Self {
        match engine.as_str() {
            "native" => Engine::Native,
            "pytorch" => Engine::PyTorch,
            _ => Engine::Undefined,
        }
    }

    /// Translates the struct to a string.
    /// 
    /// # Returns
    /// * `String` - The struct as a string.
    pub fn to_string(&self) -> String {
        match self {
            Engine::Native => "native".to_string(),
            Engine::PyTorch => "pytorch".to_string(),
            Engine::Undefined => "".to_string(),
        }
    }

}
