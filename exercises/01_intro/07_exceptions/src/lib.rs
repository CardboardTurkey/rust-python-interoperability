use pyo3::{
    exceptions::PyTypeError,
    prelude::*,
    types::{PyInt, PyList},
};

pub fn do_fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 | 2 => 1,
        3 => 2,
        /*
        50    => 12586269025,
        */
        _ => do_fibonacci(n - 1) + do_fibonacci(n - 2),
    }
}

#[pyfunction]
// TODO: Implement a function that returns a list containing the first `n` numbers in Fibonacci's sequence.
//  It must raise a `TypeError` if `n` is not an integer or if it is less than 0.
fn fibonacci(py_n: Bound<'_, PyInt>) -> PyResult<Bound<'_, PyList>> {
    let n = py_n.extract::<i32>()?;
    let Ok(n) = u32::try_from(n) else {
        return Err(PyTypeError::new_err("Hello"));
    };
    PyList::new(py_n.py(), (0..n).map(do_fibonacci).collect::<Vec<_>>())
}

#[pymodule]
fn exceptions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}
