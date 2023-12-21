#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub struct Piece {
    pub piece: PieceType,
    pub color: Color,
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
    Null,
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub enum Color {
    Black,
    White,
    Null,
}