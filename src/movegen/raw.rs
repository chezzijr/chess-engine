use super::{Walk, DD, HD, VD};
use crate::{Board, Piece, RawPieceMove, Square};
// How to generate legal moves:
// 1. Generate all possible raw moves
// 2. Filter out illegal moves (e.g. moves that leave the king in check)
// To filter, we execute raw move on temporary board and check if king is in check
pub struct Raw;

impl Raw {
    pub fn gen_all_raw_moves(board: &Board) -> Vec<RawPieceMove> {
        let mut moves = vec![];
        let gen_fns = [
            Self::gen_pawn_raw_moves,
            Self::gen_rook_raw_moves,
            Self::gen_bishop_raw_moves,
            Self::gen_queen_raw_moves,
            Self::gen_king_raw_moves,
            Self::gen_knight_raw_moves,
        ];
        for i in 0..64 {
            let square = Square::try_from(i).unwrap();
            let p = board[square];
            if !p.is_blank() {
                moves.extend(gen_fns[(p.get_piece() as usize) - 1](board, square));
            }
        }
        moves
    }

    pub fn gen_pawn_raw_moves(board: &Board, square: Square) -> Vec<RawPieceMove> {
        let mut moves = vec![];
        let piece = board[square];
        if piece.is_blank() {
            return moves;
        }
        // vertical, noncapturing moves
        let dir = if piece.is_white() { VD::Up } else { VD::Down };
        let max_offset = if !piece.has_moved() { 2 } else { 1 };
        let n_squares = Walk::vertical(board, square, dir, max_offset);
        moves.extend(n_squares.iter().filter_map(|&sqr| {
            let piece = board[sqr];
            if piece.is_blank() {
                Some(RawPieceMove {
                    piece,
                    from: square,
                    to: sqr,
                })
            } else {
                None
            }
        }));

        // capture moves and en passant
        let dirs = if piece.is_white() {
            [DD::UpLeft, DD::UpRight]
        } else {
            [DD::DownLeft, DD::DownRight]
        };
        for dir in dirs {
            let c_squares = Walk::diagonal(board, square, dir, 1);
            moves.extend(c_squares.iter().filter_map(|&sqr| {
                let p = board[sqr];
                if !p.is_blank() && p.get_color() != piece.get_color() {
                    Some(RawPieceMove {
                        piece,
                        from: square,
                        to: sqr,
                    })
                } else if let Some(en_passant) = board.en_passant {
                    if en_passant == sqr {
                        Some(RawPieceMove {
                            piece,
                            from: square,
                            to: sqr,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }));
        }
        moves
    }

    pub fn gen_rook_raw_moves(board: &Board, square: Square) -> Vec<RawPieceMove> {
        let mut moves = vec![];
        let piece = board[square];
        if piece.is_blank() {
            return moves;
        }
        let max_offset = 7;
        let vdirs = [VD::Up, VD::Down];
        let hdirs = [HD::Left, HD::Right];
        // vertical
        for dir in vdirs {
            let squares = Walk::vertical(board, square, dir, max_offset);
            moves.extend(squares.iter().map(|&sqr| RawPieceMove {
                piece,
                from: square,
                to: sqr,
            }))
        }
        // horizontal
        for dir in hdirs {
            let squares = Walk::horizontal(board, square, dir, max_offset);
            moves.extend(squares.iter().map(|&sqr| RawPieceMove {
                piece,
                from: square,
                to: sqr,
            }))
        }
        moves
    }

    pub fn gen_bishop_raw_moves(board: &Board, square: Square) -> Vec<RawPieceMove> {
        let mut moves = vec![];
        let piece = board[square];
        if piece.is_blank() {
            return moves;
        }
        let max_offset = 7;
        let dirs = [DD::UpLeft, DD::UpRight, DD::DownLeft, DD::DownRight];
        for dir in dirs {
            let squares = Walk::diagonal(board, square, dir, max_offset);
            moves.extend(squares.iter().map(|&sqr| RawPieceMove {
                piece,
                from: square,
                to: sqr,
            }))
        }
        moves
    }

    pub fn gen_queen_raw_moves(board: &Board, square: Square) -> Vec<RawPieceMove> {
        let mut moves = vec![];
        let piece = board[square];
        if piece.is_blank() || !piece.is_queen() {
            return moves;
        }
        moves.extend(Self::gen_bishop_raw_moves(board, square));
        moves.extend(Self::gen_rook_raw_moves(board, square));
        moves
    }

    pub fn gen_king_raw_moves(board: &Board, square: Square) -> Vec<RawPieceMove> {
        let mut moves = vec![];
        let piece = board[square];
        if piece.is_blank() {
            return moves;
        }
        let vdirs = [VD::Up, VD::Down];
        let hdirs = [HD::Left, HD::Right];
        let ddirs = [DD::UpLeft, DD::UpRight, DD::DownLeft, DD::DownRight];
        let max_offset = 1;
        // vertical
        for dir in vdirs {
            let squares = Walk::vertical(board, square, dir, max_offset);
            moves.extend(squares.iter().map(|&sqr| RawPieceMove {
                piece,
                from: square,
                to: sqr,
            }))
        }
        // horizontal
        for dir in hdirs {
            let squares = Walk::horizontal(board, square, dir, max_offset);
            moves.extend(squares.iter().map(|&sqr| RawPieceMove {
                piece,
                from: square,
                to: sqr,
            }))
        }
        // diagonal
        for dir in ddirs {
            let squares = Walk::diagonal(board, square, dir, max_offset);
            moves.extend(squares.iter().map(|&sqr| RawPieceMove {
                piece,
                from: square,
                to: sqr,
            }))
        }
        // castling
        if !piece.has_moved() {
            let mut king_side = square;
            let mut queen_side = square;
            let mut king_side_castle = false;
            let mut queen_side_castle = false;
            while let Some(sqr) = king_side.right(1) {
                let p = board[sqr];
                if p.is_blank() {
                    king_side = sqr;
                } else if p.is_rook() && !p.has_moved() && p.get_color() == piece.get_color() {
                    king_side = sqr;
                    king_side_castle = true;
                    break;
                } else {
                    break;
                }
            }
            while let Some(sqr) = queen_side.left(1) {
                let p = board[sqr];
                if p.is_blank() {
                    queen_side = sqr;
                } else if p.is_rook() && !p.has_moved() && p.get_color() == piece.get_color() {
                    queen_side = sqr;
                    queen_side_castle = true;
                    break;
                } else {
                    break;
                }
            }
            if king_side_castle {
                moves.push(RawPieceMove {
                    piece,
                    from: square,
                    to: square.right(2).unwrap(),
                });
            }
            if queen_side_castle {
                moves.push(RawPieceMove {
                    piece,
                    from: square,
                    to: square.left(2).unwrap(),
                });
            }
        }
        moves
    }

    pub fn gen_knight_raw_moves(board: &Board, square: Square) -> Vec<RawPieceMove> {
        let mut moves = vec![];
        let piece = board[square];
        if piece.is_blank() {
            return moves;
        }
        let offsets = [(2, 1), (1, 2)];
        for (x, y) in offsets {
            let squares = [
                square.up(x).map(|s| s.left(y)),
                square.up(x).map(|s| s.right(y)),
                square.down(x).map(|s| s.left(y)),
                square.down(x).map(|s| s.right(y)),
            ];
            for &sqr in squares.iter() {
                if let Some(Some(sqr)) = sqr {
                    let p = board[sqr];
                    if p.is_blank() || p.get_color() != piece.get_color() {
                        moves.push(RawPieceMove {
                            piece,
                            from: square,
                            to: sqr,
                        });
                    }
                }
            }
        }
        moves
    }
}
