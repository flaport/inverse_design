pub mod utils;
pub mod brushes;
pub mod visualization;
pub mod design;
pub mod conditional_generator;


//use conditional_generator::generate_feasible_design as rn_generate_feasible_design;
use pyo3::prelude::{pyfunction,pymodule, PyResult, Python, PyModule};
use pyo3::wrap_pyfunction;

#[pyfunction]
fn generate_feasible_design(
    latent_t: String,
) -> PyResult<String> {
    let result = latent_t;
    println!("rust: {result:?}");
    return Ok(result);
}

#[pymodule]
fn inverse_design_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_feasible_design, m)?)?;
    Ok(())
}
