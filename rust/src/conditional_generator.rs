use super::brushes::notched_square_brush;
use super::design::{Design, Status, XYOrMask};
use super::profiling::{Profiler, print_profiler_summary};
use super::utils::{any, argmax2d, argmin2d, dilute, item, randn, sum};
use arrayfire::{
    constant, div, eq, index, or, select, set_seed, tanh, transpose_inplace, Array, Dim4, Seq,
};
use std::fs::{metadata, File};
use std::io::Read;

pub fn test_conditional_generator() {
    set_seed(42);

    let (m, n) = (100, 100);
    let brush = notched_square_brush(5, 1);
    // let latent = new_latent_design(shape, 0.0);
    let latent = read_array(&format!("latent42_{m}x{n}.bin"), m, n);
    let latent_t = transform(&latent, &brush, 5.0);
    // visualize_array(&(6.0 * (&latent_t + 1.0)));
    let _design = generate_feasible_design(&latent_t, &brush, true);
    print_profiler_summary();
}

pub fn generate_feasible_design(
    latent_t: &Array<f32>,
    brush: &Array<bool>,
    verbose: bool,
) -> Design {
    let profiler = Profiler::start("generate_feasible_design");
    let dim4 = latent_t.dims();
    let shape4 = dim4.get();
    let shape = (shape4[0], shape4[1]);
    let mut design = Design::new(shape);
    if verbose {
        println!("create empty design.");
    }

    let mut i = 0 as usize;
    loop {
        if verbose {
            println!("iteration {i}");
        }
        match design.step(latent_t, brush, verbose) {
            Err(_) => break,
            Ok(_) => {}
        }
        // visualize_design(&design);
        i += 1;
    }
    profiler.stop();
    return design;
}

pub fn transform(latent: &Array<f32>, brush: &Array<bool>, beta: f32) -> Array<f32> {
    let mut float_brush = brush.cast::<f32>();
    let brush_total = sum(&float_brush);
    float_brush = div(
        &float_brush,
        &constant(brush_total, float_brush.dims()),
        false,
    );
    let convolved = dilute(&latent, &float_brush);
    return tanh(&(beta * convolved));
}

impl Design {
    pub fn any_unassigned(&self) -> bool {
        any(&eq(
            &self.design(),
            &constant(Status::Unassigned as u8, self.void_pixels.dims()),
            false,
        ))
    }
    pub fn step(
        &mut self,
        latent_t: &Array<f32>,
        brush: &Array<bool>,
        verbose: bool,
    ) -> Result<(), &str> {
        let profiler = Profiler::start("step");
        let dim4 = self.void_touches.dims();
        let void_touch_mask = eq(
            &self.void_touches,
            &constant(Status::TouchValid as u8, dim4),
            false,
        );
        let solid_touch_mask = eq(
            &self.solid_touches,
            &constant(Status::TouchValid as u8, dim4),
            false,
        );
        let touch_mask = or(&solid_touch_mask, &void_touch_mask, false);

        let void_free_mask = eq(
            &self.void_touches,
            &constant(Status::TouchFree as u8, dim4),
            false,
        );
        let solid_free_mask = eq(
            &self.solid_touches,
            &constant(Status::TouchFree as u8, dim4),
            false,
        );
        let free_mask = or(&solid_free_mask, &void_free_mask, false);

        let void_resolving_mask = eq(
            &self.void_touches,
            &constant(Status::TouchResolving as u8, dim4),
            false,
        );
        let solid_resolving_mask = eq(
            &self.solid_touches,
            &constant(Status::TouchResolving as u8, dim4),
            false,
        );
        let resolving_mask = or(&solid_resolving_mask, &void_resolving_mask, false);

        if any(&free_mask) {
            let void_selector = select(&latent_t, &void_free_mask, &constant(0.0, dim4));
            let solid_selector = select(&latent_t, &solid_free_mask, &constant(0.0, dim4));
            if sum(&void_selector).abs() > sum(&solid_selector).abs() {
                self.take_free_void_touches(&brush);
                if verbose {
                    println!("take free void.")
                }
            } else {
                self.take_free_solid_touches(&brush);
                if verbose {
                    println!("take free solid.")
                }
            }
        } else if any(&resolving_mask) {
            let void_needs_resolving = any(&void_resolving_mask);
            let solid_needs_resolving = any(&solid_resolving_mask);
            let void_selector = select(
                &latent_t,
                &void_resolving_mask,
                &constant(f32::INFINITY, dim4),
            );
            let solid_selector = select(
                &latent_t,
                &solid_resolving_mask,
                &constant(f32::NEG_INFINITY, dim4),
            );

            if void_needs_resolving & (!solid_needs_resolving) {
                let (i_v, j_v) = argmin2d(&void_selector);
                self.add_void_touch(&brush, XYOrMask::XY((i_v, j_v)));
                if verbose {
                    println!("resolve void ({i_v}, {j_v}).")
                }
            } else if (!void_needs_resolving) & solid_needs_resolving {
                let (i_s, j_s) = argmax2d(&solid_selector);
                self.add_solid_touch(&brush, XYOrMask::XY((i_s, j_s)));
                if verbose {
                    println!("resolve solid ({i_s}, {j_s}).")
                }
            } else {
                let (i_v, j_v) = argmin2d(&void_selector);
                let (i_s, j_s) = argmax2d(&solid_selector);
                let v = item(&index(
                    &latent_t,
                    &[
                        Seq::new(i_v as f32, i_v as f32, 1.0),
                        Seq::new(j_v as f32, j_v as f32, 1.0),
                    ],
                ));
                let s = item(&index(
                    &latent_t,
                    &[
                        Seq::new(i_s as f32, i_s as f32, 1.0),
                        Seq::new(j_s as f32, j_s as f32, 1.0),
                    ],
                ));
                if v.abs() > s.abs() {
                    self.add_void_touch(&brush, XYOrMask::XY((i_v, j_v)));
                    if verbose {
                        println!("touch void ({i_v}, {j_v}).")
                    }
                } else {
                    self.add_solid_touch(&brush, XYOrMask::XY((i_s, j_s)));
                    if verbose {
                        println!("touch solid ({i_s}, {j_s}).")
                    }
                }
            }
        } else if any(&touch_mask) {
            let void_selector = select(&latent_t, &void_touch_mask, &constant(f32::INFINITY, dim4));
            let solid_selector = select(
                &latent_t,
                &solid_touch_mask,
                &constant(f32::NEG_INFINITY, dim4),
            );
            let (i_v, j_v) = argmin2d(&void_selector);
            let (i_s, j_s) = argmax2d(&solid_selector);
            let v = item(&index(
                &latent_t,
                &[
                    Seq::new(i_v as f32, i_v as f32, 1.0),
                    Seq::new(j_v as f32, j_v as f32, 1.0),
                ],
            ));
            let s = item(&index(
                &latent_t,
                &[
                    Seq::new(i_s as f32, i_s as f32, 1.0),
                    Seq::new(j_s as f32, j_s as f32, 1.0),
                ],
            ));
            if v.abs() > s.abs() {
                self.add_void_touch(&brush, XYOrMask::XY((i_v, j_v)));
                if verbose {
                    println!("touch void ({i_v}, {j_v}).")
                }
            } else {
                self.add_solid_touch(&brush, XYOrMask::XY((i_s, j_s)));
                if verbose {
                    println!("touch solid ({i_s}, {j_s}).")
                }
            }
        } else {
            return Err("No steps possible");
        }
        profiler.stop();
        return Ok(());
    }
}

pub fn new_latent_design(shape: (u64, u64), bias: f32) -> Array<f32> {
    return randn::<f32>(shape) + bias;
}

pub fn read_array(filename: &str, m: usize, n: usize) -> Array<f32> {
    let mut f = File::open(&filename).expect(&format!("no file '{filename}' found."));
    let meta = metadata(&filename).expect(&format!("unable to read '{filename}' metadata."));
    let mut buffer = vec![0; meta.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    return parse_array(&buffer, m, n);
}

pub fn parse_array(bts: &Vec<u8>, m: usize, n: usize) -> Array<f32> {
    let chunks = _chunks(&bts);
    let values: Vec<f32> = chunks.into_iter().map(|a| f32::from_le_bytes(a)).collect();
    let mut array = Array::new(&values, Dim4::new(&[n as u64, m as u64, 1, 1]));
    transpose_inplace(&mut array, false); // C -> F memory layout.
    return array;
}

fn _chunks(barry: &Vec<u8>) -> Vec<[u8; 4]> {
    let n = 4;
    let mut chunks = Vec::new();
    let length = barry.len();

    if length % n != 0 {
        panic!("This should never happen.")
    }

    for i in (0..length).step_by(n) {
        let mut array = [0u8; 4];
        for (j, p) in array.iter_mut().enumerate() {
            *p = barry[i + j];
        }
        chunks.push(array);
    }

    return chunks;
}
