// use pyo3::exceptions::PyValueError;
// use pyo3::prelude::*;
// mod core;

// /// Formats the sum of two numbers as string.
// #[pyfunction]
// pub fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

// /// Sets up the project
// #[pyfunction]
// pub fn setup() -> PyResult<()> {
//     match core::setup().await {
//         Ok(()) => Ok(()),
//         Err(_s) => Err(PyValueError::new_err("Error occured during setup!")),
//     }
// }

// /// A Python module implemented in Rust.
// #[pymodule]
// fn algovault(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
//     m.add_function(wrap_pyfunction!(setup, m)?)?;
//     Ok(())
// }
