use std::fmt;
use crate::PieceError;

type PieceResult<T> = Result<T, PieceError>;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Piece {
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6,
}

pub const PROMOTION_PIECES: [Piece; 4] = [
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
];

impl TryFrom<u8> for Piece {
    type Error = PieceError;

    fn try_from(value: u8) -> PieceResult<Self> {
        match value {
            1 => Ok(Piece::Pawn),
            2 => Ok(Piece::Knight),
            3 => Ok(Piece::Bishop),
            4 => Ok(Piece::Rook),
            5 => Ok(Piece::Queen),
            6 => Ok(Piece::King),
            _ => Err(PieceError::InvalidValue(value)),
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = PieceError;

    fn try_from(value: char) -> PieceResult<Self> {
        match value.to_ascii_uppercase() {
            'P' => Ok(Piece::Pawn),
            'N' => Ok(Piece::Knight),
            'B' => Ok(Piece::Bishop),
            'R' => Ok(Piece::Rook),
            'Q' => Ok(Piece::Queen),
            'K' => Ok(Piece::King),
            _ => Err(PieceError::InvalidChar(value)),
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Piece::Pawn => 'p',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Rook => 'r',
            Piece::Queen => 'q',
            Piece::King => 'k',
        };
        write!(f, "{}", c)
    }
}
