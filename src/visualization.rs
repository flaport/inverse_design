use super::design::{Design, Status};
use arrayfire::{assign_seq, constant, transpose, Array, Dim4, HasAfEnum, Seq};

pub fn visualize_array<T: HasAfEnum + Copy>(arr: &Array<T>) {
    let dims = *arr.dims().get();
    // let m = dims[0];
    let n = dims[1];

    // let mut arr = arr.copy().cast::<f32>();
    // let el    = constant(3.0 as f32, Dim4::new(&[1, 1, 1, 1]));
    // let seqs = &[Seq::new(0.0, 0.0, 1.0), Seq::new(1.0, 1.0, 1.0)];
    // assign_seq(&mut arr, seqs, &el);
    // af_print!("", &arr);

    let arr = transpose(&arr, false).cast::<u8>(); // visualization needs row-first memory layout
    let mut buffer = Vec::<u8>::new();
    buffer.resize(arr.elements(), 0_u8);
    arr.host(&mut buffer);

    let mut s = "".to_string();
    for (i, v) in buffer.into_iter().enumerate() {
        let block = Block::from_u8(v);
        let block_str = block.to_string();
        s = format!("{s}{block_str}");
        if (i % (n as usize)) == (n as usize - 1) {
            s = format!("{s}\n");
        }
    }
    println!("{s}");
}

pub fn visualize_design(design: &Design) {
    let (m, n) = design.shape();
    let refs = [
        &design.design(),
        &design.void_pixels,
        &design.solid_pixels,
        &design.void_touches,
        &design.solid_touches,
    ];

    let k = refs.len() as u64;
    let dim4 = Dim4::new(&[m, (n + 1) * k, 1, 1]);
    let mut vis = constant(Status::Unknown as u8, dim4);

    let n = n as f32;
    for (i, r) in refs.into_iter().enumerate() {
        let i = i as f32;
        assign_seq(
            &mut vis,
            &[
                Seq::default(),
                Seq::new(i * (n + 1.0), i * (n + 1.0) + n - 1.0, 1.0),
            ],
            r,
        );
    }

    visualize_array(&vis);
}

#[allow(dead_code)]
enum Block {
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
    Unknown,
}

impl Block {
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
            Self::Unknown => "  ",
        };
        return s.to_string();
    }
    pub fn from_u8(u: u8) -> Self {
        let block = match u {
            0 => Self::DarkWhite, //
            1 => Self::White, //
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
            _ => Self::Unknown,
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
        let block = Block::from_u8(i as u8);
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
