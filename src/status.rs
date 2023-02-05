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
