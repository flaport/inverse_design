use super::array::{k, new_array};
use super::profiling::Profiler;

pub fn test_brushes() {
    let brush = Brush::notched_square(5, 1);
    let mask = brush.mask();
    let big_brush = compute_big_brush(&brush);

    brush.visualize();
    big_brush.visualize();
}

pub struct Brush {
    pub brush: Vec<(i32, i32)>,
    pub size: (usize, usize),
}

impl Brush {
    pub fn notched_square(width: usize, notch: usize) -> Self {
        let brush = notched_square_brush(width, notch);
        let size = (width, width);
        return Self { brush, size };
    }

    pub fn mask(&self) -> Vec<bool> {
        let (size_i, size_j) = self.size;
        let mut mask = new_array(size_i * size_j, false);
        for (i, j) in self.brush.iter() {
            let i = i + (size_i as i32) / 2;
            let j = j + (size_j as i32) / 2;
            if i < 0 {
                continue;
            }
            if j < 0 {
                continue;
            }
            if (size_i as i32) <= i {
                continue;
            }
            if (size_j as i32) <= j {
                continue;
            }
            let idx = k(i as usize, j as usize, size_j);
            mask[idx] = true;
        }
        return mask;
    }
}

pub fn compute_big_brush(brush: &Brush) -> Brush {
    let (m, n) = brush.size;
    let (m_, n_) = (2 * m, 2 * n);
    let mut mask = new_array(m_ * n_, false);
    for (i, j) in brush.brush.iter() {
        let i_ = (m_ as i32 / 2 + *i) as usize - m % 2;
        let j_ = (n_ as i32 / 2 + *j) as usize - n % 2;
        apply_brush((m_, n_), &mut mask, &brush, (i_, j_), true);
    }

    let mut new_brush = Vec::new();
    for i in 0..m_ {
        for j in 0..n_ {
            if mask[k(i, j, n_)] {
                let i = (i as i32) - (m_ as i32) / 2;
                let j = (j as i32) - (n_ as i32) / 2;
                new_brush.push((i + 1, j + 1)); // yes, +1
            }
        }
    }
    return Brush {
        brush: new_brush,
        size: (m_ - 1, n_ - 1),
    };
}

pub fn apply_touch<T: Copy>(
    shape: (usize, usize),
    array: &mut Vec<T>,
    pos: (usize, usize),
    value: T,
) {
    let profiler = Profiler::start("apply_touch");
    let (_, size_j) = shape;
    let (m, n) = pos;
    let idx = k(m, n, size_j);
    array[idx] = value;
    profiler.stop();
}

pub fn apply_brush<T: Copy>(
    shape: (usize, usize),
    array: &mut Vec<T>,
    brush: &Brush,
    pos: (usize, usize),
    value: T,
) {
    let profiler = Profiler::start("apply_brush");
    let (size_i, size_j) = shape;
    let (m, n) = pos;
    let m = m as i32;
    let n = n as i32;
    for (i, j) in brush.brush.iter() {
        let i = i + m;
        let j = j + n;
        if i < 0 {
            continue;
        }
        if j < 0 {
            continue;
        }
        if (size_i as i32) <= i {
            continue;
        }
        if (size_j as i32) <= j {
            continue;
        }
        let idx = k(i as usize, j as usize, size_j);
        array[idx] = value;
    }
    profiler.stop();
}

fn notched_square_brush(width: usize, notch: usize) -> Vec<(i32, i32)> {
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

fn _translate_brush(brush: &Vec<(i32, i32)>, m: i32, n: i32) -> Vec<(i32, i32)> {
    let m = m as i32;
    let n = n as i32;
    let mut new = Vec::new();
    for (i, j) in brush.iter() {
        new.push(((*i + m), (*j + n)));
    }
    return new;
}
