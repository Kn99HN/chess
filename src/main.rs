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
    let from_square = chess_board.board[from];
    let to_square = chess_board.board[to];
    let from_piece = from_square.value;
    let to_piece = to_square.value;
    match (from_piece, to_piece)  {
        (Some(a), None) => true,
        (Some(a), Some(b)) => true,
        (None, Some(b)) => false,
        (None, None) => false,
    }
}

pub fn validate_move_by_piece(chess_piece: ChessPiece, from: usize, to: usize) -> bool {
    let current_piece = chess_piece.piece;
    return match current_piece {
        Piece::Pawn => (from + 8) == to,
        Piece::King => (from + 8) == to,
        // @ToDo: Determine how to validate other moves
        _ => false
    };
}

fn main() {
    let contents = match fs::read_to_string("./inputs/game1.txt") {
        Err(why) => panic!("Couldn't read file because {}", why),
        Ok(content) => content
    };
    
    let op = parse(&contents);
}
