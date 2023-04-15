
use crate::piece::PieceType::*;
use crate::piece::Color::*;
//use crate::piece::Color;
use crate::piece::PieceType;

use crate::piece::Piece;
use std::collections::HashMap;

pub type Board = [[Piece; 8]; 8];

#[allow(dead_code)]
pub fn setup_test_board(board: &mut Board) {
    board[3][3] = Piece {piece: King, color: White};
    //board[3][5] = Piece {piece: Knight, color: Black};
}

pub fn setup_board(board: &mut Board) {
    board[0][0] = Piece {piece: Rook, color: White};
    board[0][4] = Piece {piece: King, color: White};
    board[0][7] = Piece {piece: Rook, color: White};

    board[7][0] = Piece {piece: Rook, color: Black};
    board[7][4] = Piece {piece: King, color: Black};
    board[7][7] = Piece {piece: Rook, color: Black};
    /*board[0][0] = Piece {piece: Rook, color: White};
    board[0][1] = Piece {piece: Knight, color: White};
    board[0][2] = Piece {piece: Bishop, color: White};
    board[0][3] = Piece {piece: Queen, color: White};
    board[0][4] = Piece {piece: King, color: White};
    board[0][5] = Piece {piece: Bishop, color: White};
    board[0][6] = Piece {piece: Knight, color: White};
    board[0][7] = Piece {piece: Rook, color: White};
    board[1] = [Piece {piece: Pawn, color: White}; 8];

    board[7][0] = Piece {piece: Rook, color: Black};
    board[7][1] = Piece {piece: Knight, color: Black};
    board[7][2] = Piece {piece: Bishop, color: Black};
    board[7][3] = Piece {piece: Queen, color: Black};
    board[7][4] = Piece {piece: King, color: Black};
    board[7][5] = Piece {piece: Bishop, color: Black};
    board[7][6] = Piece {piece: Knight, color: Black};
    board[7][7] = Piece {piece: Rook, color: Black};
    board[6] = [Piece {piece: Pawn, color: Black}; 8];*/
}

pub fn print_board(board: Board) {
    // for black piece unicodes, add 6
    let unicode_hashmap = HashMap::from([
        (King, 0x2654),
        (Queen, 0x2655),
        (Bishop, 0x2657),
        (Knight, 0x2658),
        (Rook, 0x2656),
        (Pawn, 0x2659),
        (PieceType::Null, 0x2800),
    ]);

    print!("  ABCDEFGH");
    //let mut i = 0;
    for i in 0..8 {
        println!();
        print!("{} ", 8-i);
        for piece in board[7-i] {
            let mut piece_unicode = unicode_hashmap.get(&piece.piece).unwrap().clone();
            if piece.color == White {
                piece_unicode += 6;
            }
            print!("{}", char::from_u32(piece_unicode).unwrap());
        }
        print!(" {}", 8-i);
    }
    println!();
    print!("  ABCDEFGH");
}