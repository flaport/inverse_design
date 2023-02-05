use arrayfire::{
    af_print, any_true, constant, convolve2, eq, flat, imax, imin, randn as _randn, randu as _rand,
    set_seed, sum as _sum, Array, ConvDomain, ConvMode, Dim4, FloatingPoint, HasAfEnum,
};

pub fn conv2d<T: HasAfEnum>(lhs: &Array<T>, rhs: &Array<T>) -> Array<T> {
    convolve2(lhs, rhs, ConvMode::DEFAULT, ConvDomain::FREQUENCY)
}

pub fn batch_conv2d<T: HasAfEnum>(lhs: &Array<T>, rhs: &Array<T>) -> Array<T> {
    convolve2(lhs, rhs, ConvMode::DEFAULT, ConvDomain::FREQUENCY)
}

pub fn dilute<T: HasAfEnum>(touches: &Array<T>, brush: &Array<T>) -> Array<T> {
    convolve2(touches, brush, ConvMode::DEFAULT, ConvDomain::FREQUENCY)
}

pub fn argmax2d<T: HasAfEnum + Clone>(arr: &Array<T>) -> (u64, u64) {
    let (m, _) = _size2d(&arr);
    let arr_flat = flat(&arr);
    let (_, ks) = imax(&arr_flat, 0);
    let mut buffer = Vec::<u32>::new();
    buffer.resize(ks.elements(), 0_u32);
    ks.host(&mut buffer);
    let val = *buffer.iter().next().unwrap() as u64;
    return (val % m, val / m);
}

pub fn argmin2d<T: HasAfEnum + Clone>(arr: &Array<T>) -> (u64, u64) {
    let (m, _) = _size2d(&arr);
    let arr_flat = flat(&arr);
    let (_, ks) = imin(&arr_flat, 0);
    let mut buffer = Vec::<u32>::new();
    buffer.resize(ks.elements(), 0_u32);
    ks.host(&mut buffer);
    let val = *buffer.iter().next().unwrap() as u64;
    return (val % m, val / m);
}

pub fn item(arr: &Array<f32>) -> f32 {
    let mut buffer: Vec<f32> = Vec::new();
    buffer.resize(arr.elements(), 0.0);
    arr.host(&mut buffer);
    return *buffer.iter().next().unwrap();
}

pub fn sum(arr: &Array<f32>) -> f32 {
    return item(&_sum(&flat(&arr), 0));
}

pub fn any(arr: &Array<bool>) -> bool {
    let result = any_true(&flat(&arr), 0);
    let mut buffer: Vec<bool> = Vec::new();
    buffer.resize(result.elements(), false);
    result.host(&mut buffer);
    return *buffer.iter().next().unwrap();
}

pub fn not(arr: &Array<bool>, batch: bool) -> Array<bool> {
    let dim4 = arr.dims();
    let false_ = constant(false, dim4);
    return eq(arr, &false_, batch);
}

pub fn all(arr: &Array<bool>) -> bool {
    return any(&not(&arr, false));
}

pub fn randn<T: FloatingPoint>(shape: (u64, u64)) -> Array<T> {
    let (m, n) = shape;
    let data = _randn::<T>(Dim4::new(&[m, n, 1, 1]));
    return data;
}

pub fn rand<T: HasAfEnum>(shape: (u64, u64)) -> Array<T> {
    let (m, n) = shape;
    let data = _rand::<T>(Dim4::new(&[m, n, 1, 1]));
    return data;
}

fn _size2d<T: HasAfEnum>(arr: &Array<T>) -> (u64, u64) {
    let dim4 = arr.dims();
    let vec = dim4.get();
    let m = vec[0];
    let n = vec[1];
    return (m, n);
}

pub fn test_utils() {
    set_seed(420);
    let data = randn::<f32>((2, 3));
    let idx_max = argmax2d(&data);
    let idx_min = argmin2d(&data);
    af_print!("Create a 5-by-3 matrix of random floats on the GPU", data);
    println!("{idx_min:?} {idx_max:?}")
}
