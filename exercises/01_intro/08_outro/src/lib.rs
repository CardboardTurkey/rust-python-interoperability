// TODO: Expose a function named `max_k` that takes a list of unsigned integers and return as output
//   a list containing the `k` largest numbers in the list, in descending order.
//
// Hint: you can use the `num_bigint` crate if you think it'd be useful.
use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
};

#[pyfunction]
fn max_k(mut input: Vec<i128>, k: usize) -> PyResult<Vec<i128>> {
    if input.len() < k {
        return Err(PyValueError::new_err("Hello"));
    }
    for &mut i in &mut input {
        if i < 0 {
            return Err(PyTypeError::new_err("Hello"));
        }
    }
    input.sort();
    input.reverse();
    input.truncate(k);
    Ok(input)
}

#[pymodule]
fn outro1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(max_k, m)?)?;
    Ok(())
}
