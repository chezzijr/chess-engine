use crate::{Board, RawPieceMove};

use super::Raw;
// Legal moves = raw moves + move does not result in check or checkmate
pub struct Legal;

impl Legal {
    pub fn gen_all_legal_moves(board: &Board) -> Vec<RawPieceMove> {
        let raw_moves = Raw::gen_all_raw_moves(board);
        Self::filter(board, &raw_moves)
    }
    pub fn filter(board: &Board, raw_moves: &Vec<RawPieceMove>) -> Vec<RawPieceMove> {
        raw_moves
            .iter()
            .filter_map(|&mv| {
                let mut b = board.clone();
                b.force_execute_raw_move(mv);
                if !b.is_being_checked(b.turn, raw_moves) {
                    Some(mv)
                } else {
                    None
                }
            })
            .collect()
    }
}
