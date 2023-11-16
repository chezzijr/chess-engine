use std::ops::{IndexMut, Index};
use crate::{BitPiece, Color, Square, movegen::{Raw, RawMove}};

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
    pub fn force_execute_raw_move(&mut self, mv: RawMove) {
        match mv {
            RawMove::Single(info) => {
                self[info.from] = BitPiece::new_blank();
                self[info.to] = info.piece;
                if let Some(capture) = info.capture {
                    self[capture.square] = BitPiece::new_blank();
                }
                if let Some(en_passant_square) = info.en_passant_square {
                    self.en_passant = Some(en_passant_square);
                }
            },
            RawMove::Double(info1, info2) => {
                self[info1.from] = BitPiece::new_blank();
                self[info1.to] = info1.piece;
                self[info2.from] = BitPiece::new_blank();
                self[info2.to] = info2.piece;
            }
        }
    }

    pub fn is_being_checked(&self, color: Color, raw_moves: &Vec<RawMove>) -> bool {
        // current turn meaning that opponent is not checked or checkmated
        // or previous status of the board is not check or checkmated
        // in other way, the fact that the board can advance to current turn
        // meaning that current turn is not checking or checkmating opponent
        // we can use opponent's raw moves instead of legal moves to determine
        // if current turn is being checked
        // if we chose legal moves, this function calls Legal::gen_legal_moves
        // Legal::gen_legal_moves call is_being_checked => indefinite recursion

        // we check if destination of a move can result in current 
        // color king square => capture king
        // being said, castle move cannot result in king capture
        let dest = raw_moves.iter().filter_map(|&mov| {
            match mov {
                RawMove::Single(info) => {
                    if info.piece.get_color() != color {
                        Some(info.to)
                    } else {
                        None
                    }
                },
                _ => None
            }
        });
        for mov in dest {
            let p = self[mov];
            if !p.is_blank() && p.is_king() && p.get_color() == color {
                return true;
            }
        }
        false
    }
}
