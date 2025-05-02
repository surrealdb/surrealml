pub mod images;
pub mod srt_reciever;
pub mod tags;

#[cfg(feature = "python")]
pub mod python_api;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn data_access_layer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(python_api::read_rgb_image, m)?)?;
    Ok(())
}
