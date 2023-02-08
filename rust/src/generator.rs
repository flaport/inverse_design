use super::array::read_f32;
use super::brushes::Brush;
use super::debug::{counter, Profiler};
use super::design::Design;
use std::mem::swap;
// use super::visualize_f32_array;

pub fn test_generator() {
    let profiler = Profiler::start("test_generator");
    let seed = 42;
    let (m, n) = (30, 30);
    let brush = Brush::notched_square(5, 1);
    let latent_t = read_f32(&format!("latent_t_{seed}_{m}x{n}.bin"));
    brush.visualize();
    // visualize_f32_array((m, n), &latent_t);
    let design = generate_feasible_design((m, n), &latent_t, brush, false);
    design.visualize();
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

    let mut void_latent_t: Vec<f32> = latent_t.iter().map(|l| -l).collect();
    let mut solid_latent_t: Vec<f32> = latent_t.iter().map(|l| *l).collect();

    let mut design = Design::new(shape, brush);
    let mut solid_indices: Vec<(usize, usize)> = (0..m * n).map(|k| (k / n, k % n)).collect();
    sort_indices_by_value(&mut solid_indices, &solid_latent_t, shape);
    let mut void_indices: Vec<(usize, usize)> = (0..m * n).map(|k| (k / n, k % n)).collect();
    sort_indices_by_value(&mut void_indices, &void_latent_t, shape);

    let mut prev_idxs = vec![(m, n), (m, n)];
    let mut prev_indexer = 0;
    loop {
        let ijv = void_indices.pop();
        let ijs = solid_indices.pop();

        let ((iv, jv), (is, js)) = match (ijv, ijs) {
            (None, None) => break,
            (Some(ijv), None) => (ijv, ijv),
            (None, Some(ijs)) => (ijs, ijs),
            (Some(ijv), Some(ijs)) => (ijv, ijs),
        };

        let latent_v = void_latent_t[iv * n + jv];
        let latent_s = solid_latent_t[is * n + js];

        let is_solid_touch = latent_s > latent_v;

        let (i, j) = if is_solid_touch {
            void_indices.push((iv, jv));
            (is, js)
        } else {
            solid_indices.push((is, js));
            (iv, jv)
        };

        // I thought I would not need a check like this, but I was wrong...
        if (prev_idxs[0] == (i, j)) & (prev_idxs[1] == (i, j)) {
            break;
        } else {
            prev_idxs[prev_indexer] = (i, j);
            prev_indexer = (prev_indexer + 1) % 2
        }

        if is_solid_touch {
            design.invert();
            swap(&mut solid_latent_t, &mut void_latent_t);
        }

        let void_touch_possible =
            !(design.void_touch_invalid[i * n + j] | design.void_touch_existing[i * n + j]);

        if !void_touch_possible {
            if is_solid_touch {
                design.invert();
                swap(&mut solid_latent_t, &mut void_latent_t);
            }
            continue;
        }

        if verbose {
            println!("iteration {}", counter().value());
        }

        if verbose {
            if is_solid_touch {
                println!("touch solid ({i}, {j})");
            } else {
                println!("touch void ({i}, {j}).");
            }
        }

        let (mut required_pixels, mut resolving_touches) = void_step(&mut design, (i, j));

        resolve_required_void_pixels(
            &mut design,
            &mut required_pixels,
            &mut resolving_touches,
            &void_latent_t,
            is_solid_touch,
            verbose,
        );

        // revert inversion
        if is_solid_touch {
            design.invert();
            swap(&mut solid_latent_t, &mut void_latent_t);
        }

        //design.visualize();

        //if counter().gt(20) {
        //    break;
        //}
        counter().inc();
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
    if (design.void_touch_invalid[i * n + j]) | (design.void_touch_existing[i * n + j]) {
        return (Vec::new(), Vec::new());
    }
    let (required_pixels, resolving_touches) = design.add_void_touch(pos);
    return (required_pixels, resolving_touches);
}

pub fn resolve_required_void_pixels(
    design: &mut Design,
    required_pixels: &mut Vec<(usize, usize)>,
    resolving_touches: &mut Vec<(usize, usize)>,
    void_latent_t: &Vec<f32>,
    is_solid_touch: bool,
    verbose: bool,
) {
    let profiler = Profiler::start("resolving");
    let (_, n) = design.shape;
    loop {
        sort_indices_by_value(resolving_touches, &void_latent_t, design.shape);

        let any_required_pixels = required_pixels
            .iter()
            .any(|(i, j)| design.void_pixel_required[i * n + j]);
        if !any_required_pixels {
            break;
        }
        let (ir, jr) = match resolving_touches.pop() {
            None => {
                // println!("pop from resolving touches");
                break;
            }
            Some(idxs) => idxs,
        };

        counter().inc();
        let (mut new_required_pixels, mut new_resolving_touches) = void_step(design, (ir, jr));
        if verbose {
            println!("iteration {}", counter().value());
            if is_solid_touch {
                println!("resolve solid ({ir}, {jr}).");
            } else {
                println!("resolve void ({ir}, {jr}).");
            }
        }

        swap(required_pixels, &mut new_required_pixels);
        swap(resolving_touches, &mut new_resolving_touches);

        sort_indices_by_value(resolving_touches, &void_latent_t, design.shape);
    }
    profiler.stop();
}

pub fn sort_indices_by_value(
    indices: &mut Vec<(usize, usize)>,
    values: &Vec<f32>,
    shape: (usize, usize),
) {
    let (_, n) = shape;
    indices.sort_by(|(i, j), (k, l)| values[*i * n + j].partial_cmp(&values[*k * n + l]).unwrap());
}
