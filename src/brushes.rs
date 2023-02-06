use super::array::{k, new_array};
use super::visualization::visualize_mask;

pub fn test_brushes() {
    let (size, notch) = (5, 1);
    let brush = notched_square_brush(size, notch);
    println!("{}", brush.len());
    let mask = brush_mask(&brush, size);
    visualize_mask(&mask, size);
}

pub fn notched_square_brush(width: usize, notch: usize) -> Vec<(i32, i32)> {
    let mut brush = Vec::new();
    for i in 0..width {
        for j in 0..width {
            if (i < notch) & (j < notch) {
                continue;
            }
            if (width - notch <= i) & (j < notch) {
                continue;
            }
            if (i < notch) & (width - notch <= j) {
                continue;
            }
            if (width - notch <= i) & (width - notch <= j) {
                continue;
            }
            brush.push((
                i as i32 - (width / 2) as i32,
                (j as i32) - (width / 2) as i32,
            ));
        }
    }
    return brush;
}

pub fn translate_brush(brush: &Vec<(i32, i32)>, m: i32, n: i32) -> Vec<(i32, i32)> {
    let m = m as i32;
    let n = n as i32;
    let mut new = Vec::new();
    for (i, j) in brush.iter() {
        new.push(((*i + m), (*j + n)));
    }
    return new;
}

pub fn brush_mask(brush: &Vec<(i32, i32)>, size: usize) -> Vec<bool> {
    let brush = translate_brush(brush, (size as i32) / 2, (size as i32) / 2);
    let mut mask = new_array(size * size, false);
    for (i, j) in brush.into_iter() {
        if i < 0 {
            continue;
        }
        if j < 0 {
            continue;
        }
        if (size as i32) <= i {
            continue;
        }
        if (size as i32) <= j {
            continue;
        }
        let idx = k(i as usize, j as usize, size);
        mask[idx] = true;
    }
    return mask;
}
