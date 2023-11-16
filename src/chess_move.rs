// this file contains the definition of a chess move
// which will be stored in history of the match
use crate::BitPiece;
use crate::Square;
#[derive(Debug, Copy, Clone, PartialEq)]

pub enum CastleMove {
    KingSide,
    QueenSide,
}

// this contains extra information about a move
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MoveInfo {
    pub piece: BitPiece,
    pub from: Square,
    pub to: Square,
    // does this move capture a piece?
    pub capture: Option<BitPiece>,
    // does this move promote a pawn?
    pub promotion: Option<BitPiece>,
    // is this a castle move?
    pub castle: Option<CastleMove>,
    // is this an en passant move?
    pub en_passant: bool,
    // does this move create an en passant square?
    pub en_passant_square: Option<Square>,
    // does this move check the opponent's king?
    pub check: bool,
    // does this move checkmate the opponent's king?
    pub checkmate: bool,
}
