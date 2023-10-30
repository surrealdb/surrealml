//! Defines the saving and loading of the entire `surml` file.
// use tch::jit::CModule;
use std::fs::File;
use std::io::{self, Read, Write};

use crate::storage::header::Header;


/// The `SurMlFile` struct represents the entire `surml` file.
/// 
/// # Fields
/// * `header` - The header of the `surml` file containing data such as key bindings for inputs and normalisers.
/// * `model` - The PyTorch model in C.
pub struct SurMlFile {
    pub header: Header,
    pub model: Vec<u8>,
}


impl SurMlFile {

    /// Creates a new `SurMlFile` struct with an empty header.
    /// 
    /// # Arguments
    /// * `model` - The PyTorch model in C.
    /// 
    /// # Returns
    /// A new `SurMlFile` struct with no columns or normalisers.
    pub fn fresh(model: Vec<u8>) -> Self {
        Self {
            header: Header::fresh(),
            model
        }
    }

    /// Creates a new `SurMlFile` struct.
    /// 
    /// # Arguments
    /// * `header` - The header of the `surml` file containing data such as key bindings for inputs and normalisers.
    /// * `model` - The PyTorch model in C.
    /// 
    /// # Returns
    /// A new `SurMlFile` struct.
    pub fn new(header: Header, model: Vec<u8>) -> Self {
        Self {
            header,
            model,
        }
    }

    /// Creates a new `SurMlFile` struct from a vector of bytes.
    /// 
    /// # Arguments
    /// * `bytes` - A vector of bytes representing the header and the model.
    /// 
    /// # Returns
    /// A new `SurMlFile` struct.
    pub fn from_bytes(bytes: Vec<u8>) -> io::Result<Self> {
        let mut header_bytes = Vec::new();
        let mut model_bytes = Vec::new();

        // extract the first 4 bytes as an integer to get the length of the header
        let mut buffer = [0u8; 4];
        buffer.copy_from_slice(&bytes[0..4]);
        let integer_value = u32::from_be_bytes(buffer);

        // Read the next integer_value bytes for the header
        header_bytes.extend_from_slice(&bytes[4..(4 + integer_value as usize)]);

        // Read the remaining bytes for the model
        model_bytes.extend_from_slice(&bytes[(4 + integer_value as usize)..]);

        // construct the header and C model from the bytes
        let header = Header::from_bytes(header_bytes).unwrap();
        let model = model_bytes;
        Ok(Self {
            header,
            model,
        })
    }

    /// Creates a new `SurMlFile` struct from a file.
    /// 
    /// # Arguments
    /// * `file_path` - The path to the `surml` file.
    /// 
    /// # Returns
    /// A new `SurMlFile` struct.
    pub fn from_file(file_path: &str) -> io::Result<Self> {
        let mut file = File::open(file_path)?;

        // extract the first 4 bytes as an integer to get the length of the header
        let mut buffer = [0u8; 4];
        file.read_exact(&mut buffer)?;
        let integer_value = u32::from_be_bytes(buffer);

        // Read the next integer_value bytes for the header
        let mut header_buffer = vec![0u8; integer_value as usize];
        file.read_exact(&mut header_buffer)?;

        // Create a Vec<u8> to store the data
        let mut model_buffer = Vec::new();

        // Read the rest of the file into the buffer
        file.take(usize::MAX as u64).read_to_end(&mut model_buffer)?;

        // construct the header and C model from the bytes
        let header = Header::from_bytes(header_buffer).unwrap();
        Ok(Self {
            header,
            model: model_buffer,
        })
    }

    /// Converts the header and the model to a vector of bytes.
    /// 
    /// # Returns
    /// A vector of bytes representing the header and the model.
    pub fn to_bytes(&self) -> Vec<u8> {
        // compile the header into bytes.
        let (num, header_bytes) = self.header.to_bytes();
        let num_bytes = i32::to_be_bytes(num).to_vec();
        
        // combine the bytes into a single vector
        let mut combined_vec: Vec<u8> = Vec::new();
        combined_vec.extend(num_bytes);
        combined_vec.extend(header_bytes);
        combined_vec.extend(self.model.clone());
        return combined_vec
    }

    /// Writes the header and the model to a `surml` file.
    /// 
    /// # Arguments
    /// * `file_path` - The path to the `surml` file.
    /// 
    /// # Returns
    /// An `io::Result` indicating whether the write was successful.
    pub fn write(&self, file_path: &str) -> io::Result<()> {
        let combined_vec = self.to_bytes();

        // write the bytes to a file
        let mut file = File::create(file_path)?;
        file.write(&combined_vec)?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_write() {

        let mut header = Header::fresh();
        header.add_column(String::from("squarefoot"));
        header.add_column(String::from("num_floors"));
        header.add_output(String::from("house_price"), None);

        let mut file = File::open("./stash/linear_test.onnx").unwrap();

        let mut model_bytes = Vec::new();
        file.read_to_end(&mut model_bytes).unwrap();

        let surml_file = SurMlFile::new(header, model_bytes);
        surml_file.write("./stash/test.surml").unwrap();

        let _ = SurMlFile::from_file("./stash/test.surml").unwrap();
    }

    #[test]
    fn test_write_forrest() {

        let header = Header::fresh();

        let mut file = File::open("./stash/forrest_test.onnx").unwrap();

        let mut model_bytes = Vec::new();
        file.read_to_end(&mut model_bytes).unwrap();

        let surml_file = SurMlFile::new(header, model_bytes);
        surml_file.write("./stash/forrest.surml").unwrap();

        let _ = SurMlFile::from_file("./stash/forrest.surml").unwrap();

    }

}