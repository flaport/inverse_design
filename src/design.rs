use super::array::new_array;
use super::brushes::{notched_square_brush, touch};
use super::visualization::visualize_mask;

pub fn test_design() {
    let (size_x, size_y): (usize, usize) = (6, 8);
    let (brush_size, notch): (usize, usize) = (5, 1);
    let brush = notched_square_brush(brush_size, notch);
    let mut design = new_array(size_x * size_y, false);
    touch(&mut design, (size_x, size_y), &brush, (0, 6));
    visualize_mask(&design, size_y);
}

