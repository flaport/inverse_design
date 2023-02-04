use arrayfire::{af_print, add, lt, pow, range, select, sub, Array, Dim4};

pub fn circular_brush(diameter: u64) -> Array<f32> {
    let radius = (diameter as f32) / 2.0;
    let small_radius = radius - 0.5;
    let x = range::<f32>(Dim4::new(&[diameter, diameter, 1, 1]), 0);
    let y = range::<f32>(Dim4::new(&[diameter, diameter, 1, 1]), 1);

    let rx2 = pow(&sub(&x, &small_radius, false), &2, false);
    let ry2 = pow(&sub(&y, &small_radius, false), &2, false);
    let r2 = add(&rx2, &ry2, false);

    let mask = lt(&r2, &(&radius * &radius), false);
    let zeros: Array<f32> = Array::new_empty(Dim4::new(&[diameter, diameter, 1, 1]));
    let ones = add(&zeros, &1, false);
    let brush = select(&ones, &mask, &zeros);

    return brush;
}


pub fn test_brushes() {
    let brush = circular_brush(10);
    af_print!("", brush);
}
