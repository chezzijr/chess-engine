use crate::Board;

use super::{Raw, RawMove};
// Legal moves = raw moves + move does not result in check or checkmate
pub(crate) struct Legal;

impl Legal {
    pub(crate) fn gen_all_legal_moves(board: &Board) -> Vec<RawMove> {
        let raw_moves = Raw::gen_all_raw_moves(board);
        Self::filter(board, &raw_moves)
    }

    pub(crate) fn filter(board: &Board, raw_moves: &Vec<RawMove>) -> Vec<RawMove> {
        raw_moves
            .iter()
            .filter_map(|&mv| {
                let mut b = board.clone();
                b.force_execute_raw_move(mv);
                if !b.is_being_checked(board.turn, &Raw::gen_all_raw_moves(&b)) {
                    Some(mv)
                } else {
                    None
                }
            })
            .collect()
    }
}
