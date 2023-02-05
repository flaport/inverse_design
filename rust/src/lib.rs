pub mod brushes;
pub mod conditional_generator;
pub mod design;
pub mod utils;
pub mod visualization;
pub mod profiling;

use arrayfire::transpose_inplace;
use conditional_generator::{generate_feasible_design as generate_feasible_design_rs, parse_array};
use pyo3::prelude::{pyfunction, pymodule, PyModule, PyResult, Python};
use pyo3::wrap_pyfunction;
use utils::buffer;

#[pyfunction]
fn generate_feasible_design(
    latent_t_shape: (usize, usize),
    latent_t_bytes: Vec<u8>,
    brush_shape: (usize, usize),
    brush_bytes: Vec<u8>,
    verbose: bool,
) -> PyResult<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)> {
    let (m, n) = latent_t_shape;
    let latent_t = parse_array(&latent_t_bytes, m, n);

    let (m, n) = brush_shape;
    let brush = parse_array(&brush_bytes, m, n).cast::<bool>();

    //let mut profiler = Profiler::new();
    let mut design = generate_feasible_design_rs(&latent_t, &brush, verbose);

    // F -> C memory layout
    transpose_inplace(&mut design.void_pixels, false);
    transpose_inplace(&mut design.solid_pixels, false);
    transpose_inplace(&mut design.void_touches, false);
    transpose_inplace(&mut design.solid_touches, false);

    return Ok((
        buffer(&design.void_pixels),
        buffer(&design.solid_pixels),
        buffer(&design.void_touches),
        buffer(&design.solid_touches),
    ));
}

#[pymodule]
fn inverse_design_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_feasible_design, m)?)?;
    Ok(())
}
