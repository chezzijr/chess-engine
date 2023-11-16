use std::cmp::{min, max};
use crate::{Board, Square};

// vertical direction
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VD {
    Up,
    Down,
}

// horizontal direction
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HD {
    Left,
    Right,
}

// diagonal direction
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DD {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

pub struct Walk;

impl Walk {
    // the max value of max_offset is 7
    // the value of max_offset is 1 (walk 1 square forward)
    pub fn vertical(board: &Board, mut square: Square, dir: VD, max_offset: u8) -> Vec<Square> {
        let mut squares = Vec::new();
        let piece = board[square];
        if piece.is_blank() {
            return squares;
        }
        let color = piece.get_color();
        let mut max_offset = max(min(max_offset, 7), 1);
        while let Some(next_square) = {
            if dir == VD::Up {
                square.up(1)
            } else {
                square.down(1)
            }
        } {
            if max_offset == 0 {
                break;
            }
            let p = board[next_square];
            if !p.is_blank() {
                if p.get_color() != color {
                    squares.push(next_square);
                    break;
                } else {
                    break;
                }
            }
            max_offset -= 1;
            square = next_square;
            squares.push(square);
        }
        squares
    }

    pub fn horizontal(board: &Board, mut square: Square, dir: HD, max_offset: u8) -> Vec<Square> {
        let mut squares = Vec::new();
        let piece = board[square];
        if piece.is_blank() {
            return squares;
        }
        let color = piece.get_color();
        let mut max_offset = max(min(max_offset, 7), 1);
        while let Some(next_square) = {
            if dir == HD::Left {
                square.left(1)
            } else {
                square.right(1)
            }
        } {
            if max_offset == 0 {
                break;
            }
            let p = board[next_square];
            if !p.is_blank() {
                if p.get_color() != color {
                    squares.push(next_square);
                    break;
                } else {
                    break;
                }
            }
            max_offset -= 1;
            square = next_square;
            squares.push(square);
        }
        squares
    }

    pub fn diagonal(board: &Board, mut square: Square, dir: DD, max_offset: u8) -> Vec<Square> {
        let mut squares = Vec::new();
        let piece = board[square];
        if piece.is_blank() {
            return squares;
        }
        let color = piece.get_color();
        let mut max_offset = max(min(max_offset, 7), 1);
        while let Some(Some(next_square)) = {
            match dir {
                DD::UpLeft => square.up(1).map(|s| s.left(1)),
                DD::UpRight => square.up(1).map(|s| s.right(1)),
                DD::DownLeft => square.down(1).map(|s| s.left(1)),
                DD::DownRight => square.down(1).map(|s| s.right(1)),
            }
        } {
            if max_offset == 0 {
                break;
            }
            let p = board[next_square];
            if !p.is_blank() {
                if p.get_color() != color {
                    squares.push(next_square);
                    break;
                } else {
                    break;
                }
            }
            max_offset -= 1;
            square = next_square;
            squares.push(square);
        }
        squares
    }
}
