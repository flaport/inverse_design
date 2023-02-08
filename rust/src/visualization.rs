use super::brushes::Brush;
use super::debug::Profiler;
use super::design::Design;
use super::status::Status;
use itertools::izip;

pub fn visualize_u8_array(shape: (usize, usize), array: &Vec<u8>) {
    visualize_array(shape, array, |n| Color::from_u8(n));
}

pub fn visualize_f32_array(shape: (usize, usize), array: &Vec<f32>) {
    let min = array.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let max = array.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let array_u8: Vec<u8> = array
        .iter()
        .map(|x| (3.0 * (x - min) / (max - min)) as u8)
        .collect();
    visualize_array(shape, &array_u8, |n| match n {
        0 => Color::White,
        1 => Color::DarkWhite,
        _ => Color::BrightBlack,
    });
}

fn visualize_array<T: Copy, F: Fn(T) -> Color>(shape: (usize, usize), array: &Vec<T>, fun: F) {
    let (_, n) = shape;
    let mut s = "".to_string();
    for (i, elem) in array.iter().enumerate() {
        let block = fun(*elem).to_string();
        s = format!("{s}{block}");
        if i % n == n - 1 {
            s = format!("{s}\n");
        }
    }
    println!("{s}");
}

fn visualize_arrays<T: Copy, F: Fn(T) -> Color>(
    shape: (usize, usize),
    arrays: &Vec<&Vec<T>>,
    empty: T,
    fun: F,
) {
    let (m, n) = shape;
    let l = arrays.len();
    let n_ = l * (n + 1);
    let mut full = Vec::new(); //new_array(m*n_, true);
    for i in 0..m {
        for j_ in 0..n_ {
            let p = j_ / (n + 1);
            let j = j_ % (n + 1);
            let b = if j < n { arrays[p][i * n + j] } else { empty };
            full.push(b);
        }
    }
    visualize_array((m, n_), &full, fun);
}

impl Brush {
    pub fn visualize(&self) {
        let mask = self.mask();
        visualize_array(self.shape, &mask, &|b| match b {
            true => Color::White,
            false => Color::BrightBlack,
        });
    }
}

impl Design {
    pub fn visualize(&self) {
        let design_view = self.design_view();
        let void_pixel_view = self.void_pixel_view();
        let solid_pixel_view = self.solid_pixel_view();
        let void_touches_view = self.void_touches_view();
        let solid_touches_view = self.solid_touches_view();
        visualize_arrays(
            self.shape,
            &vec![
                &design_view,
                &void_pixel_view,
                &solid_pixel_view,
                &void_touches_view,
                &solid_touches_view,
            ],
            Status::Unknown,
            &|s| Color::from_status(&s),
        );
    }
}

#[allow(dead_code)]
enum Color {
    Black,
    DarkRed,
    DarkGreen,
    DarkYellow,
    DarkBlue,
    DarkMagenta,
    DarkCyan,
    DarkWhite,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    White,
    Transparent,
}

impl Color {
    pub fn to_string(&self) -> String {
        let s = match self {
            Self::Black => "\x1b[0;40m█\x1b[0m\x1b[0;40m█\x1b[0m",
            Self::DarkRed => "\x1b[0;31m█\x1b[0m\x1b[0;31m█\x1b[0m",
            Self::DarkGreen => "\x1b[0;32m█\x1b[0m\x1b[0;32m█\x1b[0m",
            Self::DarkYellow => "\x1b[0;33m█\x1b[0m\x1b[0;33m█\x1b[0m",
            Self::DarkBlue => "\x1b[0;34m█\x1b[0m\x1b[0;34m█\x1b[0m",
            Self::DarkMagenta => "\x1b[0;35m█\x1b[0m\x1b[0;35m█\x1b[0m",
            Self::DarkCyan => "\x1b[0;36m█\x1b[0m\x1b[0;36m█\x1b[0m",
            Self::DarkWhite => "\x1b[0;37m█\x1b[0m\x1b[0;37m█\x1b[0m",
            Self::BrightBlack => "\x1b[0;90m█\x1b[0m\x1b[0;90m█\x1b[0m",
            Self::BrightRed => "\x1b[0;91m█\x1b[0m\x1b[0;91m█\x1b[0m",
            Self::BrightGreen => "\x1b[0;92m█\x1b[0m\x1b[0;92m█\x1b[0m",
            Self::BrightYellow => "\x1b[0;93m█\x1b[0m\x1b[0;93m█\x1b[0m",
            Self::BrightBlue => "\x1b[0;94m█\x1b[0m\x1b[0;94m█\x1b[0m",
            Self::BrightMagenta => "\x1b[0;95m█\x1b[0m\x1b[0;95m█\x1b[0m",
            Self::BrightCyan => "\x1b[0;96m█\x1b[0m\x1b[0;96m█\x1b[0m",
            Self::White => "\x1b[0;97m█\x1b[0m\x1b[0;97m█\x1b[0m",
            Self::Transparent => "  ",
        };
        return s.to_string();
    }
    pub fn from_u8(u: u8) -> Self {
        let block = match u {
            0 => Self::DarkWhite,   //
            1 => Self::White,       //
            2 => Self::BrightBlack, //
            3 => Self::BrightCyan,
            4 => Self::BrightYellow,
            5 => Self::DarkMagenta,
            6 => Self::DarkRed,
            7 => Self::BrightBlue,
            8 => Self::DarkBlue,
            9 => Self::DarkYellow,
            10 => Self::BrightGreen,
            11 => Self::BrightMagenta,
            12 => Self::Black,
            // 13 => Self::DarkGreen,
            // 14 => Self::BrightRed,
            // 15 => Self::DarkCyan,
            _ => Self::Transparent,
        };
        return block;
    }
    pub fn from_status(status: &Status) -> Self {
        let block = match status {
            Status::Unassigned => Self::DarkWhite, //
            Status::Void => Self::White,           //
            Status::Solid => Self::BrightBlack,    //
            Status::PixelImpossible => Self::BrightCyan,
            Status::PixelExisting => Self::BrightYellow,
            Status::PixelPossible => Self::DarkMagenta,
            Status::PixelRequired => Self::DarkRed,
            Status::TouchRequired => Self::BrightBlue,
            Status::TouchInvalid => Self::DarkBlue,
            Status::TouchExisting => Self::DarkYellow,
            Status::TouchValid => Self::BrightGreen,
            Status::TouchFree => Self::BrightMagenta,
            Status::TouchResolving => Self::Black,
            // _ => Self::DarkGreen,
            // _ => Self::BrightRed,
            // _ => Self::DarkCyan,
            _ => Self::Transparent,
        };
        return block;
    }
}

pub fn test_visualization() {
    let mut s = "".to_string();
    for i in 0..14 {
        let space = if i < 10 { "  " } else { " " };
        s = format!("{s}{space}{i}");
    }
    println!("{s}");

    let mut s = "".to_string();
    for i in 0..14 {
        let block = Color::from_u8(i as u8);
        let block_str = block.to_string();
        s = format!("{s} {block_str}");
    }
    println!("{s}");

    let mut s = "".to_string();
    for i in 0..14 {
        let status: Status = (i as u8).into();
        let status_str: String = status.into();
        s = format!("{s} {status_str}");
    }
    println!("{s}");
}

impl Design {
    pub fn design_view(&self) -> Vec<Status> {
        let profiler = Profiler::start("design_view");
        let result = self
            .void
            .iter()
            .zip(self.solid.iter())
            .map(|(v, s)| {
                if *v {
                    Status::Void
                } else if *s {
                    Status::Solid
                } else {
                    Status::Unassigned
                }
            })
            .collect();
        profiler.stop();
        return result;
    }

    pub fn void_pixel_view(&self) -> Vec<Status> {
        let profiler = Profiler::start("void_pixel_view");
        let result = izip!(
            self.void_pixel_impossible.iter(),
            self.void_pixel_existing.iter(),
            self.void_pixel_required.iter(),
        )
        .map(|(i, e, r)| {
            if *r {
                Status::PixelRequired
            } else if *e {
                Status::PixelExisting
            } else if *i {
                Status::PixelImpossible
            } else {
                Status::PixelPossible
            }
        })
        .collect();
        profiler.stop();
        return result;
    }

    pub fn solid_pixel_view(&self) -> Vec<Status> {
        let profiler = Profiler::start("solid_pixel_view");
        let result = izip!(
            self.solid_pixel_impossible.iter(),
            self.solid_pixel_existing.iter(),
            self.solid_pixel_required.iter(),
        )
        .map(|(i, e, r)| {
            if *r {
                Status::PixelRequired
            } else if *e {
                Status::PixelExisting
            } else if *i {
                Status::PixelImpossible
            } else {
                Status::PixelPossible
            }
        })
        .collect();
        profiler.stop();
        return result;
    }

    pub fn void_touches_view(&self) -> Vec<Status> {
        let profiler = Profiler::start("void_touches_view");
        let result = izip!(
            self.void_touch_required.iter(),
            self.void_touch_invalid.iter(),
            self.void_touch_existing.iter(),
        )
        .map(|(r, i, e)| {
            if *e {
                Status::TouchExisting
            } else if *r {
                Status::TouchRequired
            } else if *i {
                Status::TouchInvalid
            } else {
                Status::TouchValid
            }
        })
        .collect();
        profiler.stop();
        return result;
    }

    pub fn solid_touches_view(&self) -> Vec<Status> {
        let profiler = Profiler::start("solid_touches_view");
        let result = izip!(
            self.solid_touch_required.iter(),
            self.solid_touch_invalid.iter(),
            self.solid_touch_existing.iter(),
        )
        .map(|(r, i, e)| {
            if *e {
                Status::TouchExisting
            } else if *r {
                Status::TouchRequired
            } else if *i {
                Status::TouchInvalid
            } else {
                Status::TouchValid
            }
        })
        .collect();
        profiler.stop();
        return result;
    }
}
