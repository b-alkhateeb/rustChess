use crate::piece::Piece;
use crate::piece::PieceType;
use crate::piece::PieceType::*;
use crate::piece::Color::*;
use crate::piece::Color;
use crate::board::Board;
use crate::position::*;

pub fn find_all_legal_moves(board: &Board, turn: Color, move_history: &Vec<MoveHistoryEntry>) -> Vec<Move> {
    let mut res: Vec<Move> = vec![];

    res.extend(find_basic_legal_moves(board, turn));
    
    res.extend(find_en_passant_moves(board, turn, move_history));
    res.extend(find_castling_moves(board, turn, move_history));

    remove_moves_leading_to_check(&mut res, &board, turn);

    return res;
}

pub fn find_basic_legal_moves(board: &Board, turn: Color) -> Vec<Move> {
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

    return res
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
                special_move: if to_pos.rank == 7 {Some(SpecialMoveType::Promote)} else {None},
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
                special_move: Some(SpecialMoveType::PawnLongMove),
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
                special_move: if to_pos.rank == 7 {Some(SpecialMoveType::Promote)} else {None},
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
                special_move: if to_pos.rank == 7 {Some(SpecialMoveType::Promote)} else {None},
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
                special_move: if to_pos.rank == 0 {Some(SpecialMoveType::Promote)} else {None},
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
                special_move: Some(SpecialMoveType::PawnLongMove),
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
                special_move: if to_pos.rank == 0 {Some(SpecialMoveType::Promote)} else {None},
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
                special_move: if to_pos.rank == 0 {Some(SpecialMoveType::Promote)} else {None},
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
        let fake_legal_moves = find_basic_legal_moves(&fake_board, fake_turn); // I think we can get away without the move history and special moves here?

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

pub fn find_en_passant_moves(board: &Board, turn: Color, move_history: &Vec<MoveHistoryEntry>) -> Vec<Move> {
    let mut res = vec![];
    let last_played_move_opt = move_history.last();

    if last_played_move_opt == None {
        return res;
    }

    let last_played_move = &last_played_move_opt.unwrap().moveEntry;

    // last_played_move being PawnLongMove also implies that the color is opposite to turn
    // and that the piece is a pawn
    if last_played_move.special_move == Some(SpecialMoveType::PawnLongMove) {
        if turn == Color::White {
            // posible positions for a pawn to be able to take the last moved pawn enpassant
            let possible_position_1 = move_right(last_played_move.to.clone());
            let possible_position_2 = move_left(last_played_move.to.clone());

            // Some assumptions are valid here hence the unwraps
            // since the last move is a PawnLongMove, we know that the square behind it is free
            // we also know that it won't cross the board boundaries

            if let Ok(pos) = possible_position_1 {
                if board[pos.rank][pos.file].piece == PieceType::Pawn && board[pos.rank][pos.file].color == Color::White {
                    res.push(Move {
                        from: pos.clone(),
                        to: move_up_left(pos.clone()).unwrap(),
                        piece: board[pos.rank][pos.file].clone(),
                        special_move: Some(SpecialMoveType::EnPassant),
                    });
                }
            }

            if let Ok(pos) = possible_position_2 {
                if board[pos.rank][pos.file].piece == PieceType::Pawn && board[pos.rank][pos.file].color == Color::White {
                    res.push(Move {
                        from: pos.clone(),
                        to: move_up_right(pos.clone()).unwrap(),
                        piece: board[pos.rank][pos.file].clone(),
                        special_move: Some(SpecialMoveType::EnPassant),
                    });
                }
            }
        } else if turn == Color::Black {
            // posible positions for a pawn to be able to take the last moved pawn enpassant
            let possible_position_1 = move_right(last_played_move.to.clone());
            let possible_position_2 = move_left(last_played_move.to.clone());

            // Some assumptions are valid here hence the unwraps
            // since the last move is a PawnLongMove, we know that the square behind it is free
            // we also know that it won't cross the board boundaries

            if let Ok(pos) = possible_position_1 {
                if board[pos.rank][pos.file].piece == PieceType::Pawn && board[pos.rank][pos.file].color == Color::Black {
                    res.push(Move {
                        from: pos.clone(),
                        to: move_down_left(pos.clone()).unwrap(),
                        piece: board[pos.rank][pos.file].clone(),
                        special_move: Some(SpecialMoveType::EnPassant),
                    });
                }
            }

            if let Ok(pos) = possible_position_2 {
                if board[pos.rank][pos.file].piece == PieceType::Pawn && board[pos.rank][pos.file].color == Color::Black {
                    res.push(Move {
                        from: pos.clone(),
                        to: move_down_right(pos.clone()).unwrap(),
                        piece: board[pos.rank][pos.file].clone(),
                        special_move: Some(SpecialMoveType::EnPassant),
                    });
                }
            }
        }
    }

    res
}

pub fn find_castling_moves(board: &Board, turn: Color, move_history: &Vec<MoveHistoryEntry>) -> Vec<Move> {
    let mut res = vec![];

    if white_can_castle_long(board, move_history) && turn == Color::White {
        res.push(Move {
            from: Square{file: 4, rank: 0},
            to: Square{file: 2, rank: 0},
            piece: Piece {piece: PieceType::King, color: Color::White},
            special_move: Some(SpecialMoveType::CastleLong),
        })
    }

    if white_can_castle_short(board, move_history) && turn == Color::White {
        res.push(Move {
            from: Square{file: 4, rank: 0},
            to: Square{file: 6, rank: 0},
            piece: Piece {piece: PieceType::King, color: Color::White},
            special_move: Some(SpecialMoveType::CastleShort),
        })
    }

    if black_can_castle_long(board, move_history) && turn == Color::Black {
        res.push(Move {
            from: Square{file: 4, rank: 7},
            to: Square{file: 2, rank: 7},
            piece: Piece {piece: PieceType::King, color: Color::Black},
            special_move: Some(SpecialMoveType::CastleLong),
        })
    }

    if black_can_castle_short(board, move_history) && turn == Color::Black {
        res.push(Move {
            from: Square{file: 4, rank: 7},
            to: Square{file: 6, rank: 7},
            piece: Piece {piece: PieceType::King, color: Color::Black},
            special_move: Some(SpecialMoveType::CastleShort),
        })
    }

    return res;
}

pub fn white_can_castle_long(board: &Board, move_history: &Vec<MoveHistoryEntry>) -> bool {
    let mut king_rook_never_moved: bool = true;
    for hmove in move_history {
        if hmove.moveEntry.piece.piece == PieceType::King && hmove.moveEntry.piece.color == Color::White {
            king_rook_never_moved = false;
            break;
        }
        if hmove.moveEntry.piece.piece == PieceType::Rook && hmove.moveEntry.piece.color == Color::White {
            if hmove.moveEntry.from.file == 0 && hmove.moveEntry.from.rank == 0 {
                king_rook_never_moved = false;
                break;
            }
        }
    }
    // extra check for non-standard initial positions
    if board[4][0].piece != PieceType::King && board[4][0].color != Color::White {
        king_rook_never_moved = false;
    }

    let no_pieces_block_castle = board[0][1].piece == PieceType::Null 
        && board[0][2].piece == PieceType::Null
        && board[0][3].piece == PieceType::Null;

    let mut hypothetical_king_moves = vec![Move {
        from: Square{file: 4, rank: 0},
        to: Square{file: 4, rank: 0},
        piece: Piece {piece: PieceType::King, color: Color::White},
        special_move: None,
    }, Move {
        from: Square{file: 4, rank: 0},
        to: Square{file: 3, rank: 0},
        piece: Piece {piece: PieceType::King, color: Color::White},
        special_move: None,
    }, Move {
        from: Square{file: 4, rank: 0},
        to: Square{file: 2, rank: 0},
        piece: Piece {piece: PieceType::King, color: Color::White},
        special_move: None,
    }];
    remove_moves_leading_to_check(&mut hypothetical_king_moves, board, Color::White);
    let no_check_in_king_path = hypothetical_king_moves.len() == 3;

    return king_rook_never_moved && no_pieces_block_castle && no_check_in_king_path;
}

pub fn white_can_castle_short(board: &Board, move_history: &Vec<MoveHistoryEntry>) -> bool {
    let mut king_rook_never_moved: bool = true;
    for hmove in move_history {
        if hmove.moveEntry.piece.piece == PieceType::King && hmove.moveEntry.piece.color == Color::White {
            king_rook_never_moved = false;
            break;
        }
        if hmove.moveEntry.piece.piece == PieceType::Rook && hmove.moveEntry.piece.color == Color::White {
            if hmove.moveEntry.from.file == 7 && hmove.moveEntry.from.rank == 0 {
                king_rook_never_moved = false;
                break;
            }
        }
    }
    // extra check for non-standard initial positions
    if board[4][0].piece != PieceType::King && board[4][0].color != Color::White {
        king_rook_never_moved = false;
    }

    let no_pieces_block_castle = board[0][5].piece == PieceType::Null 
        && board[0][6].piece == PieceType::Null;

    let mut hypothetical_king_moves = vec![Move {
        from: Square{file: 4, rank: 0},
        to: Square{file: 4, rank: 0},
        piece: Piece {piece: PieceType::King, color: Color::White},
        special_move: None,
    }, Move {
        from: Square{file: 4, rank: 0},
        to: Square{file: 5, rank: 0},
        piece: Piece {piece: PieceType::King, color: Color::White},
        special_move: None,
    }, Move {
        from: Square{file: 4, rank: 0},
        to: Square{file: 6, rank: 0},
        piece: Piece {piece: PieceType::King, color: Color::White},
        special_move: None,
    }];
    remove_moves_leading_to_check(&mut hypothetical_king_moves, board, Color::White);
    let no_check_in_king_path = hypothetical_king_moves.len() == 3;

    return king_rook_never_moved && no_pieces_block_castle && no_check_in_king_path;
}

pub fn black_can_castle_long(board: &Board, move_history: &Vec<MoveHistoryEntry>) -> bool {
    let mut king_rook_never_moved: bool = true;
    for hmove in move_history {
        if hmove.moveEntry.piece.piece == PieceType::King && hmove.moveEntry.piece.color == Color::Black {
            king_rook_never_moved = false;
            break;
        }
        if hmove.moveEntry.piece.piece == PieceType::Rook && hmove.moveEntry.piece.color == Color::Black {
            if hmove.moveEntry.from.file == 0 && hmove.moveEntry.from.rank == 7 {
                king_rook_never_moved = false;
                break;
            }
        }
    }
    // extra check for non-standard initial positions
    if board[4][7].piece != PieceType::King && board[4][7].color != Color::Black {
        king_rook_never_moved = false;
    }

    let no_pieces_block_castle = board[7][1].piece == PieceType::Null 
        && board[7][2].piece == PieceType::Null
        && board[7][3].piece == PieceType::Null;

    let mut hypothetical_king_moves = vec![Move {
        from: Square{file: 4, rank: 7},
        to: Square{file: 4, rank: 7},
        piece: Piece {piece: PieceType::King, color: Color::Black},
        special_move: None,
    }, Move {
        from: Square{file: 4, rank: 7},
        to: Square{file: 3, rank: 7},
        piece: Piece {piece: PieceType::King, color: Color::Black},
        special_move: None,
    }, Move {
        from: Square{file: 4, rank: 7},
        to: Square{file: 2, rank: 7},
        piece: Piece {piece: PieceType::King, color: Color::Black},
        special_move: None,
    }];
    remove_moves_leading_to_check(&mut hypothetical_king_moves, board, Color::Black);
    let no_check_in_king_path = hypothetical_king_moves.len() == 3;

    return king_rook_never_moved && no_pieces_block_castle && no_check_in_king_path;
}

pub fn black_can_castle_short(board: &Board, move_history: &Vec<MoveHistoryEntry>) -> bool {
    let mut king_rook_never_moved: bool = true;
    for hmove in move_history {
        if hmove.moveEntry.piece.piece == PieceType::King && hmove.moveEntry.piece.color == Color::Black {
            king_rook_never_moved = false;
            break;
        }
        if hmove.moveEntry.piece.piece == PieceType::Rook && hmove.moveEntry.piece.color == Color::Black {
            if hmove.moveEntry.from.file == 7 && hmove.moveEntry.from.rank == 0 {
                king_rook_never_moved = false;
                break;
            }
        }
    }
    // extra check for non-standard initial positions
    if board[4][7].piece != PieceType::King && board[4][7].color != Color::Black {
        king_rook_never_moved = false;
    }

    let no_pieces_block_castle = board[7][5].piece == PieceType::Null 
        && board[7][6].piece == PieceType::Null;

    let mut hypothetical_king_moves = vec![Move {
        from: Square{file: 4, rank: 7},
        to: Square{file: 4, rank: 7},
        piece: Piece {piece: PieceType::King, color: Color::Black},
        special_move: None,
    }, Move {
        from: Square{file: 4, rank: 7},
        to: Square{file: 5, rank: 7},
        piece: Piece {piece: PieceType::King, color: Color::Black},
        special_move: None,
    }, Move {
        from: Square{file: 4, rank: 7},
        to: Square{file: 6, rank: 7},
        piece: Piece {piece: PieceType::King, color: Color::Black},
        special_move: None,
    }];
    remove_moves_leading_to_check(&mut hypothetical_king_moves, board, Color::Black);
    let no_check_in_king_path = hypothetical_king_moves.len() == 3;

    return king_rook_never_moved && no_pieces_block_castle && no_check_in_king_path;
}