use std::env;
use std::fs;
use std::convert::From;

enum Piece {
    King,
    Queen,
    Knight,
    Bishop,
    Rook,
    Pawn
}

struct ChessMove {
    piece: Piece,
    row: i8,
    col: i8
}

struct ChessBoard {
    rows: i8,
    cols: i8,
    board: [i8; 64]
}

impl ChessBoard {
    fn new() -> Self {
        ChessBoard {
            rows: 8,
            cols: 8,
            board: [0; 64]
        }
    }

    // validate whether the move is within the board
    fn validateMove(mv:ChessMove) {
        
    }

    // validate a move from the piece is valid or not
    fn validatePieceMove(mv:ChessMove) {
        
    }
}

impl ChessMove {
    fn new(mv:&str) -> Self {
        let piecech = mv.chars().next().unwrap();
        match piecech {
            'K' => ChessMove { piece: Piece::King, row: 0, col: 0},
            'Q' => ChessMove { piece: Piece::Queen, row: 0, col: 0},
            'N' => ChessMove { piece: Piece::Knight, row: 0, col: 0},
            'B' => ChessMove { piece: Piece::Bishop, row: 0, col: 0},
            'R' => ChessMove { piece: Piece::Rook, row: 0, col: 0},
            _ => ChessMove { piece: Piece::Pawn, row: 0, col: 0}
        }
    }

    fn convertCol(col:char) {
        
    }
}



fn main() {
    let contents = match fs::read_to_string("./test/game1.txt") {
        Err(why) => panic!("Couldn't read file because {}", why),
        Ok(content) => content
    };
    
    let mut white_moves:Vec<&str> = Vec::new();
    let mut black_moves:Vec<&str> = Vec::new();
    let mut moves = contents.split_whitespace();
    for chess_move in moves {
        let mut bw_moves = chess_move.split(",");
        let white_move = match bw_moves.next() {
            Some(mv) => white_moves.push(mv),
            None => ()
        };
        let black_move = match bw_moves.next() {
            Some(mv) => black_moves.push(mv),
            None => ()
        };
    }
}
