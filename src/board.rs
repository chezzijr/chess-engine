use crate::{
    movegen::{Legal, RawMove},
    BitPiece, BoardError, Color, Piece, Square, MoveInfo, CastleMove,
};
use regex::Regex;
use std::{
    fmt,
    ops::{Index, IndexMut},
};

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

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    board: BitBoard,
    pub turn: Color,
    pub status: BoardStatus,
    // castling: 0b0000_0000
    // bit 3: white king side
    // bit 2: white queen side
    // bit 1: black king side
    // bit 0: black queen side
    pub castling: u8,
    pub en_passant: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,

    pub history: Vec<MoveInfo>,
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
        let mut board = [BitPiece::new_blank(); 64];
        for i in 0..8 {
            board[8 + i] = BitPiece::new(Piece::Pawn, Color::White, false);
            board[48 + i] = BitPiece::new(Piece::Pawn, Color::Black, false);
        }
        for i in 0..2 {
            let color = if i == 0 { Color::White } else { Color::Black };
            let row = if i == 0 { 0 } else { 7 };
            board[0 + row * 8] = BitPiece::new(Piece::Rook, color, false);
            board[7 + row * 8] = BitPiece::new(Piece::Rook, color, false);
            board[1 + row * 8] = BitPiece::new(Piece::Knight, color, false);
            board[6 + row * 8] = BitPiece::new(Piece::Knight, color, false);
            board[2 + row * 8] = BitPiece::new(Piece::Bishop, color, false);
            board[5 + row * 8] = BitPiece::new(Piece::Bishop, color, false);
            board[3 + row * 8] = BitPiece::new(Piece::Queen, color, false);
            board[4 + row * 8] = BitPiece::new(Piece::King, color, false);
        }
        Self {
            board,
            turn: Color::White,
            status: BoardStatus::Ongoing,
            castling: 0b1111,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,

            history: Vec::new(),
        }
    }
}

impl Board {
    pub fn from_fen(fen: String) -> Result<Self, BoardError> {
        let coll = fen.split_whitespace().collect::<Vec<&str>>();
        match coll[..] {
            [board, turn, castling, en_passant, halfmove_clock, fullmove_number] => {
                let mut b: BitBoard = [BitPiece::new_blank(); 64];
                let mut rank = 7;
                let mut file = 0;
                for c in board.chars() {
                    if c == '/' {
                        rank -= 1;
                        file = 0;
                    } else if c.is_digit(10) {
                        file += c.to_digit(10).unwrap();
                    } else {
                        let p = match BitPiece::try_from(c) {
                            Ok(p) => p,
                            Err(e) => return Err(BoardError::InvalidFEN(e.to_string())),
                        };
                        b[rank * 8 + file as usize] = p;
                        file += 1;
                    }
                }

                let color = match turn {
                    "w" => Color::White,
                    "b" => Color::Black,
                    _ => {
                        return Err(BoardError::InvalidFEN(format!(
                            "{} is not a valid turn",
                            turn
                        )))
                    }
                };

                let castling = match castling {
                    "-" => 0,
                    _ => {
                        let mut castle = 0;
                        for c in castling.chars() {
                            match c {
                                'K' => castle |= 0b1000,
                                'Q' => castle |= 0b0100,
                                'k' => castle |= 0b0010,
                                'q' => castle |= 0b0001,
                                _ => {
                                    return Err(BoardError::InvalidFEN(format!(
                                        "{} is not a valid castling",
                                        castling
                                    )))
                                }
                            }
                        }
                        castle
                    }
                };

                let en_passant = match en_passant {
                    "-" => None,
                    _ => match Square::try_from(en_passant.to_owned()) {
                        Ok(en_passant) => Some(en_passant),
                        Err(e) => return Err(BoardError::InvalidFEN(e.to_string())),
                    },
                };

                let halfmove_clock = match halfmove_clock.parse::<u8>() {
                    Ok(halfmove_clock) => halfmove_clock,
                    Err(e) => return Err(BoardError::InvalidFEN(e.to_string())),
                };

                if halfmove_clock > 100 {
                    return Err(BoardError::InvalidFEN(format!(
                        "{} is not a valid halfmove clock",
                        halfmove_clock
                    )));
                }

                let fullmove_number = match fullmove_number.parse::<u16>() {
                    Ok(fullmove_number) => fullmove_number,
                    Err(e) => return Err(BoardError::InvalidFEN(e.to_string())),
                };

                if fullmove_number == 0 {
                    return Err(BoardError::InvalidFEN(format!(
                        "{} is not a valid fullmove number",
                        fullmove_number
                    )));
                }

                Ok(Self {
                    board: b,
                    turn: color,
                    status: BoardStatus::Ongoing,
                    castling,
                    en_passant,
                    halfmove_clock,
                    fullmove_number,
                    history: Vec::new(),
                })
            }
            _ => return Err(BoardError::InvalidFEN(fen)),
        }
    }

    // Should not be used outside library
    // use execute_move instead
    pub fn force_execute_raw_move(&mut self, mv: RawMove) {
        match mv {
            RawMove::Single(mut info) => {
                if let Some(capture) = info.capture {
                    self[capture.square] = BitPiece::new_blank();
                }
                self[info.from] = BitPiece::new_blank();
                info.piece.set_moved();
                self[info.to] = info.piece;
                if let Some(en_passant_square) = info.en_passant_square {
                    self.en_passant = Some(en_passant_square);
                }
            }
            RawMove::Castle(mut info1, mut info2) => {
                self[info1.from] = BitPiece::new_blank();
                info1.piece.set_moved();
                self[info1.to] = info1.piece;
                self[info2.from] = BitPiece::new_blank();
                info2.piece.set_moved();
                self[info2.to] = info2.piece;
            }
        }
    }

    pub fn execute_move(&mut self, mv: &MoveInfo, rmv: &RawMove) {
        match rmv {
            RawMove::Single(mut info) => {
                if let Some(capture) = info.capture {
                    self[capture.square] = BitPiece::new_blank();
                }
                self[info.from] = BitPiece::new_blank();
                if let Some(promotion) = mv.promotion {
                    self[info.to] = promotion;
                } else {
                    info.piece.set_moved();
                    self[info.to] = info.piece;
                }
                if let Some(en_passant_square) = info.en_passant_square {
                    self.en_passant = Some(en_passant_square);
                }
            }
            RawMove::Castle(mut info1, mut info2) => {
                self[info1.from] = BitPiece::new_blank();
                info1.piece.set_moved();
                self[info1.to] = info1.piece;
                self[info2.from] = BitPiece::new_blank();
                info2.piece.set_moved();
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
        let dest = raw_moves.iter().filter_map(|&mov| match mov {
            RawMove::Single(info) => {
                if info.piece.get_color() != color {
                    Some(info.to)
                } else {
                    None
                }
            }
            _ => None,
        });
        for mov in dest {
            let p = self[mov];
            if !p.is_blank() && p.is_king() && p.get_color() == color {
                return true;
            }
        }
        false
    }

    pub fn legal_moves(&self) -> Vec<RawMove> {
        Legal::gen_all_legal_moves(self)
    }

    pub fn parse_move(&self, m: String) -> Result<RawMove, BoardError> {
        // this is used to parse string move to RawMove to execute move
        // pawn abbreviation may be specified or not, eg. e4 or Pe4
        // normal case: {piece}{to} eg. Nf3
        // capture: {piece}x{to} eg. Nxe4
        // promotion: {piece}{to}={piece} eg. e8=Q
        // en passant: same as capture but with en passant square eg. exd6
        // castle: O-O or O-O-O
        // conclude: {file or rank or none}{piece}{x or none}{to}{promotion or none} if noncastling
        // else O-O or O-O-O
        // {to} is a must-have field
        let mut legal_moves = self.legal_moves().into_iter().filter(|&mov| {
            match mov {
                RawMove::Single(info) => {
                    if info.piece.get_color() == self.turn {
                        true
                    } else {
                        false
                    }
                }
                RawMove::Castle(info1, _) => {
                    if info1.piece.get_color() == self.turn {
                        true
                    } else {
                        false
                    }
                },
            }
        }).collect::<Vec<RawMove>>();

        if m == "O-O" || m == "O-O-O" {
            let legal_castle = legal_moves.iter().find(|&mov| match mov {
                RawMove::Castle(_, info2) => {
                    if m == "O-O" {
                        info2.from.file() == 'h'
                    } else {
                        info2.from.file() == 'a'
                    }
                }
                _ => false,
            });
            if let Some(m) = legal_castle {
                return Ok(m.clone());
            } else {
                return Err(BoardError::IllegalMove(m));
            }
        }

        const PATTERN: &'static str = r"^(?<from>([a-h]|[1-8]))?(?<piece>[kqbnrpKQBNRP])?(?<capture>x)?(?<to>[a-h][1-8])(?<promote>=[qnbrQNBR])?$";
        let re = Regex::new(PATTERN).unwrap();
        let Some(cap) = re.captures(&m) else {
            return Err(BoardError::InvalidPattern(m));
        };

        let from = cap.name("from");
        let piece = cap.name("piece");
        let capture = cap.name("capture");
        let to = cap.name("to");
        let promote = cap.name("promote");

        let to_square = Square::try_from(to.unwrap().as_str().to_owned()).unwrap();
        legal_moves.retain(|&mov| match mov {
            RawMove::Single(info) => info.to == to_square,
            _ => false,
        });

        if let Some(from) = from {
            legal_moves.retain(|&mov| match mov {
                RawMove::Single(info) => {
                    // because of the match, from is guaranteed to be a file or rank
                    let c = from.as_str().chars().next().unwrap();
                    if c.is_digit(10) {
                        info.from.rank() == c.to_digit(10).unwrap() as u8
                    } else {
                        info.from.file() == c
                    }
                },
                _ => false,
            });
        }

        if let Some(piece) = piece {
            let chr = piece.as_str().chars().next().unwrap();
            if chr.is_uppercase() && self.turn == Color::Black {
                return Err(BoardError::IllegalMove(m));
            }
            if chr.is_lowercase() && self.turn == Color::White {
                return Err(BoardError::IllegalMove(m));
            }
            legal_moves.retain(|&mov| match mov {
                RawMove::Single(info) => {
                    let p = piece.as_str();
                    let p = if p.len() == 0 { 
                        'P'
                    } else {
                        p.chars().next().unwrap()
                    };
                    info.piece.get_piece() == Piece::try_from(p).unwrap()
                },
                _ => false,
            });
        }

        if let Some(_) = capture {
            legal_moves.retain(|&mov| match mov {
                RawMove::Single(info) => {
                    info.capture.is_some()
                },
                _ => false,
            });
        }

        if let Some(promote) = promote {
            let chr = promote.as_str().chars().nth(1).unwrap();
            if chr.is_uppercase() && self.turn == Color::Black {
                return Err(BoardError::IllegalMove(m));
            }
            if chr.is_lowercase() && self.turn == Color::White {
                return Err(BoardError::IllegalMove(m));
            }
            legal_moves.retain(|&mov| match mov {
                RawMove::Single(info) => {
                    info.promotion
                },
                _ => false,
            });
        };

        // if there is 1 move match the algebraic notation, return it
        // else if more than 1 => ambiguous move
        // else if 0 => illegal move
        if legal_moves.len() == 1 {
            Ok(legal_moves[0].clone())
        } else if legal_moves.len() > 1 {
            Err(BoardError::AmbiguousMove(m))
        } else {
            Err(BoardError::IllegalMove(m))
        }
    }

    pub fn make_move(&mut self, m: String) -> Result<(), BoardError> {
        let mov = self.parse_move(m.clone())?;

        let mut move_info = match mov {
            RawMove::Single(info) => {
                // if pawn move or capture, reset halfmove clock
                if info.piece.is_pawn() || info.capture.is_some() {
                    self.halfmove_clock = 0;
                } else {
                    self.halfmove_clock += 1;
                };
                // update en passant
                if let Some(en_passant_square) = info.en_passant_square {
                    self.en_passant = Some(en_passant_square);
                } else {
                    self.en_passant = None;
                };
                // castling
                if info.piece.is_king() && !info.piece.has_moved() {
                    match self.turn {
                        Color::White => {
                            self.castling &= 0b1100;
                        },
                        Color::Black => {
                            self.castling &= 0b0011;
                        },
                    };
                } else if info.piece.is_rook() && !info.piece.has_moved() {
                    match self.turn {
                        Color::White => {
                            if info.from.file() == 'h' {
                                self.castling &= 0b1011;
                            } else if info.from.file() == 'a' {
                                self.castling &= 0b1110;
                            }
                        },
                        Color::Black => {
                            if info.from.file() == 'h' {
                                self.castling &= 0b0111;
                            } else if info.from.file() == 'a' {
                                self.castling &= 0b1101;
                            }
                        },
                    };
                }

                MoveInfo {
                    piece: info.piece,
                    from: info.from,
                    to: info.to,
                    capture: if let Some(capture) = info.capture {
                        Some(capture.piece)
                    } else {
                        None
                    },
                    promotion: if info.promotion {
                        let chr = m.chars().last().unwrap();
                        let p = Piece::try_from(chr).unwrap();
                        Some(BitPiece::new(p, self.turn, true))
                    } else {
                        None
                    },
                    castle: None,
                    en_passant: info.en_passant,
                    en_passant_square: info.en_passant_square,
                    check: false,
                    checkmate: false,
                }
            },
            RawMove::Castle(info1, info2) => {
                // because we update turn in the beginning of this function
                // to get the correct turn, we need to get the opposite of current turn
                match self.turn {
                    Color::White => {
                        self.castling &= 0b1100;
                    },
                    Color::Black => {
                        self.castling &= 0b0011;
                    },
                };
                self.halfmove_clock += 1;
                MoveInfo {
                    piece: info1.piece,
                    from: info1.from,
                    to: info1.to,
                    capture: None,
                    promotion: None,
                    castle: if info2.from.file() == 'h' {
                        Some(CastleMove::KingSide)
                    } else {
                        Some(CastleMove::QueenSide)
                    },
                    en_passant: false,
                    en_passant_square: None,
                    check: false,
                    checkmate: false,
                }
            }
        };

        self.execute_move(&move_info, &mov);
        self.turn = self.turn.opposite();
        let next_legal_moves = self.legal_moves();

        if self.is_being_checked(self.turn, &next_legal_moves) {
            if next_legal_moves.iter().find(|&mov| match mov {
                RawMove::Single(info) => {
                    if info.piece.get_color() == self.turn {
                        true
                    } else {
                        false
                    }
                }
                RawMove::Castle(info1, _) => {
                    if info1.piece.get_color() == self.turn {
                        true
                    } else {
                        false
                    }
                },
            }).is_some() {
                self.status = BoardStatus::Check(self.turn);
                move_info.check = true;
            } else {
                self.status = BoardStatus::Checkmate(self.turn);
                move_info.checkmate = true;
            }
        }

        if self.turn == Color::White {
            self.fullmove_number += 1;
        }

        self.history.push(move_info);

        Ok(())
    }

    pub fn fen_notation(&self) -> String {
        let mut fen = String::new();
        // board
        for i in (0..8).rev() {
            let mut blank_count = 0;
            for j in 0..8 {
                let p = self[Square::try_from(i * 8 + j).unwrap()];
                if p.is_blank() {
                    blank_count += 1;
                } else {
                    if blank_count > 0 {
                        fen.push_str(&blank_count.to_string());
                        blank_count = 0;
                    }
                    fen.push_str(&format!("{}", p));
                }
            }
            if blank_count > 0 {
                fen.push_str(&blank_count.to_string());
            }
            if i > 0 {
                fen.push('/');
            }
        }
        fen.push(' ');
        // turn
        fen.push_str(&format!("{}", self.turn));
        fen.push(' ');

        // castling
        if self.castling == 0 {
            fen.push('-');
        } else {
            if self.castling & 0b1000 != 0 {
                fen.push('K');
            }
            if self.castling & 0b0100 != 0 {
                fen.push('Q');
            }
            if self.castling & 0b0010 != 0 {
                fen.push('k');
            }
            if self.castling & 0b0001 != 0 {
                fen.push('q');
            }
        }
        fen.push(' ');

        // en passant
        if let Some(en_passant) = self.en_passant {
            fen.push_str(&format!("{}", en_passant));
        } else {
            fen.push('-');
        }
        fen.push(' ');

        // halfmove clock
        fen.push_str(&format!("{}", self.halfmove_clock));
        fen.push(' ');

        // fullmove number
        fen.push_str(&format!("{}", self.fullmove_number));
        fen
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in (0..8).rev() {
            write!(f, "{} ", i + 1)?;
            for j in 0..8 {
                let p = self[Square::try_from(i * 8 + j).unwrap()];
                if p.is_blank() {
                    write!(f, ". ")?;
                } else {
                    write!(f, "{} ", p)?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "  a b c d e f g h\n")
    }
}
