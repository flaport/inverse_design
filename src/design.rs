use super::array::new_array;
use super::brushes::{apply_brush, apply_touch, notched_square_brush};
use super::visualization::visualize_brush;

pub fn test_design() {
    let shape: (usize, usize) = (6, 8);
    let (brush_size, notch): (usize, usize) = (5, 1);
    let brush = notched_square_brush(brush_size, notch);
    visualize_brush((brush_size, brush_size), &brush);

    println!("step 1");
    let mut design = Design::new(shape);
    design.visualize();

    println!("step 2");
    design.add_void_touch(&brush, (0, 6));
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

    pub fn add_void_touch(&mut self, brush: &Vec<(i32, i32)>, pos: (usize, usize)) {
        apply_brush(self.shape, &mut self.pixels, brush, pos, true);
        apply_brush(self.shape, &mut self.void_pixels, brush, pos, true);

        apply_touch(self.shape, &mut self.touches, pos, true);
        apply_touch(self.shape, &mut self.void_touches, pos, true);
    }
}
