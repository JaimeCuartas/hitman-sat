use pyo3::prelude::*;

#[pyfunction]
fn sat() -> bool {
    hitman::small_sat()
}

#[pymodule]
fn hitman_sat(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sat, m)?)?;
    Ok(())
}
