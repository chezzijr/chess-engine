use crate::SquareError;
use std::fmt;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Square(u8);

impl Square {
    pub fn rank(&self) -> u8 {
        self.0 / 8 + 1
    }

    pub fn file(&self) -> char {
        (b'a' + self.0 % 8) as char
    }

    // helper functions
    pub fn up(&self, offset: u8) -> Option<Square> {
        let new_square = self.0 + 8 * offset;
        if new_square > 63 {
            None
        } else {
            Some(Square(new_square))
        }
    }

    pub fn down(&self, offset: u8) -> Option<Square> {
        if 8 * offset > self.0 {
            return None
        }
        Some(Square(self.0 - 8 * offset))
    }

    pub fn left(&self, offset: u8) -> Option<Square> {
        if self.0 % 8 < offset {
            return None
        }
        Some(Square(self.0 - offset))
    }

    pub fn right(&self, offset: u8) -> Option<Square> {
        if self.0 % 8 + offset > 7 {
            return None
        }
        Some(Square(self.0 + offset))
    }
}

impl TryFrom<u8> for Square {
    type Error = SquareError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 63 {
            Err(SquareError::InvalidIndex(value as usize))
        } else {
            Ok(Square(value))
        }
    }
}

impl From<Square> for u8 {
    fn from(square: Square) -> Self {
        square.0
    }
}

impl From<Square> for usize {
    fn from(square: Square) -> Self {
        square.0 as usize
    }
}

impl TryFrom<String> for Square {
    type Error = SquareError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(SquareError::InvalidSquare(value));
        }
        let mut chars = value.chars();
        let file = chars.next().unwrap();
        let rank = chars.next().unwrap();

        if !file.is_ascii_alphabetic() {
            return Err(SquareError::InvalidFile(file));
        } else if !('a' <= file && file <= 'h') {
            return Err(SquareError::InvalidFile(file));
        }

        if !rank.is_ascii_digit() {
            return Err(SquareError::InvalidRank(rank));
        } else if !('1' <= rank && rank <= '8') {
            return Err(SquareError::InvalidRank(rank));
        }

        let file = file as u8 - 97;
        let rank = rank as u8 - 49;
        Ok(Square(file + rank * 8))
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}
