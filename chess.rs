
mod position;
mod piece;
mod board;
mod legal_moves;

/*
TODO
    evaluate win (checkmates)
        stalemate
        50 move draw rule
        3 fold repetition ----> this is hard, might need to store all positions in a map with count?
*/

use crate::piece::Piece;
use crate::piece::PieceType;
use crate::piece::Color;
use crate::piece::Color::*;
use crate::board::Board;
use crate::board::setup_board;
use crate::board::print_board;
use crate::position::*;
use crate::legal_moves::*;
use std::collections::HashMap;

fn main() {
    let empty_piece = Piece {piece: PieceType::Null, color: Color::Null};

    let mut move_history: Vec<Move> = Vec::new();

    let mut board: Board = [[empty_piece; 8]; 8];
    let mut turn: Color = White;
    setup_board(&mut board);
    
    loop {
        let mut legal_moves = find_all_legal_moves(&board, turn, &move_history);
        remove_moves_leading_to_check(&mut legal_moves, &board, turn);
        for legal_move in legal_moves.iter() {
            println!("{:?}", legal_move);
        }

        print_board(board);
        println!();

        let input_move = match read_input_move() {
            Ok(v) => v,
            _ => {
                println!("illegal move format");
                continue;
            }
        };

        let mut move_played_flag = false;
        for legal_move in legal_moves.iter() {
            if legal_move.from == input_move.from && legal_move.to == input_move.to {
                play_move(&mut board, legal_move.clone());
                move_history.push(legal_move.clone());
                if legal_move.special_move == Some(SpecialMoveType::Promote) {
                    prompt_promotion(&mut board, legal_move.to.clone());
                }
                turn = if turn == White {Black} else {White};
                move_played_flag = true;
            }
        }

        if !move_played_flag {
            println!("illegal move, pick another");
        }

        println!("----");
        for hist_move in move_history.iter() {
            println!("{:?}", hist_move);
        }
        println!("----");
    }
}

fn read_input_move() -> Result<Move, ()> {
    let mut user_input = String::new();
    let _b = match std::io::stdin().read_line(&mut user_input) {
        Ok(v) => v,
        _ => {return Err(());}
    };

    let file_to_num: HashMap<char, usize> = HashMap::from([
        ('a', 0), ('A', 0),
        ('b', 1), ('B', 1),
        ('c', 2), ('C', 2),
        ('d', 3), ('D', 3),
        ('e', 4), ('E', 4),
        ('f', 5), ('F', 5),
        ('g', 6), ('G', 6),
        ('h', 7), ('H', 7),
    ]);
    
    Ok(Move {
        from: Square {
            file: *file_to_num.get(&user_input.chars().nth(0).ok_or(())?).ok_or(())?,
            rank: (user_input.chars().nth(1).ok_or(())?.to_digit(10).ok_or(())? - 1) as usize,
        },
        to: Square {
            file: *file_to_num.get(&user_input.chars().nth(2).ok_or(())?).ok_or(())?,
            rank: (user_input.chars().nth(3).ok_or(())?.to_digit(10).ok_or(())? - 1) as usize, 
        },
        // piece and special_move fields are in legal_moves array, dont need to find them here
        piece: Piece {piece: PieceType::Null, color: Color::Null}, 
        special_move: None,
    })
}

fn play_move(board: &mut Board, input_move: Move) {
    board[input_move.to.rank][input_move.to.file] = board[input_move.from.rank][input_move.from.file];
    board[input_move.from.rank][input_move.from.file] = Piece {piece: PieceType::Null, color: Color::Null};
    
    if input_move.special_move == Some(SpecialMoveType::EnPassant) {
        if input_move.piece.color == Color::White {
            let captured_pawn_square = move_down(input_move.to.clone()).unwrap();
            board[captured_pawn_square.rank][captured_pawn_square.file] = Piece {piece: PieceType::Null, color: Color::Null};
        } else if input_move.piece.color == Color::Black {
            let captured_pawn_square = move_up(input_move.to.clone()).unwrap();
            board[captured_pawn_square.rank][captured_pawn_square.file] = Piece {piece: PieceType::Null, color: Color::Null};
        }
    } else if input_move.special_move == Some(SpecialMoveType::CastleLong) {
        if input_move.piece.color == Color::White {
            board[0][0] = Piece {piece: PieceType::Null, color: Color::Null};
            board[0][3] = Piece {piece: PieceType::Rook, color: Color::White};
        } else if input_move.piece.color == Color::Black {
            board[7][0] = Piece {piece: PieceType::Null, color: Color::Null};
            board[7][3] = Piece {piece: PieceType::Rook, color: Color::Black};
        }
    } else if input_move.special_move == Some(SpecialMoveType::CastleShort) {
        if input_move.piece.color == Color::White {
            board[0][7] = Piece {piece: PieceType::Null, color: Color::Null};
            board[0][5] = Piece {piece: PieceType::Rook, color: Color::White};
        } else if input_move.piece.color == Color::Black {
            board[7][7] = Piece {piece: PieceType::Null, color: Color::Null};
            board[7][5] = Piece {piece: PieceType::Rook, color: Color::Black};
        }
    }
}

fn prompt_promotion(board: &mut Board, square: Square) {
    loop {
       println!("Which piece would you like to promote to (enter Q,K,B,R)");

        let mut user_input = String::new();
        let _b = match std::io::stdin().read_line(&mut user_input) {
            Ok(v) => v,
            _ => {println!("Illegal input, please enter Q, K, B, or R"); continue}
        };
    
        if vec!['Q', 'K', 'B', 'R'].contains(&user_input.chars().nth(0).unwrap()) {
            match user_input.chars().nth(0).unwrap() {
                'Q' => board[square.rank][square.file].piece = PieceType::Queen,
                'K' => board[square.rank][square.file].piece = PieceType::Knight,
                'B' => board[square.rank][square.file].piece = PieceType::Bishop,
                'R' => board[square.rank][square.file].piece = PieceType::Rook,
                _ => {}
            }
            break;
        }

        println!("Illegal input, please enter Q, K, B, or R");
    }
}