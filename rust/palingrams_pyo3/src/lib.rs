use pyo3::prelude::*;
use palingrams_rs::{find_palingrams as fp, find_palingrams_concurrent as fpc};

#[pyfunction]
fn find_palingrams(word_list: Vec<String>) -> PyResult<Vec<(String, String)>> {
    Ok(fp(word_list))
}

#[pyfunction]
fn find_palingrams_concurrent(word_list: Vec<String>) -> PyResult<Vec<(String, String)>> {
    Ok(fpc(word_list))
}

#[pymodule]
fn palingrams_pyo3(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(find_palingrams, m)?)?;
    m.add_function(wrap_pyfunction!(find_palingrams_concurrent, m)?)?;
    Ok(())
}