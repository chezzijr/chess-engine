use std::fmt;

use super::{Walk, DD, HD, VD};
use crate::{Board, Square, CastleMove, BitPiece};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RawMoveInfo {
    pub piece: BitPiece,
    pub from: Square,
    pub to: Square,
    pub capture: Option<CaptureInfo>,
    // does this move allow promotion
    pub promotion: bool,
    // is this move a castle move
    pub castle: Option<CastleMove>,
    // is this an en passant move?
    pub en_passant: bool,
    // does this move create an en passant square?
    pub en_passant_square: Option<Square>,
}

// information about a captures piece
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CaptureInfo {
    pub piece: BitPiece,
    pub square: Square
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RawMove {
    Single(RawMoveInfo),
    // castle move contains 2 moves, the king and the rook
    Castle(RawMoveInfo, RawMoveInfo)
}

impl fmt::Display for RawMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RawMove::Single(info) => {
                write!(f, "{} from {} to {}", info.piece, info.from, info.to)
            },
            RawMove::Castle(info1, _) => {
                write!(f, "{} from {} to {}", info1.piece, info1.from, info1.to)
            }
        }
    }
}
// How to generate legal moves:
// 1. Generate all possible raw moves
// 2. Filter out illegal moves (e.g. moves that leave the king in check)
// To filter, we execute raw move on temporary board and check if king is in check
pub struct Raw;

impl Raw {
    pub fn gen_all_raw_moves(board: &Board) -> Vec<RawMove> {
        let mut moves = vec![];
        let gen_fns = [
            Self::gen_pawn_raw_moves,
            Self::gen_knight_raw_moves,
            Self::gen_bishop_raw_moves,
            Self::gen_rook_raw_moves,
            Self::gen_queen_raw_moves,
            Self::gen_king_raw_moves,
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

    pub fn gen_pawn_raw_moves(board: &Board, square: Square) -> Vec<RawMove> {
        let mut moves = vec![];
        let piece = board[square];
        if piece.is_blank() {
            return moves;
        }
        // vertical, noncapturing moves
        let dir = if piece.is_white() { VD::Up } else { VD::Down };
        let max_offset = if piece.has_moved() { 1 } else { 2 };
        let n_squares = Walk::vertical(board, square, dir, max_offset);
        moves.extend(n_squares.iter().filter_map(|&sqr| {
            let p = board[sqr];
            if p.is_blank() {
                Some(RawMove::Single(RawMoveInfo {
                    piece,
                    from: square,
                    to: sqr,
                    capture: None,
                    promotion: if piece.is_white() {
                        sqr.rank() == 8
                    } else {
                        sqr.rank() == 1
                    },
                    castle: None,
                    en_passant: false,
                    en_passant_square: if piece.is_white() {
                        square.up(1)
                    } else {
                        square.down(1)
                    },
                }))
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
                    Some(RawMove::Single(RawMoveInfo {
                        piece,
                        from: square,
                        to: sqr,
                        capture: Some(CaptureInfo {
                            piece: p,
                            square: sqr,
                        }),
                        promotion: if piece.is_white() {
                            sqr.rank() == 8
                        } else {
                            sqr.rank() == 1
                        },
                        castle: None,
                        en_passant: false,
                        en_passant_square: None,
                    }))
                } else if let Some(en_passant) = board.en_passant {
                    if en_passant == sqr {
                        Some(RawMove::Single(RawMoveInfo {
                            piece,
                            from: square,
                            to: sqr,
                            capture: Some(CaptureInfo {
                                piece: board[en_passant.down(1).unwrap()],
                                square: en_passant.down(1).unwrap(),
                            }),
                            promotion: if piece.is_white() {
                                sqr.rank() == 8
                            } else {
                                sqr.rank() == 1
                            },
                            castle: None,
                            en_passant: true,
                            en_passant_square: None,
                        }))
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

    pub fn gen_rook_raw_moves(board: &Board, square: Square) -> Vec<RawMove> {
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
            moves.extend(squares.iter().map(|&sqr| {
                let p = board[sqr];
                RawMove::Single(RawMoveInfo {
                    piece,
                    from: square,
                    to: sqr,
                    capture: if p.is_blank() {
                        None
                    } else {
                        Some(CaptureInfo {
                            piece: p,
                            square: sqr,
                        })
                    },
                    promotion: false,
                    castle: None,
                    en_passant: false,
                    en_passant_square: None,
                })
            }))
        }
        // horizontal
        for dir in hdirs {
            let squares = Walk::horizontal(board, square, dir, max_offset);
            moves.extend(squares.iter().map(|&sqr| {
                let p = board[sqr];
                RawMove::Single(RawMoveInfo {
                    piece,
                    from: square,
                    to: sqr,
                    capture: if p.is_blank() {
                        None
                    } else {
                        Some(CaptureInfo {
                            piece: p,
                            square: sqr,
                        })
                    },
                    promotion: false,
                    castle: None,
                    en_passant: false,
                    en_passant_square: None,
                })
            }))
        }
        moves
    }

    pub fn gen_bishop_raw_moves(board: &Board, square: Square) -> Vec<RawMove> {
        let mut moves = vec![];
        let piece = board[square];
        if piece.is_blank() {
            return moves;
        }
        let max_offset = 7;
        let dirs = [DD::UpLeft, DD::UpRight, DD::DownLeft, DD::DownRight];
        for dir in dirs {
            let squares = Walk::diagonal(board, square, dir, max_offset);
            moves.extend(squares.iter().map(|&sqr| {
                let p = board[sqr];
                RawMove::Single(RawMoveInfo {
                    piece,
                    from: square,
                    to: sqr,
                    capture: if p.is_blank() {
                        None
                    } else {
                        Some(CaptureInfo {
                            piece: p,
                            square: sqr,
                        })
                    },
                    promotion: false,
                    castle: None,
                    en_passant: false,
                    en_passant_square: None,
                })
            }))
        }
        moves
    }

    pub fn gen_queen_raw_moves(board: &Board, square: Square) -> Vec<RawMove> {
        let mut moves = vec![];
        let piece = board[square];
        if piece.is_blank() || !piece.is_queen() {
            return moves;
        }
        moves.extend(Self::gen_bishop_raw_moves(board, square));
        moves.extend(Self::gen_rook_raw_moves(board, square));
        moves
    }

    pub fn gen_king_raw_moves(board: &Board, square: Square) -> Vec<RawMove> {
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
            moves.extend(squares.iter().map(|&sqr| {
                let p = board[sqr];
                RawMove::Single(RawMoveInfo {
                    piece,
                    from: square,
                    to: sqr,
                    capture: if p.is_blank() {
                        None
                    } else {
                        Some(CaptureInfo {
                            piece: p,
                            square: sqr,
                        })
                    },
                    promotion: false,
                    castle: None,
                    en_passant: false,
                    en_passant_square: None,
                })
            }))
        }
        // horizontal
        for dir in hdirs {
            let squares = Walk::horizontal(board, square, dir, max_offset);
            moves.extend(squares.iter().map(|&sqr| {
                let p = board[sqr];
                RawMove::Single(RawMoveInfo {
                    piece,
                    from: square,
                    to: sqr,
                    capture: if p.is_blank() {
                        None
                    } else {
                        Some(CaptureInfo {
                            piece: p,
                            square: sqr,
                        })
                    },
                    promotion: false,
                    castle: None,
                    en_passant: false,
                    en_passant_square: None,
                })
            }))
        }
        // diagonal
        for dir in ddirs {
            let squares = Walk::diagonal(board, square, dir, max_offset);
            moves.extend(squares.iter().map(|&sqr| {
                let p = board[sqr];
                RawMove::Single(RawMoveInfo {
                    piece,
                    from: square,
                    to: sqr,
                    capture: if p.is_blank() {
                        None
                    } else {
                        Some(CaptureInfo {
                            piece: p,
                            square: sqr,
                        })
                    },
                    promotion: false,
                    castle: None,
                    en_passant: false,
                    en_passant_square: None,
                })
            }))
        }
        // castling
        if !piece.has_moved() {
            // current position of king is "e_"
            let queen_side_rook_square =
                Square::try_from(format!("{}{}", 'a', square.rank())).unwrap();
            let king_side_rook_square =
                Square::try_from(format!("{}{}", 'h', square.rank())).unwrap();
            let queen_side_rook = board[queen_side_rook_square];
            let king_side_rook = board[king_side_rook_square];
            if !queen_side_rook.is_blank() && !queen_side_rook.is_rook() && !queen_side_rook.has_moved() {
                // check if squares between king and rook are empty
                let mut all_empty = true;
                for file in (b'b'..=b'd').map(char::from) {
                    let sqr = Square::try_from(format!("{}{}", file, square.rank())).unwrap();
                    if !board[sqr].is_blank() {
                        all_empty = false;
                        break;
                    }
                }
                if all_empty {
                    moves.push(
                        RawMove::Castle(RawMoveInfo {
                        piece,
                        from: square,
                        to: Square::try_from(format!("{}{}", 'c', square.rank())).unwrap(),
                        capture: None,
                        promotion: false,
                        castle: Some(CastleMove::QueenSide),
                        en_passant: false,
                        en_passant_square: None,
                    }, RawMoveInfo {
                        piece,
                        from: square,
                        to: Square::try_from(format!("{}{}", 'd', square.rank())).unwrap(),
                        capture: None,
                        promotion: false,
                        castle: Some(CastleMove::QueenSide),
                        en_passant: false,
                        en_passant_square: None,
                    }));
                }
            }
            if !king_side_rook.is_blank() && !king_side_rook.is_rook() && !king_side_rook.has_moved() {
                // check if squares between king and rook are empty
                let mut all_empty = true;
                for file in (b'f'..=b'g').map(char::from) {
                    let sqr = Square::try_from(format!("{}{}", file, square.rank())).unwrap();
                    if !board[sqr].is_blank() {
                        all_empty = false;
                        break;
                    }
                }
                if all_empty {
                    moves.push(
                        RawMove::Castle(RawMoveInfo {
                        piece,
                        from: square,
                        to: Square::try_from(format!("{}{}", 'g', square.rank())).unwrap(),
                        capture: None,
                        promotion: false,
                        castle: Some(CastleMove::KingSide),
                        en_passant: false,
                        en_passant_square: None,
                    }, RawMoveInfo {
                        piece,
                        from: square,
                        to: Square::try_from(format!("{}{}", 'f', square.rank())).unwrap(),
                        capture: None,
                        promotion: false,
                        castle: Some(CastleMove::KingSide),
                        en_passant: false,
                        en_passant_square: None,
                    }));
                }
            }
        }
        moves
    }

    pub fn gen_knight_raw_moves(board: &Board, square: Square) -> Vec<RawMove> {
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
                        moves.push(RawMove::Single(RawMoveInfo {
                            piece,
                            from: square,
                            to: sqr,
                            capture: if p.is_blank() {
                                None
                            } else {
                                Some(CaptureInfo {
                                    piece: p,
                                    square: sqr,
                                })
                            },
                            promotion: false,
                            castle: None,
                            en_passant: false,
                            en_passant_square: None,
                        }));
                    }
                }
            }
        }
        moves
    }
}
