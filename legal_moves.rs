use crate::piece::Piece;
use crate::piece::PieceType;
use crate::piece::PieceType::*;
use crate::piece::Color::*;
use crate::piece::Color;
use crate::board::Board;
use crate::position::*;

pub fn find_all_legal_moves(board: &Board, turn: Color) -> Vec<Move> {
    let mut res: Vec<Move> = vec![];

    for rank in 0..8 {
        for file in 0..8 {
            let pos = Square {file, rank};
            if board[rank][file].color == turn {
                match board[rank][file].piece {
                    Rook => res.extend(find_rook_legal_moves(board, pos, board[rank][file].color)),
                    Bishop => res.extend(find_bishop_legal_moves(board, pos, board[rank][file].color)),
                    Queen => res.extend(find_queen_legal_moves(board, pos, board[rank][file].color)),
                    Knight => res.extend(find_knight_legal_moves(board, pos, board[rank][file].color)),
                    King => res.extend(find_king_legal_moves(board, pos, board[rank][file].color)),
                    Pawn => res.extend(find_pawn_legal_moves(board, pos, board[rank][file].color)),
                    _ => {}
                }
            }
        }
    }
    
    return res;
}

fn find_bishop_legal_moves(board: &Board, position: Square, color: Color) -> Vec<Move> {
    let mut res: Vec<Move> = vec![];
    
    res.extend(find_moves_in_direction(board, position.clone(), color, move_up_right));
    res.extend(find_moves_in_direction(board, position.clone(), color, move_up_left));
    res.extend(find_moves_in_direction(board, position.clone(), color, move_down_right));
    res.extend(find_moves_in_direction(board, position.clone(), color, move_down_left));
    
    return res;
}

fn find_rook_legal_moves(board: &Board, position: Square, color: Color) -> Vec<Move> {
    let mut res: Vec<Move> = vec![];

    res.extend(find_moves_in_direction(board, position.clone(), color, move_right));
    res.extend(find_moves_in_direction(board, position.clone(), color, move_left));
    res.extend(find_moves_in_direction(board, position.clone(), color, move_up));
    res.extend(find_moves_in_direction(board, position.clone(), color, move_down));
    
    return res;
}

fn find_queen_legal_moves(board: &Board, position: Square, color: Color) -> Vec<Move> {
    let mut res: Vec<Move> = vec![];

    res.extend(find_rook_legal_moves(board, position.clone(), color));
    res.extend(find_bishop_legal_moves(board, position.clone(), color));
    
    return res;
}

fn find_knight_legal_moves(board: &Board, position: Square, color: Color) -> Vec<Move> {
    let mut res: Vec<Move> = vec![];

    // vec of fn pointers
    let moves_to_test = Vec::from([move_up_up_left, move_up_left_left, move_up_up_right, move_up_right_right,
                move_down_down_left, move_down_left_left, move_down_down_right, move_down_right_right]);

    for move_to_test in moves_to_test {
        if let Ok(to_pos) = move_to_test(position.clone()) {
            let move_under_consideration = Move {
                from: position.clone(),
                to: to_pos.clone(),
                piece: board[position.rank][position.file].clone(),
                special_move: None,
            };
            if board[to_pos.rank][to_pos.file].color != color{
                res.push(move_under_consideration);
            }
        }
    }
    
    return res;
}

fn find_king_legal_moves(board: &Board, position: Square, color: Color) -> Vec<Move> {
    let mut res: Vec<Move> = vec![];

    // vec of fn pointers
    let moves_to_test = Vec::from([move_up, move_down, move_left, move_right,
                move_up_right, move_up_left, move_down_right, move_down_left]);

    for move_to_test in moves_to_test {
        if let Ok(to_pos) = move_to_test(position.clone()) {
            let move_under_consideration = Move {
                from: position.clone(),
                to: to_pos.clone(),
                piece: board[position.rank][position.file].clone(),
                special_move: None,
            };
            if board[to_pos.rank][to_pos.file].color != color {
                res.push(move_under_consideration);
            }
        }
    }
    
    return res;
}

fn find_pawn_legal_moves(board: &Board, position: Square, color: Color) -> Vec<Move> {
    let mut res: Vec<Move> = vec![];

    if color == White {
        if let Ok(to_pos) = move_up(position.clone()) {
            let move_under_consideration = Move {
                from: position.clone(),
                to: to_pos.clone(),
                piece: board[position.rank][position.file].clone(),
                special_move: None,
            };
            if board[to_pos.rank][to_pos.file].color == Color::Null {
                res.push(move_under_consideration);
            }
        }
        if let Ok(to_pos) = move_up_up(position.clone()) {
            let move_under_consideration = Move {
                from: position.clone(),
                to: to_pos.clone(),
                piece: board[position.rank][position.file].clone(),
                special_move: None,
            };
            if position.rank == 1 {
                if board[to_pos.rank][to_pos.file].color == Color::Null {
                    res.push(move_under_consideration);
                }
            }
        }
        if let Ok(to_pos) = move_up_left(position.clone()) {
            let move_under_consideration = Move {
                from: position.clone(),
                to: to_pos.clone(),
                piece: board[position.rank][position.file].clone(),
                special_move: None,
            };
            if board[to_pos.rank][to_pos.file].color == Color::Black {
                res.push(move_under_consideration);
            }
        }
        if let Ok(to_pos) = move_up_right(position.clone()) {
            let move_under_consideration = Move {
                from: position.clone(),
                to: to_pos.clone(),
                piece: board[position.rank][position.file].clone(),
                special_move: None,
            };
            if board[to_pos.rank][to_pos.file].color == Color::Black {
                res.push(move_under_consideration);
            }
        }
    } else if color == Black {
        if let Ok(to_pos) = move_down(position.clone()) {
            let move_under_consideration = Move {
                from: position.clone(),
                to: to_pos.clone(),
                piece: board[position.rank][position.file].clone(),
                special_move: None,
            };
            if board[to_pos.rank][to_pos.file].color == Color::Null {
                res.push(move_under_consideration);
            }
        }
        if let Ok(to_pos) = move_down_down(position.clone()) {
            let move_under_consideration = Move {
                from: position.clone(),
                to: to_pos.clone(),
                piece: board[position.rank][position.file].clone(),
                special_move: None,
            };
            if position.rank == 6 {
                if board[to_pos.rank][to_pos.file].color == Color::Null {
                    res.push(move_under_consideration);
                }
            }
        }
        if let Ok(to_pos) = move_down_left(position.clone()) {
            let move_under_consideration = Move {
                from: position.clone(),
                to: to_pos.clone(),
                piece: board[position.rank][position.file].clone(),
                special_move: None,
            };
            if board[to_pos.rank][to_pos.file].color == Color::White {
                res.push(move_under_consideration);
            }
        }
        if let Ok(to_pos) = move_down_right(position.clone()) {
            let move_under_consideration = Move {
                from: position.clone(),
                to: to_pos.clone(),
                piece: board[position.rank][position.file].clone(),
                special_move: None,
            };
            if board[to_pos.rank][to_pos.file].color == Color::White {
                res.push(move_under_consideration);
            }
        }
    }
    
    return res;
}

fn find_moves_in_direction(
    board: &Board, 
    position: Square, 
    color: Color, 
    direction_fn: fn(Square)->Result<Square,()>,
) -> Vec<Move> {
    let mut res = vec![];
    let from_pos = position.clone();
    let mut pos = from_pos.clone();
    while let Ok(to_pos) = direction_fn(pos.clone()) {
        let move_under_consideration = Move {
            from: from_pos.clone(),
            to: to_pos.clone(),
            piece: board[position.rank][position.file].clone(),
            special_move: None,
        };
        if board[to_pos.rank][to_pos.file].color == Color::Null {
            res.push(move_under_consideration);
        } else if board[to_pos.rank][to_pos.file].color != color {
            res.push(move_under_consideration);
            break;
        } else {
            break;
        }
        pos = to_pos;
    }

    res
}

pub fn remove_moves_leading_to_check(legal_moves: &mut Vec<Move>, board: &Board, turn: Color) {
    // for every move assumed to be legal
    legal_moves.retain(|legal_move| {
        let mut fake_board = board.clone();

        // play the move
        fake_board[legal_move.to.rank][legal_move.to.file] = fake_board[legal_move.from.rank][legal_move.from.file];
        fake_board[legal_move.from.rank][legal_move.from.file] = Piece {piece: PieceType::Null, color: Color::Null};
        
        // find all legal moves for the opponent 
        let fake_turn = if turn == White {Black} else {White};
        let fake_legal_moves = find_all_legal_moves(&fake_board, fake_turn);

        // locate the current player's king
        let mut king_position: Square = Square {rank: 8, file: 8};
        for i in 0..8 {
            for j in 0..8 {
                let piece = fake_board[i][j];
                if piece.piece == King && piece.color == turn {
                    king_position = Square {rank: i, file: j};
                }
            }
        }

        // check if any opponent move can capture the king
        for fake_legal_move in fake_legal_moves {
            if fake_legal_move.to == king_position {
                return false;
            }
        }

        true
    });

}