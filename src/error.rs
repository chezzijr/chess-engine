use thiserror::Error;

#[derive(Error, Debug)]
pub enum PieceError {
    #[error("Invalid piece value: {0}")]
    InvalidValue(u8),
    #[error("Invalid piece character: {0}")]
    InvalidChar(char),
}

#[derive(Error, Debug)]
pub enum ColorError {
    #[error("Invalid color value: {0}")]
    InvalidValue(u8),
    #[error("Invalid color character: {0}")]
    InvalidChar(char),
}

#[derive(Error, Debug)]
pub enum SquareError {
    #[error("Invalid rank: {0}")]
    InvalidRank(char),
    #[error("Invalid file: {0}")]
    InvalidFile(char),
    #[error("Invalid square: {0}")]
    InvalidSquare(String),
    #[error("Invalid square index: {0}")]
    InvalidIndex(usize),
}

#[derive(Error, Debug)]
pub enum BoardError {
    #[error("Illegal move: {0}")]
    IllegalMove(String),
    #[error("Invalid FEN Notation: {0}")]
    InvalidFEN(String),
}
