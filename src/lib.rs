use pyo3::prelude::*;
mod graph;

#[pymodule]
#[pyo3(name = "deers")]
fn my_module(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<graph::Graph>()?;
    Ok(())
}
