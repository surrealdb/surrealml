//! # Rust SurrealML Client
//! This crate is the client for the SurrealML project enabling the client to interact with surml files and run computations on them.
//! Currently this client has the following bindings:
//! - python
//! 
//! ## Storage APIs
//! The direct storage APIs are exposed to the client to enable the client to interact with the surml files directly. This is in a different
//! crate in the `modules/utils` directory 
#![recursion_limit = "256"]
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

pub mod python_apis;

#[cfg(test)]
pub mod transport;

pub mod python_state;

use crate::python_apis::storage::{
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
    add_origin,
    upload_model
};
use crate::python_apis::execution::{
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
    let _ = m.add_wrapped(wrap_pyfunction!(upload_model));
    Ok(())
}