use super::brushes::notched_square_brush;
use super::utils::dilute;
use super::visualization::visualize_design;
use arrayfire::{and, assign_seq, constant, eq, or, select, Array, Dim4, Seq};
use std::convert::From;
use std::mem::swap;

pub fn test_design() {
    let brush = notched_square_brush(5, 1);

    println!("step 1");
    let mut design = Design::new((6, 8));
    visualize_design(&design);

    println!("step 2");
    design.add_void_touch(&brush, XYOrMask::XY((0, 6))); // todo: make more efficient?
    visualize_design(&design);

    println!("step 3");
    design.take_free_void_touches(&brush);
    visualize_design(&design);

    println!("step 4");
    design.add_solid_touch(&brush, XYOrMask::XY((0, 0)));
    visualize_design(&design);

    println!("step 5");
    design.add_void_touch(&brush, XYOrMask::XY((4, 6)));
    visualize_design(&design);

    println!("step 6");
    design.take_free_void_touches(&brush);
    visualize_design(&design);

    println!("step 7");
    design.add_void_touch(&brush, XYOrMask::XY((4, 4)));
    visualize_design(&design);

    println!("step 8");
    design.take_free_void_touches(&brush);
    visualize_design(&design);

    println!("step 9");
    design.add_void_touch(&brush, XYOrMask::XY((5, 0)));
    visualize_design(&design);

    println!("step 10");
    design.take_free_void_touches(&brush);
    visualize_design(&design);

    println!("step 11");
    design.add_void_touch(&brush, XYOrMask::XY((2, 5)));
    visualize_design(&design);

    println!("step 12");
    design.take_free_void_touches(&brush);
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

    pub fn invert(&mut self) {
        swap(&mut self.solid_pixels, &mut self.void_pixels);
        swap(&mut self.solid_touches, &mut self.void_touches);
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

    pub fn add_solid_touch(&mut self, brush: &Array<bool>, xy_or_mask: XYOrMask) {
        self.invert();
        self.add_void_touch(brush, xy_or_mask);
        self.invert();
    }

    pub fn add_void_touch(&mut self, brush: &Array<bool>, xy_or_mask: XYOrMask) {
        let dim4 = self.void_touches.dims();

        let mut void_touch_existing = eq(
            &self.void_touches,
            &constant(Status::TouchExisting as u8, dim4),
            false,
        );

        match xy_or_mask {
            XYOrMask::XY((m, n)) => {
                assign_seq(
                    &mut void_touch_existing,
                    &[
                        Seq::new(m as f32, m as f32, 1.0),
                        Seq::new(n as f32, n as f32, 1.0),
                    ],
                    &constant(true, Dim4::new(&[1, 1, 1, 1])),
                );
            }
            XYOrMask::Mask(mask) => {
                void_touch_existing = or(&mask, &void_touch_existing, false);
            }
        }

        let void_pixel_existing = or(
            &dilute(&void_touch_existing, &brush),
            &eq(
                &self.void_pixels,
                &constant(Status::PixelExisting as u8, dim4),
                false,
            ),
            false,
        );

        let void_pixel_required = find_required_pixels(&void_pixel_existing, &brush);
        let solid_touch_invalid = dilute(&void_pixel_existing, &brush);
        let void_touch_free = find_free_touches(
            &void_touch_existing,
            &or(&void_pixel_existing, &void_pixel_required, false),
            &brush,
        );
        let mut void_touches = select(
            &constant(Status::TouchValid as u8, dim4),
            &eq(
                &self.void_touches,
                &constant(Status::TouchResolving as u8, dim4),
                false,
            ),
            &self.void_touches,
        );
        void_touches = select(
            &constant(Status::TouchExisting as u8, dim4),
            &void_touch_existing,
            &void_touches,
        );
        void_touches = select(
            &constant(Status::TouchFree as u8, dim4),
            &void_touch_free,
            &void_touches,
        );

        let void_touch_resolving = select(
            &dilute(&void_pixel_required, &brush),
            &eq(
                &void_touches,
                &constant(Status::TouchValid as u8, dim4),
                false,
            ),
            &constant(false, dim4),
        );

        void_touches = select(
            &constant(Status::TouchResolving as u8, dim4),
            &void_touch_resolving,
            &void_touches,
        );

        let mut void_pixels = select(
            &constant(Status::PixelExisting as u8, dim4),
            &void_pixel_existing,
            &self.void_pixels,
        );
        void_pixels = select(
            &constant(Status::PixelRequired as u8, dim4),
            &void_pixel_required,
            &void_pixels,
        );

        let mut solid_pixels = select(
            &constant(Status::PixelImpossible as u8, dim4),
            &void_pixel_existing,
            &self.solid_pixels,
        );
        solid_pixels = select(
            &constant(Status::PixelImpossible as u8, dim4),
            &void_pixel_required,
            &solid_pixels,
        );

        let solid_touches = select(
            &constant(Status::TouchInvalid as u8, dim4),
            &solid_touch_invalid,
            &self.solid_touches,
        );

        self.void_pixels = void_pixels;
        self.solid_pixels = solid_pixels;
        self.void_touches = void_touches;
        self.solid_touches = solid_touches;
    }

    pub fn take_free_void_touches(&mut self, brush: &Array<bool>) {
        let dim4 = self.void_touches.dims();
        let free_touches_mask = eq(
            &self.void_touches,
            &constant(Status::TouchFree as u8, dim4),
            false,
        );
        self.add_void_touch(brush, XYOrMask::Mask(&free_touches_mask));
    }

    pub fn take_free_solid_touches(&mut self, brush: &Array<bool>) {
        let dim4 = self.void_touches.dims();
        let free_touches_mask = eq(
            &self.solid_touches,
            &constant(Status::TouchFree as u8, dim4),
            false,
        );
        self.add_solid_touch(brush, XYOrMask::Mask(&free_touches_mask));
    }
}

fn find_required_pixels(pixel_map: &Array<bool>, brush: &Array<bool>) -> Array<bool> {
    let mask = and(
        &not(pixel_map, false),
        &not(&dilute(pixel_map, brush), false),
        false,
    );
    return not(&or(&dilute(&mask, brush), pixel_map, false), false);
}

fn find_free_touches(
    touches_mask: &Array<bool>,
    pixels_mask: &Array<bool>,
    brush: &Array<bool>,
) -> Array<bool> {
    let brush_u64 = brush.cast::<u64>();
    let sum_mask = dilute(&pixels_mask.cast::<u64>(), &brush_u64);
    let ref_mask = dilute(&constant(1 as u64, sum_mask.dims()), &brush_u64);
    let mut free_mask = eq(&sum_mask, &ref_mask, false);
    free_mask = and(&free_mask, &not(&touches_mask, false), false);
    return free_mask;
}

fn not(arr: &Array<bool>, batch: bool) -> Array<bool> {
    let dim4 = arr.dims();
    let false_ = constant(false, dim4);
    return eq(arr, &false_, batch);
}

pub enum Status {
    /*  0 */ Unassigned,
    /*  1 */ Void,
    /*  2 */ Solid,
    /*  3 */ PixelImpossible,
    /*  4 */ PixelExisting,
    /*  5 */ PixelPossible,
    /*  6 */ PixelRequired,
    /*  7 */ TouchRequired,
    /*  8 */ TouchInvalid,
    /*  9 */ TouchExisting,
    /* 10 */ TouchValid,
    /* 11 */ TouchFree,
    /* 12 */ TouchResolving,
    /* 13 */ Unknown,
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

impl From<Status> for String {
    fn from(status: Status) -> Self {
        match status {
            Status::Unassigned => " U".to_string(),
            Status::Void => " V".to_string(),
            Status::Solid => " S".to_string(),
            Status::PixelImpossible => "PI".to_string(),
            Status::PixelExisting => "PE".to_string(),
            Status::PixelPossible => "PP".to_string(),
            Status::PixelRequired => "PR".to_string(),
            Status::TouchRequired => "TR".to_string(),
            Status::TouchInvalid => "TI".to_string(),
            Status::TouchExisting => "TE".to_string(),
            Status::TouchValid => "TV".to_string(),
            Status::TouchFree => "TF".to_string(),
            Status::TouchResolving => "Tr".to_string(),
            Status::Unknown => "  ".to_string(),
        }
    }
}

pub enum XYOrMask<'a> {
    XY((u64, u64)),
    Mask(&'a Array<bool>),
}
