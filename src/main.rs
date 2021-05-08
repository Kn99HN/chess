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
struct ChessPiece {
    piece: Piece,
    player: Player 
}

#[derive(Copy, Clone)]
struct Square {
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

struct ChessBoard {
    rows: i8,
    cols: i8,
    board: [Square; 64]
}

impl ChessBoard {
    fn new() -> Self {
        let mut chessBoard = [Square::new(); 64];
        chessBoard[0].init(ChessPiece {
            piece: Piece::Rook,
            player: Player::Black
        });
        chessBoard[7].init(ChessPiece {
            piece: Piece::Rook,
            player: Player::Black
        });

        chessBoard[1].init(ChessPiece {
            piece: Piece::Knight,
            player: Player::Black
        });
        chessBoard[6].init(ChessPiece {
            piece: Piece::Knight,
            player: Player::Black
        });

        chessBoard[2].init(ChessPiece {
            piece: Piece::Bishop,
            player: Player::Black
        });
        chessBoard[5].init(ChessPiece {
            piece: Piece::Bishop,
            player: Player::Black
        });

        chessBoard[3].init(ChessPiece {
            piece: Piece::Queen,
            player: Player::Black
        });
        chessBoard[4].init(ChessPiece {
            piece: Piece::King,
            player: Player::Black
        });


        chessBoard[56].init(ChessPiece {
            piece: Piece::Rook,
            player: Player::White
        });
        chessBoard[63].init(ChessPiece {
            piece: Piece::Rook,
            player: Player::White
        });

        chessBoard[57].init(ChessPiece {
            piece: Piece::Knight,
            player: Player::White
        });
        chessBoard[62].init(ChessPiece {
            piece: Piece::Knight,
            player: Player::White
        });

        chessBoard[58].init(ChessPiece {
            piece: Piece::Bishop,
            player: Player::White
        });
        chessBoard[61].init(ChessPiece {
            piece: Piece::Bishop,
            player: Player::White
        });

        chessBoard[59].init(ChessPiece {
            piece: Piece::Queen,
            player: Player::White
        });
        chessBoard[60].init(ChessPiece {
            piece: Piece::King,
            player: Player::White
        });

        for idx in 8..15 {
            chessBoard[idx].init(ChessPiece {
                piece: Piece::Pawn,
                player: Player::Black
            })
        }

        for idx in 48..55 {
            chessBoard[idx].init(ChessPiece {
                piece: Piece::Pawn,
                player: Player::White
            });
        }

        ChessBoard {
            rows: 8,
            cols: 8,
            board: chessBoard
        }
    }
}

struct Operations {
    op: String,
    from: String,
    to: String,
}

fn main() {
    let contents = match fs::read_to_string("./inputs/game1.txt") {
        Err(why) => panic!("Couldn't read file because {}", why),
        Ok(content) => content
    };
    
    let mut moves = contents.split_whitespace();
}
