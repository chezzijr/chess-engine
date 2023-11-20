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
    fn make_move() {
        let mut board1 = Board::default();
        assert_eq!(board1.fen_notation(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        board1.make_move("e4".to_owned()).unwrap();
        assert_eq!(board1.fen_notation(), "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
        board1.make_move("c5".to_owned()).unwrap();
        assert_eq!(board1.fen_notation(), "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2");
        board1.make_move("Nf3".to_owned()).unwrap();
        assert_eq!(board1.fen_notation(), "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");

        let mut board = Board::from_fen("2p5/1P6/8/8/8/8/5k2/K7 w KQkq - 0 1".into()).unwrap();
        board.make_move("xc8=Q".into()).unwrap();
        assert_eq!(board.fen_notation(), "2Q5/8/8/8/8/8/5k2/K7 b KQkq - 0 1");
    }
}
