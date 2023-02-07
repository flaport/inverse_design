use super::array::new_array;
use super::brushes::{
    apply_brush, apply_touch, compute_big_brush, compute_very_big_square_brush, multi_apply_brush,
    multi_apply_touch, Brush,
};
use super::profiling::Profiler;
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

    pub unassigned: Vec<bool>, /*  0 */
    pub void: Vec<bool>,       /*  1 */
    pub solid: Vec<bool>,      /*  2 */

    pub void_pixel_impossible: Vec<bool>, /*  3 */
    pub void_pixel_existing: Vec<bool>,   /*  4 */
    pub void_pixel_possible: Vec<bool>,   /*  5 */
    pub void_pixel_required: Vec<bool>,   /*  6 */

    pub solid_pixel_impossible: Vec<bool>, /*  3 */
    pub solid_pixel_existing: Vec<bool>,   /*  4 */
    pub solid_pixel_possible: Vec<bool>,   /*  5 */
    pub solid_pixel_required: Vec<bool>,   /*  6 */

    pub void_touch_required: Vec<bool>,  /*  7 */
    pub void_touch_invalid: Vec<bool>,   /*  8 */
    pub void_touch_existing: Vec<bool>,  /*  9 */
    pub void_touch_valid: Vec<bool>,     /* 10 */
    pub void_touch_free: Vec<bool>,      /* 11 */
    pub void_touch_resolving: Vec<bool>, /* 12 */

    pub solid_touch_required: Vec<bool>,  /*  7 */
    pub solid_touch_invalid: Vec<bool>,   /*  8 */
    pub solid_touch_existing: Vec<bool>,  /*  9 */
    pub solid_touch_valid: Vec<bool>,     /* 10 */
    pub solid_touch_free: Vec<bool>,      /* 11 */
    pub solid_touch_resolving: Vec<bool>, /* 12 */
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

            unassigned: new_array(size_x * size_y, true),
            void: new_array(size_x * size_y, false),
            solid: new_array(size_x * size_y, false),

            void_pixel_impossible: new_array(size_x * size_y, false),
            void_pixel_existing: new_array(size_x * size_y, false),
            void_pixel_possible: new_array(size_x * size_y, true),
            void_pixel_required: new_array(size_x * size_y, false),

            solid_pixel_impossible: new_array(size_x * size_y, false),
            solid_pixel_existing: new_array(size_x * size_y, false),
            solid_pixel_possible: new_array(size_x * size_y, true),
            solid_pixel_required: new_array(size_x * size_y, false),

            void_touch_required: new_array(size_x * size_y, false),
            void_touch_invalid: new_array(size_x * size_y, false),
            void_touch_existing: new_array(size_x * size_y, false),
            void_touch_valid: new_array(size_x * size_y, true),
            void_touch_free: new_array(size_x * size_y, false),
            void_touch_resolving: new_array(size_x * size_y, false),

            solid_touch_required: new_array(size_x * size_y, false),
            solid_touch_invalid: new_array(size_x * size_y, false),
            solid_touch_existing: new_array(size_x * size_y, false),
            solid_touch_valid: new_array(size_x * size_y, true),
            solid_touch_free: new_array(size_x * size_y, false),
            solid_touch_resolving: new_array(size_x * size_y, false),
        };
    }

    pub fn add_void_touch(
        &mut self,
        pos: (usize, usize),
    ) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
        let profiler = Profiler::start("add_touch");

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

    fn find_resolving_touches_for_required_pixels(
        &mut self,
        required_pixels: &Vec<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        let (_, n) = self.shape;
        let mut resolving_touches = Vec::new();
        for (i, j) in required_pixels.into_iter() {
            if self.void_pixel_existing[i * n + j] {
                continue;
            }
            for (it, jt) in self.brush.at((*i, *j), self.shape).iter() {
                if self.void_touch_invalid[*it * n + *jt] {
                    continue;
                }
                apply_touch(self.shape, &mut self.void_touch_resolving, (*it, *jt), true);
                resolving_touches.push((*it, *jt));
            }
        }
        return resolving_touches;
    }

    fn find_required_pixels_around_pos(&mut self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (_, n) = self.shape;

        let very_big_brush_pixels = self.very_big_brush.at(pos, self.shape);
        let mut relevant_pixels = Vec::new();
        for (i, j) in very_big_brush_pixels.iter() {
            //println!("{i} {j}");
            if !(self.void_pixel_existing[i * n + j] | self.void_pixel_impossible[i * n + j]) {
                relevant_pixels.push((*i, *j));
            }
        }

        let mut required_pixels = Vec::new();
        for (ip, jp) in relevant_pixels.iter() {
            let all_touches_invalid = self
                .brush
                .at((*ip, *jp), self.shape)
                .iter()
                .all(|(i, j)| self.solid_touch_invalid[i * n + j]);
            if all_touches_invalid {
                multi_apply_touch(
                    self.shape,
                    &mut vec![&mut self.void_pixel_required],
                    (*ip, *jp),
                    &vec![true],
                );
                required_pixels.push((*ip, *jp));
            }
        }

        return required_pixels;
    }

    fn take_free_void_touches_around_pos(&mut self, pos: (usize, usize)) {
        let (_, n) = self.shape;
        for pos_ in self.big_brush.at(pos, self.shape) {
            if pos_ == pos {
                continue;
            }
            let is_free_touch = self.brush.at(pos_, self.shape).iter().all(|(i_, j_)| {
                self.void_pixel_existing[i_ * n + j_] | self.void_pixel_required[i_ * n + j_]
            });
            if is_free_touch {
                self.void_touch_at_pos(pos_);
                self.void_brush_at_pos(pos_);
            }
        }
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
                &mut self.unassigned,
                &mut self.void,
                &mut self.void_pixel_impossible,
                &mut self.void_pixel_existing,
                &mut self.void_pixel_possible,
                &mut self.void_pixel_required,
                &mut self.solid_pixel_impossible,
                &mut self.solid_pixel_existing,
                &mut self.solid_pixel_possible,
                &mut self.solid_pixel_required,
            ],
            &self.brush,
            pos,
            &vec![
                true, true, false, true, false, false, true, false, false, false,
            ],
        );
    }

    fn void_touch_at_pos(&mut self, pos: (usize, usize)) {
        multi_apply_touch(
            self.shape,
            &mut vec![
                &mut self.void_touch_required,
                &mut self.void_touch_invalid,
                &mut self.void_touch_existing,
                &mut self.void_touch_valid,
                &mut self.void_touch_free,
                &mut self.void_touch_resolving,
                &mut self.solid_touch_required,
                &mut self.solid_touch_existing,
                &mut self.solid_touch_valid,
                &mut self.solid_touch_free,
                &mut self.solid_touch_resolving,
            ],
            pos,
            &vec![
                false, false, true, false, false, false, false, false, false, false, false,
            ],
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
            &mut self.void_pixel_possible,
            &mut self.solid_pixel_possible,
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
        swap(&mut self.void_touch_valid, &mut self.solid_touch_valid);
        swap(&mut self.void_touch_free, &mut self.solid_touch_free);
        swap(
            &mut self.void_touch_resolving,
            &mut self.solid_touch_resolving,
        );
    }
}
