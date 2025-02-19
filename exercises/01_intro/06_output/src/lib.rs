use pyo3::{prelude::*, types::PyList};

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
fn fibonacci(py: Python<'_>, n: u32) -> Bound<'_, PyList> {
    PyList::new(py, (0..n).map(do_fibonacci).collect::<Vec<_>>()).unwrap()
}

#[pymodule]
fn output(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}
