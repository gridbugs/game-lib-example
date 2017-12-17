#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    Return = 4,
}

impl Input {
    pub fn from_u8(u: u8) -> Option<Self> {
        use self::Input::*;
        let input = match u {
            0 => Up,
            1 => Down,
            2 => Left,
            3 => Right,
            4 => Return,
            _ => return None,
        };
        Some(input)
    }
}
