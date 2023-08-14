#![recursion_limit = "256"]
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

pub mod storage;
pub mod execution;

#[cfg(feature = "python")]
pub mod python_state;

use crate::storage::python_api::{
    load_model,
    save_model,
    load_cached_raw_model,
    add_column,
    add_output,
    add_normaliser
};


#[pymodule]
fn rust_surrealml(_py: Python, m: &PyModule) -> PyResult<()> {
    let _ = m.add_wrapped(wrap_pyfunction!(load_model));
    let _ = m.add_wrapped(wrap_pyfunction!(save_model));
    let _ = m.add_wrapped(wrap_pyfunction!(load_cached_raw_model));
    let _ = m.add_wrapped(wrap_pyfunction!(add_column));
    let _ = m.add_wrapped(wrap_pyfunction!(add_output));
    let _ = m.add_wrapped(wrap_pyfunction!(add_normaliser));
    Ok(())
}