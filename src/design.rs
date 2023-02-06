use super::array::new_array;
use super::brushes::{compute_big_brush, multi_apply_brush, multi_apply_touch, Brush};

pub fn test_design() {
    let shape: (usize, usize) = (6, 8);
    let (brush_size, notch): (usize, usize) = (5, 1);

    let brush = Brush::notched_square(brush_size, notch);
    brush.visualize();

    let big_brush = compute_big_brush(&brush);

    println!("step 1");
    let mut design = Design::new(shape);
    design.visualize();

    println!("step 2");
    design.add_void_touch(&brush, &big_brush, (0, 6));
    design.visualize();
}

pub struct Design {
    pub shape: (usize, usize),

    pub pixels: Vec<bool>,
    pub void_pixels: Vec<bool>,
    pub solid_pixels: Vec<bool>,

    pub touches: Vec<bool>,
    pub void_touches: Vec<bool>,
    pub solid_touches: Vec<bool>,
}

impl Design {
    pub fn new(shape: (usize, usize)) -> Self {
        let (size_x, size_y) = shape;

        let pixels = new_array(size_x * size_y, false);
        let void_pixels = new_array(size_x * size_y, false);
        let solid_pixels = new_array(size_x * size_y, false);

        let touches = new_array(size_x * size_y, false);
        let void_touches = new_array(size_x * size_y, false);
        let solid_touches = new_array(size_x * size_y, false);

        return Self {
            shape,
            pixels,
            void_pixels,
            solid_pixels,
            touches,
            void_touches,
            solid_touches,
        };
    }

    pub fn add_void_touch(&mut self, brush: &Brush, big_brush: &Brush, pos: (usize, usize)) {
        multi_apply_brush(
            self.shape,
            &mut vec![&mut self.pixels, &mut self.void_pixels],
            brush,
            pos,
            &vec![true, true],
        );

        multi_apply_touch(
            self.shape,
            &mut vec![&mut self.touches, &mut self.void_touches],
            pos,
            &vec![true, true],
        );

    }
}
