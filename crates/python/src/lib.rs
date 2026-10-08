use pyo3::prelude::*;

#[pyfunction]
fn add(a: u64, b: u64) -> u64 {
    hitman::add(a,b)
}

#[pymodule]
fn hitman_sat(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}
