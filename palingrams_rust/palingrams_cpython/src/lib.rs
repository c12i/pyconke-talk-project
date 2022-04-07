use cpython::{py_fn, py_module_initializer, PyResult, Python};
use palingrams_rs::{find_palingrams, find_palingrams_concurrent};

py_module_initializer!(palingrams_cpython, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(
        py,
        "find_palingrams",
        py_fn!(py, find_palingrams_py(word_list: Vec<String>)),
    )?;
    m.add(
        py,
        "find_palingrams_concurrent",
        py_fn!(py, find_palingrams_concurrent_py(word_list: Vec<String>)),
    )?;
    Ok(())
});

fn find_palingrams_py(_: Python, word_list: Vec<String>) -> PyResult<Vec<(String, String)>> {
    let result = find_palingrams(word_list);
    Ok(result)
}

fn find_palingrams_concurrent_py(
    _: Python,
    word_list: Vec<String>,
) -> PyResult<Vec<(String, String)>> {
    let result = find_palingrams_concurrent(word_list);
    Ok(result)
}
