use super::visualization::visualize_design;
use arrayfire::{constant, eq, select, Array, Dim4};
use std::convert::From;

pub fn test_design() {
    let design = Design::new((10, 10));
    visualize_design(&design);
}

pub struct Design {
    pub void_pixels: Array<u8>,
    pub solid_pixels: Array<u8>,
    pub void_touches: Array<u8>,
    pub solid_touches: Array<u8>,
}

impl Design {
    pub fn new(shape: (u64, u64)) -> Self {
        let (m, n) = shape;
        let dim4 = Dim4::new(&[m, n, 1, 1]);
        Self {
            void_pixels: constant(Status::PixelPossible as u8, dim4),
            solid_pixels: constant(Status::PixelPossible as u8, dim4),
            void_touches: constant(Status::TouchValid as u8, dim4),
            solid_touches: constant(Status::TouchValid as u8, dim4),
        }
    }
    pub fn shape(&self) -> (u64, u64) {
        let dim4 = self.void_pixels.dims();
        let shape4 = dim4.get();
        let m = shape4[0];
        let n = shape4[1];
        return (m, n);
    }
    pub fn design(&self) -> Array<u8> {
        let dim4 = self.void_pixels.dims();
        let design = constant(Status::Unassigned as u8, dim4);
        let design = select(
            &constant(Status::Void as u8, dim4),
            &(eq(
                &self.void_pixels,
                &constant(Status::PixelExisting as u8, dim4),
                false,
            )),
            &design,
        );
        let design = select(
            &constant(Status::Solid as u8, dim4),
            &(eq(
                &self.solid_pixels,
                &constant(Status::PixelExisting as u8, dim4),
                false,
            )),
            &design,
        );
        return design;
    }

    pub fn design_mask(&self) -> Array<f32> {
        let dim4 = self.void_pixels.dims();
        let one = constant(1.0 as f32, dim4);
        let minus_one = constant(-1.0 as f32, dim4);
        let void = constant(Status::Void as u8, dim4);
        let mask = eq(&self.design(), &void, false);
        return select(&minus_one, &mask, &one);
    }
}

pub enum Status {
    Unassigned,
    Void,
    Solid,
    PixelImpossible,
    PixelExisting,
    PixelPossible,
    PixelRequired,
    TouchRequired,
    TouchInvalid,
    TouchExisting,
    TouchValid,
    TouchFree,
    TouchResolving,
    Unknown,
}

impl From<u8> for Status {
    fn from(n: u8) -> Self {
        match n {
            0 => Self::Unassigned,
            1 => Self::Void,
            2 => Self::Solid,
            3 => Self::PixelImpossible,
            4 => Self::PixelExisting,
            5 => Self::PixelPossible,
            6 => Self::PixelRequired,
            7 => Self::TouchRequired,
            8 => Self::TouchInvalid,
            9 => Self::TouchExisting,
            10 => Self::TouchValid,
            11 => Self::TouchFree,
            12 => Self::TouchResolving,
            _ => Self::Unknown,
        }
    }
}
