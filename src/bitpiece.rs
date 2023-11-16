use crate::PieceError;
use crate::Color;
use crate::Piece;
use std::fmt;

// u8 has 8 bit
// first bit is color (immutable)
// next 3 bits is piece (immutable)
// next 4 bits are custom flags (mutable)

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BitPieceFlags {
    FirstMove = 1,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BitPiece(u8);

impl BitPiece {
    pub fn new(piece: Piece, color: Color, has_moved: bool) -> BitPiece {
        let mut value = 0;
        value |= color as u8;
        value |= (piece as u8) << 1;
        value |= (has_moved as u8) << 4;
        BitPiece(value)
    }

    pub fn new_blank() -> BitPiece {
        BitPiece(0)
    }

    pub fn is_blank(&self) -> bool {
        self.0 == 0
    }

    pub fn get_piece(&self) -> Piece {
        Piece::try_from((self.0 & 0b0000_1110) >> 1).unwrap()
    }

    pub fn get_color(&self) -> Color {
        if self.0 & 0b0000_0001 == 0 {
            Color::White
        } else {
            Color::Black
        }
    }

    // check if piece has ever moved
    pub fn has_moved(&self) -> bool {
        self.0 & 0b0001_0000 != 0
    }

    // Helper functions
    pub fn is_pawn(&self) -> bool {
        self.get_piece() == Piece::Pawn
    }

    pub fn is_knight(&self) -> bool {
        self.get_piece() == Piece::Knight
    }

    pub fn is_bishop(&self) -> bool {
        self.get_piece() == Piece::Bishop
    }

    pub fn is_rook(&self) -> bool {
        self.get_piece() == Piece::Rook
    }

    pub fn is_queen(&self) -> bool {
        self.get_piece() == Piece::Queen
    }

    pub fn is_king(&self) -> bool {
        self.get_piece() == Piece::King
    }

    pub fn is_white(&self) -> bool {
        self.get_color() == Color::White
    }

    pub fn is_black(&self) -> bool {
        self.get_color() == Color::Black
    }
}

impl TryFrom<u8> for BitPiece {
    type Error = PieceError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if let Err(e) = Piece::try_from((value & 0b0000_1110) >> 1) {
            return Err(e);
        }
        Ok(BitPiece(value))
    }
}

impl TryFrom<char> for BitPiece {
    type Error = PieceError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let piece = Piece::try_from(value)?;
        Ok(BitPiece::new(piece, Color::White, false))
    }
}

impl fmt::Display for BitPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece = self.get_piece();
        let color = self.get_color();
        let mut c = piece.to_string();
        if color == Color::White {
            c = c.to_ascii_lowercase();
        }
        write!(f, "{}", c)
    }
}
