// this file contains the definition of a chess move
// which will be stored in history of the match

use crate::BitPiece;
use crate::Board;
use crate::Square;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CastleMove {
    KingSide,
    QueenSide,
}

// this contains basic information about a move, does not include extra information
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RawPieceMove {
    pub piece: BitPiece,
    pub from: Square,
    pub to: Square,
}

impl RawPieceMove {
    pub fn is_capture(&self, board: &Board) -> bool {
        let p = board[self.to];
        !p.is_blank() && p.get_color() != self.piece.get_color()
    }

    pub fn is_castle(&self, board: &Board) -> Option<CastleMove> {
        todo!()
    }
}

// this contains extra information about a move
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MoveInfo {
    // does this move capture a piece?
    pub capture: Option<BitPiece>,
    // does this move promote a pawn?
    pub promotion: Option<BitPiece>,
    // is this a castle move?
    pub castle: Option<CastleMove>,
    // if a castle move, what is the rook's move?
    pub side_effect_move: Option<RawPieceMove>,
    // is this an en passant move?
    pub en_passant: bool,
    // does this move create an en passant square?
    pub en_passant_square: Option<Square>,
    // does this move check the opponent's king?
    pub check: bool,
    // does this move checkmate the opponent's king?
    pub checkmate: bool,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Move {
    raw_move: RawPieceMove,
    info: MoveInfo,
}
