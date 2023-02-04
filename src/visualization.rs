pub fn visualize_brush(){
    let block = Block::DarkRed;
    println!("{}", block.to_string());
}

enum Block{
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
}

impl Block {
    pub fn to_string(&self) -> String {
        let s = match self {
             Block::Black => "\x1b[0;40m█\x1b[0m\x1b[0;40m█\x1b[0m",
             Block::DarkRed => "\x1b[0;31m█\x1b[0m\x1b[0;31m█\x1b[0m",
             Block::DarkGreen => "\x1b[0;32m█\x1b[0m\x1b[0;32m█\x1b[0m",
             Block::DarkYellow => "\x1b[0;33m█\x1b[0m\x1b[0;33m█\x1b[0m",
             Block::DarkBlue => "\x1b[0;34m█\x1b[0m\x1b[0;34m█\x1b[0m",
             Block::DarkMagenta => "\x1b[0;35m█\x1b[0m\x1b[0;35m█\x1b[0m",
             Block::DarkCyan => "\x1b[0;36m█\x1b[0m\x1b[0;36m█\x1b[0m",
             Block::DarkWhite => "\x1b[0;37m█\x1b[0m\x1b[0;37m█\x1b[0m",
             Block::BrightBlack => "\x1b[0;90m█\x1b[0m\x1b[0;90m█\x1b[0m",
             Block::BrightRed => "\x1b[0;91m█\x1b[0m\x1b[0;91m█\x1b[0m",
             Block::BrightGreen => "\x1b[0;92m█\x1b[0m\x1b[0;92m█\x1b[0m",
             Block::BrightYellow => "\x1b[0;93m█\x1b[0m\x1b[0;93m█\x1b[0m",
             Block::BrightBlue => "\x1b[0;94m█\x1b[0m\x1b[0;94m█\x1b[0m",
             Block::BrightMagenta => "\x1b[0;95m█\x1b[0m\x1b[0;95m█\x1b[0m",
             Block::BrightCyan => "\x1b[0;96m█\x1b[0m\x1b[0;96m█\x1b[0m",
             Block::White => "\x1b[0;97m█\x1b[0m\x1b[0;97m█\x1b[0m",
        };
        return s.to_string();
    }
    pub fn from_u8(u: u8) -> Self {
        let block = match u {
             0 => Block::Black ,
             1 => Block::DarkRed ,
             2 => Block::DarkGreen,
             3 => Block::DarkYellow,
             4 => Block::DarkBlue,
             5 => Block::DarkMagenta,
             6 => Block::DarkCyan,
             7 => Block::DarkWhite,
             8 => Block::BrightBlack,
             9 => Block::BrightRed,
             10 => Block::BrightGreen,
             11 => Block::BrightYellow,
             12 => Block::BrightBlue,
             13 => Block::BrightMagenta,
             14 => Block::BrightCyan,
             15 => Block::White,
             _ => panic!("This should not happen!"),
        };
        return block;
    }
}

pub fn test_visualization() {
    let mut s = "".to_string();
    for i in 0..16{
        let block = Block::from_u8(i as u8);
        let block_str = block.to_string();
        s = format!("{s}{block_str}");
    }
    println!("{s}");
}
