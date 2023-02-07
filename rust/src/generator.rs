use super::array::read_f32;
use super::brushes::Brush;
use super::design::Design;
use super::profiling::Profiler;
// use super::visualize_f32_array;

pub fn test_generator() {
    let profiler = Profiler::start("test_generator");
    let seed = 42;
    let (m, n) = (30, 30);
    let brush = Brush::notched_square(5, 1);
    let latent_t = read_f32(&format!("latent_t_{seed}_{m}x{n}.bin"));
    brush.visualize();
    // visualize_f32_array((m, n), &latent_t);
    let _design = generate_feasible_design((m, n), &latent_t, brush, true);
    // design.visualize();
    profiler.stop();
}

pub fn generate_feasible_design(
    shape: (usize, usize),
    latent_t: &Vec<f32>,
    brush: Brush,
    verbose: bool,
) -> Design {
    let profiler = Profiler::start("generate_feasible_design");
    let (m, n) = shape;

    let latent_t_abs: Vec<f32> = latent_t.iter().map(|l| l.abs()).collect();

    let mut design = Design::new(shape, brush);
    let mut indices: Vec<(usize, usize)> = (0..m * n).map(|k| (k / n, k % n)).collect();
    sort_indices_by_value(&mut indices, &latent_t_abs, shape);

    let mut I = 0 as usize;
    loop {
        if verbose {
            println!("iteration {I}");
        }

        let loop_profiler = Profiler::start("loop_body");
        let (i, j) = match indices.pop() {
            None => break,
            Some(idxs) => idxs,
        };

        // when it's a solid touch, invert everything and treat as void touch.
        let is_solid_touch = latent_t[i * n + j] > 0.0;
        if is_solid_touch {
            design.invert();
        }
        if verbose {
            if is_solid_touch {
                println!("touch solid ({i}, {j}).");
            } else {
                println!("touch void ({i}, {j}).");
            }
        }

        let (mut required_pixels, mut resolving_touches) = void_step(&mut design, (i, j));

        resolve_required_pixels(
            &mut design,
            &mut required_pixels,
            &mut resolving_touches,
            &latent_t_abs,
        );

        // revert inversion
        if is_solid_touch {
            design.invert();
        }

        loop_profiler.stop();

        design.visualize();
        if I == 4 {
            break;
        }

        I += 1;
    }

    profiler.stop();
    return design;
}

pub fn void_step(
    design: &mut Design,
    pos: (usize, usize),
) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let (_, n) = design.shape;
    let (i, j) = pos;
    if (!design.void_touch_valid[i * n + j]) | (design.void_touch_existing[i * n + j]) {
        return (Vec::new(), Vec::new());
    }
    let (required_pixels, resolving_touches) = design.add_void_touch(pos);
    return (required_pixels, resolving_touches);
}

pub fn resolve_required_pixels(
    design: &mut Design,
    required_pixels: &mut Vec<(usize, usize)>,
    resolving_touches: &mut Vec<(usize, usize)>,
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
                break;
            }
            Some(idxs) => idxs,
        };

        println!("({ir} {jr})");
        break;

        // let (new_required_pixels, new_resolving_touches) =
        //     void_step(design, (ir, jr));

        // for tr in new_resolving_touches.iter() {
        //     resolving_touches.push(*tr);
        // }

        // for pr in new_required_pixels.iter() {
        //     required_pixels.push(*pr);
        // }

        // // TODO: remove below!
        // loop {
        //     match required_pixels.pop() {
        //         Some(_) => continue,
        //         None => break,
        //     }
        // }
        // loop {
        //     match resolving_touches.pop() {
        //         Some(_) => continue,
        //         None => break,
        //     }
        // }
        // break
    }
}

pub fn solid_step(
    design: &mut Design,
    pos: (usize, usize),
) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    design.invert();
    let (required_pixels, resolving_touches) = void_step(design, pos);
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
