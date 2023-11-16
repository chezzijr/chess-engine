use std::ops::{IndexMut, Index};
use crate::{BitPiece, Color, Square, RawPieceMove, movegen::Raw};

type BitBoard = [BitPiece; 64];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BoardStatus {
    // color is the color of the player who is in check
    Check(Color),
    // color is the color of the player who is in checkmate
    Checkmate(Color),
    Stalemate,
    Ongoing,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Board {
    board: BitBoard,
    pub turn: Color,
    pub status: BoardStatus,
    pub en_passant: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
}

impl Index<Square> for Board {
    type Output = BitPiece;

    fn index(&self, index: Square) -> &Self::Output {
        &self.board[usize::from(index)]
    }
}

impl IndexMut<Square> for Board {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self.board[usize::from(index)]
    }
}

impl Default for Board {
    fn default() -> Self {
        todo!()
    }
}

impl Board {
    pub fn force_execute_raw_move(&mut self, mv: RawPieceMove) {
    }

    pub fn is_being_checked(&self, color: Color, raw_moves: &Vec<RawPieceMove>) -> bool {
        // current turn meaning that opponent is not checked or checkmated
        // or previous status of the board is not check or checkmated
        // in other way, the fact that the board can advance to current turn
        // meaning that current turn is not checking or checkmating opponent
        // we can use opponent's raw moves instead of legal moves to determine
        // if current turn is being checked
        // if we chose legal moves, this function calls Legal::gen_legal_moves
        // Legal::gen_legal_moves call is_being_checked => indefinite recursion
        let moves = raw_moves.iter().filter(|&mov| mov.piece.get_color() != color);
        for mov in moves {
            let p = self[mov.to];
            if !p.is_blank() && p.is_king() && p.get_color() == color {
                return true;
            }
        }
        false
    }
}
