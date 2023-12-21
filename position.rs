use crate::Piece;
use crate::Board;

#[derive(Debug, Clone, PartialEq)]
pub struct Square {
    pub file: usize,       //vertical
    pub rank: usize,       //horizontal
}

#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub piece: Piece,
    pub special_move: Option<SpecialMoveType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoveHistoryEntry {
    pub moveEntry: Move,
    pub boardState: Board,
    pub isCaptureMove: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpecialMoveType {
    PawnLongMove, // eg e2->e4, moving two squares at the start
    EnPassant,
    CastleLong,
    CastleShort,
    Promote,
}

pub fn move_right(current_square: Square) -> Result<Square, ()> {
    if current_square.file >= 7 {
        return Err(());
    }

    let mut new_square = current_square.clone();
    new_square.file += 1;

    Ok(new_square)
}

pub fn move_left(current_square: Square) -> Result<Square, ()> {
    if current_square.file <= 0 {
        return Err(());
    }

    let mut new_square = current_square.clone();
    new_square.file -= 1;

    Ok(new_square)
}

pub fn move_down(current_square: Square) -> Result<Square, ()> {
    if current_square.rank <= 0 {
        return Err(());
    }

    let mut new_square = current_square.clone();
    new_square.rank -= 1;

    Ok(new_square)
}

pub fn move_up(current_square: Square) -> Result<Square, ()> {
    if current_square.rank >= 7 {
        return Err(());
    }

    let mut new_square = current_square.clone();
    new_square.rank += 1;

    Ok(new_square)
}

pub fn move_up_right(current_square: Square) -> Result<Square, ()> {
    move_up(move_right(current_square)?)
}

pub fn move_up_left(current_square: Square) -> Result<Square, ()> {
    move_up(move_left(current_square)?)
}

pub fn move_down_right(current_square: Square) -> Result<Square, ()> {
    move_down(move_right(current_square)?)
}

pub fn move_down_left(current_square: Square) -> Result<Square, ()> {
    move_down(move_left(current_square)?)
}

//pawn stuff

pub fn move_up_up(current_square: Square) -> Result<Square, ()> {
    move_up(move_up(current_square)?)
}

pub fn move_down_down(current_square: Square) -> Result<Square, ()> {
    move_down(move_down(current_square)?)
}

// knight stuff
pub fn move_up_up_left(current_square: Square) -> Result<Square, ()> {
    move_up(move_up(move_left(current_square)?)?)
}

pub fn move_up_left_left(current_square: Square) -> Result<Square, ()> {
    move_up(move_left(move_left(current_square)?)?)
}

pub fn move_up_up_right(current_square: Square) -> Result<Square, ()> {
    move_up(move_up(move_right(current_square)?)?)
}

pub fn move_up_right_right(current_square: Square) -> Result<Square, ()> {
    move_up(move_right(move_right(current_square)?)?)
}

pub fn move_down_down_left(current_square: Square) -> Result<Square, ()> {
    move_down(move_down(move_left(current_square)?)?)
}

pub fn move_down_left_left(current_square: Square) -> Result<Square, ()> {
    move_down(move_left(move_left(current_square)?)?)
}

pub fn move_down_down_right(current_square: Square) -> Result<Square, ()> {
    move_down(move_down(move_right(current_square)?)?)
}

pub fn move_down_right_right(current_square: Square) -> Result<Square, ()> {
    move_down(move_right(move_right(current_square)?)?)
}