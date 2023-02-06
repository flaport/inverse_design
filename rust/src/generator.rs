use super::brushes::Brush;
use super::array::read_f32;
use super::visualize_f32_array;

pub fn test_generator() {
    let seed = 42;
    let (m, n) = (30, 30);
    let brush = Brush::notched_square(5, 1);
    // let latent = new_latent_design(shape, 0.0);
    let latent_t = read_f32(&format!("latent_t_{seed}_{m}x{n}.bin"));
    brush.visualize();
    visualize_f32_array((m, n), &latent_t);
    // visualize_array(&(6.0 * (&latent_t + 1.0)));
    // let _design = generate_feasible_design(&latent_t, &brush, true);
}
