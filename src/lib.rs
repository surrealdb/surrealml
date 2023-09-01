#![recursion_limit = "256"]
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

pub mod storage;
pub mod execution;

#[cfg(test)]
pub mod transport;

#[cfg(feature = "python")]
pub mod python_state;

use crate::storage::python_api::{
    load_model,
    save_model,
    load_cached_raw_model,
    add_column,
    add_output,
    add_normaliser,
    add_name,
    delete_cached_model,
    add_description,
    add_version,
    to_bytes,
    add_engine,
    add_author,
    add_origin
};
use crate::execution::python_api::{
    raw_compute,
    buffered_compute
};


#[pymodule]
fn rust_surrealml(_py: Python, m: &PyModule) -> PyResult<()> {
    let _ = m.add_wrapped(wrap_pyfunction!(load_model));
    let _ = m.add_wrapped(wrap_pyfunction!(save_model));
    let _ = m.add_wrapped(wrap_pyfunction!(load_cached_raw_model));
    let _ = m.add_wrapped(wrap_pyfunction!(add_column));
    let _ = m.add_wrapped(wrap_pyfunction!(add_output));
    let _ = m.add_wrapped(wrap_pyfunction!(add_normaliser));
    let _ = m.add_wrapped(wrap_pyfunction!(add_name));
    let _ = m.add_wrapped(wrap_pyfunction!(add_description));
    let _ = m.add_wrapped(wrap_pyfunction!(add_version));
    let _ = m.add_wrapped(wrap_pyfunction!(raw_compute));
    let _ = m.add_wrapped(wrap_pyfunction!(buffered_compute));
    let _ = m.add_wrapped(wrap_pyfunction!(delete_cached_model));
    let _ = m.add_wrapped(wrap_pyfunction!(to_bytes));
    let _ = m.add_wrapped(wrap_pyfunction!(add_engine));
    let _ = m.add_wrapped(wrap_pyfunction!(add_author));
    let _ = m.add_wrapped(wrap_pyfunction!(add_origin));
    Ok(())
}