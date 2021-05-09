use std::fs;

#[derive(Copy, Clone)]
enum Piece {
    King,
    Queen,
    Knight,
    Bishop,
    Rook,
    Pawn
}

#[derive(Copy, Clone)]
enum Player {
    Black,
    White
}

#[derive(Copy, Clone)]
pub struct ChessPiece {
    piece: Piece,
    player: Player 
}

#[derive(Copy, Clone)]
pub struct Square {
    value: Option<ChessPiece>,
}

impl Square {
    fn new() -> Self {
        Square {
            value: None
        }
    }

    pub fn init(&mut self, piece: ChessPiece){
        self.value = Some(piece);
    }
}

pub struct ChessBoard {
    board: [Square; 64]
}

impl ChessBoard {
    fn new() -> Self {
        let mut chess_board = [Square::new(); 64];
        chess_board[0].init(ChessPiece {
            piece: Piece::Rook,
            player: Player::White
        });
        chess_board[7].init(ChessPiece {
            piece: Piece::Rook,
            player: Player::White
        });

        chess_board[1].init(ChessPiece {
            piece: Piece::Knight,
            player: Player::White
        });
        chess_board[6].init(ChessPiece {
            piece: Piece::Knight,
            player: Player::White
        });

        chess_board[2].init(ChessPiece {
            piece: Piece::Bishop,
            player: Player::White
        });
        chess_board[5].init(ChessPiece {
            piece: Piece::Bishop,
            player: Player::White
        });

        chess_board[3].init(ChessPiece {
            piece: Piece::Queen,
            player: Player::White
        });
        chess_board[4].init(ChessPiece {
            piece: Piece::King,
            player: Player::White
        });


        chess_board[56].init(ChessPiece {
            piece: Piece::Rook,
            player: Player::Black
        });
        chess_board[63].init(ChessPiece {
            piece: Piece::Rook,
            player: Player::Black
        });

        chess_board[57].init(ChessPiece {
            piece: Piece::Knight,
            player: Player::Black
        });
        chess_board[62].init(ChessPiece {
            piece: Piece::Knight,
            player: Player::Black
        });

        chess_board[58].init(ChessPiece {
            piece: Piece::Bishop,
            player: Player::Black
        });
        chess_board[61].init(ChessPiece {
            piece: Piece::Bishop,
            player: Player::Black
        });

        chess_board[59].init(ChessPiece {
            piece: Piece::Queen,
            player: Player::Black
        });
        chess_board[60].init(ChessPiece {
            piece: Piece::King,
            player: Player::Black
        });

        for idx in 8..15 {
            chess_board[idx].init(ChessPiece {
                piece: Piece::Pawn,
                player: Player::White
            })
        }

        for idx in 48..55 {
            chess_board[idx].init(ChessPiece {
                piece: Piece::Pawn,
                player: Player::Black
            });
        }

        ChessBoard {
            board: chess_board
        }
    }
}

#[derive(Debug)]
pub struct Operation {
    op: String,
    from: usize,
    to: usize,
}

pub fn parse(content: &str) -> Operation {
    let mut moves = content.split_whitespace();
    let op = match moves.next() {
        Some(valid_op) => valid_op.split_whitespace().collect(),
        None => panic!("Something wrong")
    };
    let mut directions = match moves.next() {
        Some(valid_direction) => valid_direction.split(","),
        None => panic!("Something wrong")
    };
    let start = match directions.next() {
        Some(start) => start,
        None => panic!("Something wrong")
    };
    let dest = match directions.next() {
        Some(dest) => dest,
        None => panic!("Something wrong")
    };
    let from_idx = convert_to_idx(&start);
    let to_idx = convert_to_idx(&dest);
    return Operation {
        op: op,
        from: from_idx,
        to: to_idx
    }
}

pub fn convert_to_idx(chess_move: &str) -> usize {
    let mut chars = chess_move.chars();
    let start = 'A' as u32;
    let col = chars.next().unwrap() as u32;
    let row = match chars.next().unwrap().to_digit(10) {
        Some(val) => val,
        None => panic!("Something wrong")
    };
    return ((col - start) + (row - 1)* 8) as usize;
}

pub fn validate_move(op: Operation, chess_board: ChessBoard) -> bool {
    if op.from >= 64 || op.to >= 64 {
        return false
    }
    let from = op.from;
    let to = op.to;
    let from_piece = chess_board.board[from].value;
    let to_piece = chess_board.board[to].value;
    match (from_piece, to_piece)  {
        (Some(a), None) => return validate_move_by_piece(&chess_board, a, from, to),
        (Some(a), Some(b)) => return validate_move_by_pieces(&chess_board, a, b, from, to),
        (None, Some(b)) => false,
        (None, None) => false,
    }
}

pub fn validate_move_by_piece(chess_board: &ChessBoard, chess_piece: ChessPiece, from: usize, to: usize) -> bool {
    let current_piece = chess_piece.piece;
    return match current_piece {
        Piece::Pawn => (from + 8) == to,
        Piece::King => (from + 8) == to,
        Piece::Rook => return validate_horizontal_or_vertical(chess_board, from, to),
        Piece::Knight => return validate_l_shape(from, to),
        Piece::Bishop => return validate_diagonals(chess_board, from, to),
        Piece::Queen => return validate_diagonals(chess_board, from, to) && validate_horizontal_or_vertical(chess_board, from, to),
    };
}

pub fn validate_move_by_pieces(chess_board: &ChessBoard, from_piece: ChessPiece, to_piece: ChessPiece, from: usize, to: usize) -> bool {
    let from_piece_val = from_piece.piece;
    let to_piece_val = to_piece.piece;
    let from_piece_player = from_piece.player;
    let to_piece_player = to_piece.player;
    return match (from_piece_val, to_piece_val, from_piece_player, to_piece_player) {
        (Piece::Pawn, Piece::Pawn, Player::White, Player::Black) => {
            return (get_col(from) + 1 == get_col(to) && get_row(from) + 1 == get_row(to)) ||
                    (get_col(from) - 1 == get_col(to) && get_row(from) + 1 == get_row(to))
        },
        (Piece::Pawn, Piece::Pawn, Player::Black, Player::White) => {
            return (get_col(from) + 1 == get_col(to) && get_row(from) -1 == get_row(to)) ||
                    (get_col(from) - 1 == get_col(to) && get_row(from) - 1 == get_row(to))
        },
        (Piece::King, _, _, _) => {
            return (get_col(from) + 1 == get_col(to) && get_row(from) + 1 == get_row(to)) ||
                    (get_col(from) - 1 == get_col(to) && get_row(from) + 1 == get_row(to))
        },
        (Piece::Queen, _, _, _) => {
            return validate_diagonals(chess_board, from, to) && validate_horizontal_or_vertical(chess_board, from, to);
        },
        (Piece::Bishop, _, _, _) => {
            return validate_diagonals(chess_board, from, to);
        },
        (Piece::Knight, _, _, _) => {
            return validate_l_shape(from, to);
        },
        (Piece::Rook, _, _, _) => {
            return validate_horizontal_or_vertical(chess_board, from, to);
        },
        (_, _, Player::Black, Player::Black) => false,
        (_, _, Player::White, Player::White) => false, 
        (_, _, _, _) => false,
    }
}

pub fn validate_l_shape(from: usize, to:usize) -> bool {
    let from_col = get_col(from);
    let from_row = get_row(from);
    let to_col = get_col(to);
    let to_row = get_row(to);
    return
        (to_col - from_col == 1 && to_row - to_col == 2) ||
        (to_col - from_col == 1 && to_col - to_row == 2) || 
        (to_col - from_col == 2 && to_row - from_row == 1) || 
        (to_col - from_col == 2 && from_row - to_row == 1) || 
        (from_col - to_col == 2 && to_row - from_row == 1) ||
        (from_col - to_col == 2 && from_row - to_row == 1) || 
        (from_col - to_col == 1 && from_row - to_row == 2) || 
        (from_col - to_col == 1 && to_row - from_row == 2)
}

// double check this logic
pub fn validate_diagonals(chess_board: &ChessBoard, from: usize, to: usize) -> bool {
    let board = (*chess_board).board;
    let from_col = get_col(from);
    let from_row = get_row(from);
    let to_col = get_col(to);
    let to_row = get_row(to);
    if from_row < to_row && from_col < to_col {
        for (i, j) in (from_row..to_row).zip(from_col..to_col) {
            return match board[to_idx(i, j)].value {
                Some(_) => false,
                None => true,
            }
        }
    } else if from_row > to_row && from_col > to_col {
        for (i, j) in (to_row..from_row).zip(to_col..from_col) {
            return match board[to_idx(i, j)].value {
                Some(_) => false,
                None => true
            }
        }
    } else if from_row < to_row && from_col > to_col {
        for (i, j) in (from_row..to_row).zip(to_col..from_col) {
            return match board[to_idx(i, j)].value {
                Some(_) => false,
                None => true
            }
        }
    } else if from_row > to_row && from_col < to_col {
        for (i, j) in (to_row..from_row).zip(from_col..to_col) {
            return match board[to_idx(i, j)].value {
                Some(_) => false,
                None => true,
            }
        }
    }
    return false;
}

pub fn validate_horizontal_or_vertical(chess_board: &ChessBoard, from: usize, to: usize) -> bool {
    let board = (*chess_board).board;
    let from_col = get_col(from);
    let from_row = get_row(from);
    let to_row = get_row(to);
    let to_col = get_col(to);
    if from_col == to_col {
        if from_row < to_row {
            for i in from_row..to_row {
                return match board[to_idx(i, from_col)].value {
                    Some(a) => false,
                    None => true,
                };
            }
        } else if from_row > to_row {
            for i in to_row..from_row {
                return match board[to_idx(i, from_col)].value {
                    Some(a) => false,
                    None => true,
                };
            }
        }
    } else if from_row == to_row {
        if from_col < to_col {
            for i in from_col..to_col {
                return match board[to_idx(from_row, i)].value {
                    Some(a) => false,
                    None => true,
                };
            }
        } else if to_col < from_col {
            for i in to_col..from_col {
                return match board[to_idx(from_row, i)].value {
                    Some(a) => false,
                    None => true,
                };
            }
        } 
    }
    return false;
}

pub fn get_col(idx: usize) -> usize {
    return (idx - (idx % 8)) / 8;
}

pub fn get_row(idx: usize) -> usize {
    return idx % 8;
}

pub fn to_idx(row: usize, col: usize) -> usize {
    return row * 8 + col;
}

fn main() {
    let contents = match fs::read_to_string("./inputs/game1.txt") {
        Err(why) => panic!("Couldn't read file because {}", why),
        Ok(content) => content
    };
    
    let op = parse(&contents);
}
