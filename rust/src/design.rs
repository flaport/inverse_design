use super::array::new_array;
use super::brushes::{
    apply_brush, compute_big_brush, compute_very_big_square_brush, multi_apply_brush,
    multi_apply_touch, Brush,
};
use super::debug::Profiler;
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use std::mem::swap;

pub fn test_design() {
    let shape: (usize, usize) = (6, 8);
    let (brush_size, notch): (usize, usize) = (5, 1);

    let brush = Brush::notched_square(brush_size, notch);
    brush.visualize();

    println!("step 1 (new)");
    let mut design = Design::new(shape, brush);
    design.visualize();

    println!("step 2 (0, 6)");
    design.add_void_touch((0, 6));
    design.visualize();

    println!("step 3 (skipped)");
    design.visualize();

    println!("step 4 (0, 0)");
    design.add_solid_touch((0, 0));
    design.visualize();

    println!("step 5 (4, 6)");
    design.add_void_touch((4, 6));
    design.visualize();

    println!("step 6 (skipped)");
    design.visualize();

    println!("step 7 (4, 4)");
    design.add_void_touch((4, 4));
    design.visualize();

    println!("step 8 (skipped)");
    design.visualize();

    println!("step 9 (5, 0)");
    design.add_void_touch((5, 0));
    design.visualize();

    println!("step 10 (skipped)");
    design.visualize();

    println!("step 11 (2, 5)");
    design.add_void_touch((2, 5));
    design.visualize();

    println!("step 12 (skipped)");
    design.visualize();
}

pub struct Design {
    pub shape: (usize, usize),
    pub brush: Brush,
    pub big_brush: Brush,
    pub very_big_brush: Brush,

    pub void: Vec<bool>,  /*  1 */
    pub solid: Vec<bool>, /*  2 */

    pub void_pixel_impossible: Vec<bool>, /*  3 */
    pub void_pixel_existing: Vec<bool>,   /*  4 */
    pub void_pixel_required: Vec<bool>,   /*  6 */

    pub solid_pixel_impossible: Vec<bool>, /*  3 */
    pub solid_pixel_existing: Vec<bool>,   /*  4 */
    pub solid_pixel_required: Vec<bool>,   /*  6 */

    pub void_touch_required: Vec<bool>, /*  7 */
    pub void_touch_invalid: Vec<bool>,  /*  8 */
    pub void_touch_existing: Vec<bool>, /*  9 */

    pub solid_touch_required: Vec<bool>, /*  7 */
    pub solid_touch_invalid: Vec<bool>,  /*  8 */
    pub solid_touch_existing: Vec<bool>, /*  9 */
}

impl Design {
    pub fn new(shape: (usize, usize), brush: Brush) -> Self {
        let (size_x, size_y) = shape;
        let big_brush = compute_big_brush(&brush);
        let very_big_brush = compute_very_big_square_brush(&brush);

        return Self {
            shape: (size_x, size_y),
            brush,
            big_brush,
            very_big_brush,

            void: new_array(size_x * size_y, false),
            solid: new_array(size_x * size_y, false),

            void_pixel_impossible: new_array(size_x * size_y, false),
            void_pixel_existing: new_array(size_x * size_y, false),
            void_pixel_required: new_array(size_x * size_y, false),

            solid_pixel_impossible: new_array(size_x * size_y, false),
            solid_pixel_existing: new_array(size_x * size_y, false),
            solid_pixel_required: new_array(size_x * size_y, false),

            void_touch_required: new_array(size_x * size_y, false),
            void_touch_invalid: new_array(size_x * size_y, false),
            void_touch_existing: new_array(size_x * size_y, false),

            solid_touch_required: new_array(size_x * size_y, false),
            solid_touch_invalid: new_array(size_x * size_y, false),
            solid_touch_existing: new_array(size_x * size_y, false),
        };
    }

    pub fn add_void_touch(
        &mut self,
        pos: (usize, usize),
    ) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
        let profiler = Profiler::start("add_void_touch");

        self.void_brush_at_pos(pos);
        self.void_touch_at_pos(pos);
        self.big_void_brush_at_pos(pos);
        let required_pixels = self.find_required_pixels_around_pos(pos);
        self.take_free_void_touches_around_pos(pos);
        let resolving_touches = self.find_resolving_touches_for_required_pixels(&required_pixels);
        profiler.stop();
        return (required_pixels, resolving_touches);
    }

    pub fn add_solid_touch(
        &mut self,
        pos: (usize, usize),
    ) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
        self.invert();
        let (required_pixels, resolving_touches) = self.add_void_touch(pos);
        self.invert();
        return (required_pixels, resolving_touches);
    }

    fn big_void_brush_at_pos(&mut self, pos: (usize, usize)) {
        apply_brush(
            self.shape,
            &mut self.solid_touch_invalid,
            &self.big_brush,
            pos,
            true,
        )
    }

    fn void_brush_at_pos(&mut self, pos: (usize, usize)) {
        multi_apply_brush(
            self.shape,
            &mut vec![
                &mut self.void,
                &mut self.void_pixel_impossible,
                &mut self.void_pixel_existing,
                &mut self.void_pixel_required,
                &mut self.solid_pixel_impossible,
                &mut self.solid_pixel_existing,
                &mut self.solid_pixel_required,
            ],
            &self.brush,
            pos,
            &vec![true, false, true, false, true, false, false],
        );
    }

    fn void_touch_at_pos(&mut self, pos: (usize, usize)) {
        multi_apply_touch(
            self.shape,
            &mut vec![
                &mut self.void_touch_required,
                &mut self.void_touch_invalid,
                &mut self.void_touch_existing,
                &mut self.solid_touch_required,
                &mut self.solid_touch_existing,
            ],
            pos,
            &vec![false, false, true, false, false],
        );
    }
    pub fn invert(&mut self) {
        swap(&mut self.void, &mut self.solid);
        swap(
            &mut self.void_pixel_impossible,
            &mut self.solid_pixel_impossible,
        );
        swap(
            &mut self.void_pixel_existing,
            &mut self.solid_pixel_existing,
        );
        swap(
            &mut self.void_pixel_required,
            &mut self.solid_pixel_required,
        );
        swap(
            &mut self.void_touch_required,
            &mut self.solid_touch_required,
        );
        swap(&mut self.void_touch_invalid, &mut self.solid_touch_invalid);
        swap(
            &mut self.void_touch_existing,
            &mut self.solid_touch_existing,
        );
    }
    fn take_free_void_touches_around_pos(&mut self, pos: (usize, usize)) {
        let profiler1 = Profiler::start("find_free");
        let free: Vec<(usize, usize)> = self
            .very_big_brush
            .at(pos, self.shape)
            .into_iter()
            .par_bridge()
            .filter(|pos| {
                is_free_touch(
                    *pos,
                    &self.brush,
                    self.shape,
                    &self.void_pixel_existing,
                    &self.void_pixel_required,
                )
            })
            .collect();
        profiler1.stop();

        let profiler2 = Profiler::start("take_free");
        for pos in free.into_iter() {
            self.void_touch_at_pos(pos);
            self.void_brush_at_pos(pos);
        }
        profiler2.stop();
    }
    fn find_required_pixels_around_pos(&mut self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let profiler1 = Profiler::start("find_required");

        let required_pixels: Vec<(usize, usize)> = self
            .very_big_brush
            .at(pos, self.shape)
            .into_iter()
            .par_bridge()
            .filter(|pos| {
                is_required_pixel(
                    *pos,
                    self.shape,
                    &self.brush,
                    &self.void_pixel_existing,
                    &self.void_pixel_impossible,
                    &self.solid_touch_invalid,
                )
            })
            .collect();
        profiler1.stop();

        let profiler2 = Profiler::start("flag_required");
        for (ip, jp) in required_pixels.iter() {
            multi_apply_touch(
                self.shape,
                &mut vec![&mut self.void_pixel_required],
                (*ip, *jp),
                &vec![true],
            );
        }
        profiler2.stop();

        return required_pixels;
    }

    fn find_resolving_touches_for_required_pixels(
        &mut self,
        required_pixels: &Vec<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        let (_, n) = self.shape;
        let resolving_touches: Vec<(usize, usize)> = required_pixels
            .into_iter()
            .filter(|(i, j)| !self.void_pixel_existing[i * n + j])
            .map(|(i, j)| {
                self.brush
                    .at((*i, *j), self.shape)
                    .into_iter()
                    .filter(|(i, j)| !self.void_touch_invalid[*i * n + j])
            })
            .flatten()
            .collect();
        return resolving_touches;
    }
}

fn is_required_pixel(
    pos: (usize, usize),
    shape: (usize, usize),
    brush: &Brush,
    void_pixel_existing: &Vec<bool>,
    void_pixel_impossible: &Vec<bool>,
    solid_touch_invalid: &Vec<bool>,
) -> bool {
    let (ip, jp) = pos;
    let (_, n) = shape;
    if void_pixel_existing[ip * n + jp] | void_pixel_impossible[ip * n + jp] {
        return false;
    }
    let all_touches_invalid = brush
        .at((ip, jp), shape)
        .iter()
        .all(|(i, j)| solid_touch_invalid[i * n + j]);
    return all_touches_invalid;
}

fn is_free_touch(
    pos: (usize, usize),
    brush: &Brush,
    shape: (usize, usize),
    void_pixel_existing: &Vec<bool>,
    void_pixel_required: &Vec<bool>,
) -> bool {
    let (_, n) = shape;
    let is_free_touch = brush
        .at(pos, shape)
        .iter()
        .all(|(i_, j_)| void_pixel_existing[i_ * n + j_] | void_pixel_required[i_ * n + j_]);
    return is_free_touch;
}
