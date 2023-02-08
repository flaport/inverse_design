use super::array::parse_f32;
use super::brushes::Brush;
use super::debug::print_profiler_summary as print_profiler_summary_rs;
use super::generator::generate_feasible_design as generate_feasible_design_rs;
use pyo3::prelude::{pyfunction, pymodule, PyModule, PyResult, Python};
use pyo3::wrap_pyfunction;

#[pymodule]
fn inverse_design_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_feasible_design, m)?)?;
    m.add_function(wrap_pyfunction!(print_profiler_summary, m)?)?;
    Ok(())
}

#[pyfunction]
pub fn generate_feasible_design(
    latent_t_shape: (usize, usize),
    latent_t_bytes: Vec<u8>,
    brush_shape: (usize, usize),
    brush_bytes: Vec<u8>,
    verbose: bool,
) -> (Vec<bool>, Vec<bool>, Vec<bool>) {
    let latent_t = parse_f32(&latent_t_bytes);
    let brush = Brush::from_f32_mask(brush_shape, &parse_f32(&brush_bytes));
    let design = generate_feasible_design_rs(latent_t_shape, &latent_t, brush, verbose);
    return (
        design.void,
        design.void_touch_existing,
        design.solid_touch_existing,
    );
}

#[pyfunction]
pub fn print_profiler_summary() {
    print_profiler_summary_rs();
}
