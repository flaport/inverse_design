use super::visualization::visualize_array;
use arrayfire::{add, assign_seq, constant, lt, pow, range, select, sub, Array, Dim4, Seq};

pub fn circular_brush(diameter: u64) -> Array<bool> {
    let radius = (diameter as f32) / 2.0;
    let small_radius = radius - 0.5;
    let x = range::<f32>(Dim4::new(&[diameter, diameter, 1, 1]), 0);
    let y = range::<f32>(Dim4::new(&[diameter, diameter, 1, 1]), 1);

    let rx2 = pow(&sub(&x, &small_radius, false), &2, false);
    let ry2 = pow(&sub(&y, &small_radius, false), &2, false);
    let r2 = add(&rx2, &ry2, false);

    let mask = lt(&r2, &(&radius * &radius), false);
    let dim4 = Dim4::new(&[diameter, diameter, 1, 1]);
    let ones = constant(1.0 as f32, dim4);
    let zeros = constant(0.0 as f32, dim4);
    let brush = select(&ones, &mask, &zeros);

    return brush.cast::<bool>();
}

pub fn notched_square_brush(width: u64, notch: u64) -> Array<bool> {
    let dim4 = Dim4::new(&[width, width, 1, 1]);
    let mut brush = constant(1.0 as f32, dim4);

    let seqs = [
        Seq::new(0.0, notch as f32 - 1.0, 1.0),
        Seq::new((width - notch) as f32, width as f32 - 1.0, 1.0),
    ];

    let cutout = constant(0.0 as f32, Dim4::new(&[notch, notch, 1, 1]));
    for seq1 in seqs {
        for seq2 in seqs {
            assign_seq(&mut brush, &[seq1, seq2], &cutout);
        }
    }

    return brush.cast::<bool>();
}

pub fn test_brushes() {
    //let brush = circular_brush(10);
    let brush = notched_square_brush(10, 2);
    visualize_array(&brush);
}
