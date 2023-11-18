use crate::{
    movegen::{Legal, Raw, RawMove},
    BitPiece, BoardError, Color, ColorError, Piece, Square,
};
use std::{
    error::Error,
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

#[derive(Debug, Copy, Clone, PartialEq)]
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
                    _ => return Err(BoardError::InvalidFEN(format!("{} is not a valid turn", turn))),
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
                                _ => return Err(BoardError::InvalidFEN(format!("{} is not a valid castling", castling))),
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

                let mut halfmove_clock = match halfmove_clock.parse::<u8>() {
                    Ok(halfmove_clock) => halfmove_clock,
                    Err(e) => return Err(BoardError::InvalidFEN(e.to_string())),
                };

                if halfmove_clock > 100 {
                    return Err(BoardError::InvalidFEN(format!("{} is not a valid halfmove clock", halfmove_clock)));
                }

                let mut fullmove_number = match fullmove_number.parse::<u16>() {
                    Ok(fullmove_number) => fullmove_number,
                    Err(e) => return Err(BoardError::InvalidFEN(e.to_string())),
                };

                if fullmove_number == 0 {
                    return Err(BoardError::InvalidFEN(format!("{} is not a valid fullmove number", fullmove_number)));
                }

                Ok(Self {
                    board: b,
                    turn: color,
                    status: BoardStatus::Ongoing,
                    castling,
                    en_passant,
                    halfmove_clock,
                    fullmove_number,
                })
            }
            _ => return Err(BoardError::InvalidFEN(fen)),
        }
    }

    // Should not be used outside library
    // use execute_move instead
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
            }
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
