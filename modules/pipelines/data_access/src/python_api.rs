//! Here we define the Python API for data access layer. This is currently just one file, but we will probably
//! expand this module into a directory with multiple files as the project grows as we will also need to define
//! python classes in the future when building out the tags for the surgery steps.
use crate::images::read_rgb_image as read_rgb_image_rust;

use pyo3::prelude::*;

#[pyfunction]
pub fn read_rgb_image(path: String, width: usize, height: usize) -> PyResult<Vec<u8>> {
    let data = read_rgb_image_rust(path, width, height);
    Ok(data)
}
