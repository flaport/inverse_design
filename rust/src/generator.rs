use super::array::read_f32;
use super::brushes::{compute_big_brush, Brush};
use super::design::Design;
use super::profiling::Profiler;
use super::visualize_f32_array;

pub fn test_generator() {
    let profiler = Profiler::start("test_generator");
    let seed = 42;
    let (m, n) = (100, 100);
    let brush = Brush::notched_square(10, 2);
    let latent_t = read_f32(&format!("latent_t_{seed}_{m}x{n}.bin"));
    brush.visualize();
    // visualize_f32_array((m, n), &latent_t);
    let _design = generate_feasible_design((m, n), &latent_t, &brush, true);
    // design.visualize();
    profiler.stop();
}

pub fn generate_feasible_design(
    shape: (usize, usize),
    latent_t: &Vec<f32>,
    brush: &Brush,
    verbose: bool,
) -> Design {
    let profiler = Profiler::start("generate_feasible_design");
    let (m, n) = shape;
    let big_brush = &compute_big_brush(brush);

    let latent_t_abs: Vec<f32> = latent_t.iter().map(|l| l.abs()).collect();

    let mut design = Design::new(shape);
    let mut indices: Vec<(usize, usize)> = (0..m * n).map(|k| (k / n, k % n)).collect();
    sort_indices_by_value(&mut indices, &latent_t_abs, shape);

    loop {
        let (i, j) = match indices.pop() {
            None => break,
            Some(idxs) => idxs,
        };

        // when it's a solid touch, invert everything and treat as void touch.
        let is_solid_touch = latent_t[i * n + j] > 0.0;
        if is_solid_touch {
            design.invert();
        }

        let (mut required_pixels, mut resolving_touches) =
            void_step(&mut design, brush, &big_brush, (i, j));

        resolve_required_pixels(
            &mut design,
            &mut required_pixels,
            &mut resolving_touches,
            brush,
            &big_brush,
            &latent_t_abs,
        );

        // revert inversion
        if is_solid_touch {
            design.invert();
        }
    }

    profiler.stop();
    return design;
}

pub fn void_step(
    design: &mut Design,
    brush: &Brush,
    big_brush: &Brush,
    pos: (usize, usize),
) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let (_, n) = design.shape;
    let (i, j) = pos;
    if (!design.void_touch_valid[i * n + j]) | (design.void_touch_existing[i * n + j]) {
        return (Vec::new(), Vec::new());
    }
    let (required_pixels, resolving_touches) = design.add_void_touch(brush, big_brush, pos);
    return (required_pixels, resolving_touches);
}

pub fn resolve_required_pixels(
    design: &mut Design,
    required_pixels: &mut Vec<(usize, usize)>,
    resolving_touches: &mut Vec<(usize, usize)>,
    brush: &Brush,
    big_brush: &Brush,
    latent_t_abs: &Vec<f32>,
) {
    let (_, n) = design.shape;
    loop {
        sort_indices_by_value(resolving_touches, &latent_t_abs, design.shape);

        let any_required_pixels = required_pixels
            .iter()
            .any(|(i, j)| design.void_pixel_required[i * n + j]);
        if !any_required_pixels {
            break;
        }
        let (ir, jr) = match resolving_touches.pop() {
            None => {
                println!("pop from resolving touches");
                break
            },
            Some(idxs) => idxs,
        };
        let (new_required_pixels, new_resolving_touches) =
            void_step(design, brush, &big_brush, (ir, jr));

        for tr in new_resolving_touches.iter() {
            resolving_touches.push(*tr);
        }

        for pr in new_required_pixels.iter() {
            required_pixels.push(*pr);
        }
    }
}

pub fn solid_step(
    design: &mut Design,
    brush: &Brush,
    big_brush: &Brush,
    pos: (usize, usize),
) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    design.invert();
    let (required_pixels, resolving_touches) = void_step(design, brush, big_brush, pos);
    design.invert();
    return (required_pixels, resolving_touches);
}

pub fn sort_indices_by_value(
    indices: &mut Vec<(usize, usize)>,
    values: &Vec<f32>,
    shape: (usize, usize),
) {
    let (_, n) = shape;
    indices.sort_by(|(i, j), (k, l)| values[*i * n + j].partial_cmp(&values[*k * n + l]).unwrap());
}
