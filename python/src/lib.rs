use pyo3::prelude::*;
use rand::Rng;
use rule30::prelude::*;

// Copied from src/extended_ca.rs
const SIZE: usize = 80;

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

    #[pyo3(text_signature = "($self)")]
    fn get_state(&self) -> PyResult<Vec<u64>> {
        Ok(self.0.state().to_vec())
    }

    #[pyo3(text_signature = "($self, state)")]
    fn set_state(&mut self, state: Vec<u64>) -> PyResult<()> {
        if state.len() != SIZE {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "state must be of length 80",
            ));
        }
        match state.try_into() {
            Ok(state) => {
                self.0.reset(state);
                Ok(())
            }
            Err(_) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "state must be a list of 80 positive integers",
            )),
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn rule30py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Rule30>()?;
    Ok(())
}
