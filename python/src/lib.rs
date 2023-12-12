use rule30::prelude::*;
use pyo3::prelude::*;
use rand::Rng;

#[pyclass]
struct Rule30(ExtendedCA);

#[pymethods]
impl Rule30 {
    #[new]
    #[pyo3(signature = (seed=0))]
    fn new(seed: u64) -> Self {
        Rule30(ExtendedCA::seed_from_u64(seed))
    }

    #[pyo3(text_signature = "($self)")]
    fn next_u64(&mut self) -> PyResult<u64> {
        Ok(self.0.next_u64())
    }

    #[pyo3(text_signature = "($self)")]
    fn next_random(&mut self) -> PyResult<f64> {
        Ok(self.0.gen::<f64>())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn rule30py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Rule30>()?;
    Ok(())
}
