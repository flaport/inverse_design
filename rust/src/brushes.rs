use super::array::new_array;

pub fn test_brushes() {
    let brush = Brush::notched_square(5, 1);
    let big_brush = compute_big_brush(&brush);

    brush.visualize();
    big_brush.visualize();
}

pub struct Brush {
    pub brush: Vec<(i32, i32)>,
    pub shape: (usize, usize),
}

impl Brush {
    pub fn notched_square(width: usize, notch: usize) -> Self {
        let brush = notched_square_brush(width, notch);
        let shape = (width, width);
        return Self { brush, shape };
    }

    pub fn from_f32_mask(shape: (usize, usize), mask: &Vec<f32>) -> Self {
        let (m, n) = shape;
        let mut brush = Vec::new();
        let m_ = m as i32 / 2;
        let n_ = n as i32 / 2;
        for i in 0..m {
            for j in 0..n {
                if mask[i * n + j] > 0.5 {
                    brush.push((i as i32 - m_, j as i32 - n_));
                }
            }
        }
        return Self { brush, shape };
    }

    pub fn mask(&self) -> Vec<bool> {
        let (size_i, size_j) = self.shape;
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
            let idx = (i as usize) * size_j + j as usize;
            mask[idx] = true;
        }
        return mask;
    }

    pub fn at(&self, pos: (usize, usize), shape: (usize, usize)) -> Vec<(usize, usize)> {
        let (m, n) = pos;
        let (size_i, size_j) = shape;

        let m = m as i32;
        let n = n as i32;
        let size_i = size_i as i32;
        let size_j = size_j as i32;

        let mut new = Vec::new();

        for (i, j) in self.brush.iter() {
            let i = *i + m;
            let j = *j + n;
            if i < 0 {
                continue;
            }
            if j < 0 {
                continue;
            }
            if size_i <= i {
                continue;
            }
            if size_j <= j {
                continue;
            }
            new.push((i as usize, j as usize));
        }
        return new;
    }
}

pub fn compute_big_brush(brush: &Brush) -> Brush {
    let (m, n) = brush.shape;
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
            if mask[i * n_ + j] {
                let i = (i as i32) - (m_ as i32) / 2;
                let j = (j as i32) - (n_ as i32) / 2;
                new_brush.push((i + 1, j + 1)); // yes, +1
            }
        }
    }
    return Brush {
        brush: new_brush,
        shape: (m_ - 1, n_ - 1),
    };
}

pub fn compute_very_big_square_brush(brush: &Brush) -> Brush {
    let (m, n) = brush.shape;
    let (m_, n_) = (3 * m, 3 * n);

    let mut new_brush = Vec::new();
    for i in 0..m_ {
        for j in 0..n_ {
            new_brush.push((i as i32 - m_ as i32 / 2, j as i32 - n_ as i32 / 2));
        }
    }

    let brush = Brush {
        brush: new_brush,
        shape: (m_, n_),
    };
    return brush;
}

pub fn compute_big_square_brush(brush: &Brush) -> Brush {
    let (m, n) = brush.shape;
    let (m_, n_) = (2 * m, 2 * n);

    let mut new_brush = Vec::new();
    for i in 0..m_ {
        for j in 0..n_ {
            new_brush.push((i as i32 - m as i32, j as i32 - m as i32));
        }
    }
    return Brush {
        brush: new_brush,
        shape: (m_, n_),
    };
}

pub fn apply_touch<T: Copy>(
    shape: (usize, usize),
    array: &mut Vec<T>,
    pos: (usize, usize),
    value: T,
) {
    let (_, size_j) = shape;
    let (m, n) = pos;
    let idx = m * size_j + n;
    array[idx] = value;
}

pub fn multi_apply_touch<T: Copy>(
    shape: (usize, usize),
    arrays: &mut Vec<&mut Vec<T>>,
    pos: (usize, usize),
    values: &Vec<T>,
) {
    let (_, size_j) = shape;
    let (m, n) = pos;
    let idx = m * size_j + n;
    for (array, value) in arrays.iter_mut().zip(values.into_iter()) {
        array[idx] = *value;
    }
}

pub fn apply_brush<T: Copy>(
    shape: (usize, usize),
    array: &mut Vec<T>,
    brush: &Brush,
    pos: (usize, usize),
    value: T,
) {
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
        let idx = (i as usize) * size_j + j as usize;
        array[idx] = value;
    }
}

pub fn multi_apply_brush<T: Copy>(
    shape: (usize, usize),
    arrays: &mut Vec<&mut Vec<T>>,
    brush: &Brush,
    pos: (usize, usize),
    values: &Vec<T>,
) {
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
        let idx = (i as usize) * size_j + j as usize;
        for (array, value) in arrays.iter_mut().zip(values.into_iter()) {
            array[idx] = *value;
        }
    }
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

pub fn subtract<T: Copy + Eq>(v1: &Vec<T>, v2: &Vec<T>) -> Vec<T> {
    let mut v = Vec::new();
    for x in v1.iter() {
        let mut found = false;
        for y in v2.iter() {
            if x == y {
                found = true;
                break;
            }
        }
        if !found {
            v.push(*x);
        }
    }
    return v;
}
