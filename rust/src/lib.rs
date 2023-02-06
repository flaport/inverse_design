pub mod array;
pub mod brushes;
pub mod design;
pub mod profiling;
pub mod status;
pub mod visualization;

use pyo3::prelude::{pyfunction, pymodule, PyModule, PyResult, Python};
use pyo3::wrap_pyfunction;
use array::parse_f32;
use visualization::visualize_f32_array;
use brushes::Brush;

#[pyfunction]
fn generate_feasible_design(
    latent_t_shape: (usize, usize),
    latent_t_bytes: Vec<u8>,
    brush_shape: (usize, usize),
    brush_bytes: Vec<u8>,
    verbose: bool,
) { // -> PyResult<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)> {
    let latent_t = parse_f32(&latent_t_bytes);
    let brush = Brush::from_f32_mask(brush_shape, &parse_f32(&brush_bytes));
    visualize_f32_array(latent_t_shape, &latent_t);
    brush.visualize();
}

#[pymodule]
fn inverse_design_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_feasible_design, m)?)?;
    Ok(())
}
