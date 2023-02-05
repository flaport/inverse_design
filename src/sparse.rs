use std::collections::HashMap;

pub fn new_sparse<T: Copy>(m: usize, n: usize, _default: T) -> HashMap<Point, T> {
    let mut sparse = HashMap::new();
    for i in 0..m {
        for j in 0..n {
            sparse.insert(Point::new(i, j), _default);
        }
    }
    return sparse;
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
