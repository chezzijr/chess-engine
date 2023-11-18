mod piece;
pub use crate::piece::*;

mod board;
pub use crate::board::*;

mod game;
pub use crate::game::*;

mod error;
pub use crate::error::*;

mod color;
pub use crate::color::*;

mod bitpiece;
pub use crate::bitpiece::*;

mod square;
pub use crate::square::*;

mod chess_move;
pub use crate::chess_move::*;

mod movegen;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fen() {
        let board = Board::default();
        assert_eq!(board.fen_notation(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }
}
