use std::fmt;

use crate::ColorError;

type ColorResult<T> = Result<T, ColorError>;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl TryFrom <u8> for Color {
    type Error = ColorError;

    fn try_from(value: u8) -> ColorResult<Color> {
        match value {
            0 => Ok(Color::White),
            1 => Ok(Color::Black),
            _ => Err(ColorError::InvalidValue(value)),
        }
    }
}

impl TryFrom<char> for Color {
    type Error = ColorError;

    fn try_from(value: char) -> ColorResult<Color> {
        match value {
            'w' => Ok(Color::White),
            'b' => Ok(Color::Black),
            _ => Err(ColorError::InvalidValue(value as u8)),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Color::White => 'w',
            Color::Black => 'b',
        };
        write!(f, "{}", c)
    }
}
