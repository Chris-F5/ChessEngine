use std::fmt;
use std::string;

#[derive(Copy, Clone, PartialEq)]
pub struct BoardPosition {
    pub x: u8,
    pub y: u8,
}

impl BoardPosition {
    pub fn new(x: u8, y: u8) -> Self {
        BoardPosition { x: x, y: y }
    }
    pub fn is_valid(&self) -> bool {
        self.x <= 7 && self.y <= 7
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    pub fn opposite_color(&self) -> PieceColor {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}

impl string::ToString for PieceColor {
    fn to_string(&self) -> String {
        match self {
            PieceColor::White => String::from("W"),
            PieceColor::Black => String::from("B"),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

impl string::ToString for PieceType {
    fn to_string(&self) -> String {
        match self {
            PieceType::Pawn => String::from("i"),
            PieceType::Bishop => String::from("x"),
            PieceType::Knight => String::from("F"),
            PieceType::Rook => String::from("I"),
            PieceType::Queen => String::from("Q"),
            PieceType::King => String::from("K"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PieceColor,
}

impl Piece {
    pub fn new(color: PieceColor, piece_type: PieceType) -> Piece {
        Piece { color, piece_type }
    }
}

impl string::ToString for Piece {
    fn to_string(&self) -> String {
        let mut string = self.color.to_string();
        string.push_str(&*self.piece_type.to_string());
        string
    }
}

#[derive(Clone)]
pub struct BoardState {
    pieces: [[Option<Piece>; 8]; 8],
    pub white_king_castle: bool,
    pub white_queen_castle: bool,
    pub black_king_castle: bool,
    pub black_queen_castle: bool,
    pub en_passant_colunm: u8,
}

impl BoardState {
    pub fn get(&self, pos: BoardPosition) -> &Option<Piece> {
        &self.pieces[pos.y as usize][pos.x as usize]
    }
    pub fn get_mut(&mut self, pos: BoardPosition) -> &mut Option<Piece> {
        &mut self.pieces[pos.y as usize][pos.x as usize]
    }
}

impl fmt::Debug for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_string = String::new();

        for real_y in 0..8 {
            let y = 7 - real_y;
            for x in 0..8 {
                let piece = self.get(BoardPosition::new(x, y));
                match piece {
                    Some(p) => board_string.push_str(&*p.to_string()),
                    None => board_string.push_str("--"),
                }
                if x != 7 {
                    board_string.push(' ');
                }
            }
            if real_y != 7 {
                board_string.push('\n');
            }
        }

        write!(f, "{}", board_string)
    }
}

impl Default for BoardState {
    fn default() -> BoardState {
        BoardState {
            en_passant_colunm: 55,
            white_king_castle: false,
            white_queen_castle: false,
            black_king_castle: false,
            black_queen_castle: false,
            pieces: {
                [
                    [
                        Some(Piece {
                            piece_type: PieceType::Rook,
                            color: PieceColor::White,
                        }),
                        None,
                        None,
                        None,
                        Some(Piece {
                            piece_type: PieceType::King,
                            color: PieceColor::White,
                        }),
                        None,
                        None,
                        Some(Piece {
                            piece_type: PieceType::Rook,
                            color: PieceColor::White,
                        }),
                    ],
                    [None, None, None, None, None, None, None, None],
                    [None, None, None, None, None, None, None, None],
                    [None, None, None, None, None, None, None, None],
                    [None, None, None, None, None, None, None, None],
                    [None, None, None, None, None, None, None, None],
                    [None, None, None, None, None, None, None, None],
                    [
                        Some(Piece {
                            piece_type: PieceType::Rook,
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Knight,
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Bishop,
                            color: PieceColor::Black,
                        }),
                        None,
                        Some(Piece {
                            piece_type: PieceType::King,
                            color: PieceColor::Black,
                        }),
                        None,
                        None,
                        Some(Piece {
                            piece_type: PieceType::Rook,
                            color: PieceColor::Black,
                        }),
                    ],
                ]
            },
        }
    }
}

/*
impl Default for BoardState {
    fn default() -> BoardState {
        BoardState {
            en_passant_colunm: 55,
            white_king_castle: true,
            white_queen_castle: true,
            black_king_castle: true,
            black_queen_castle: true,
            pieces: {
                [
                    [
                        Some(Piece {
                            piece_type: PieceType::Rook,
                            color: PieceColor::White,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Knight,
                            color: PieceColor::White,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Bishop,
                            color: PieceColor::White,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Queen,
                            color: PieceColor::White,
                        }),
                        Some(Piece {
                            piece_type: PieceType::King,
                            color: PieceColor::White,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Bishop,
                            color: PieceColor::White,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Knight,
                            color: PieceColor::White,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Rook,
                            color: PieceColor::White,
                        }),
                    ],
                    [
                        Some(Piece {
                            piece_type: PieceType::Pawn,
                            color: PieceColor::White,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn,
                            color: PieceColor::White,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn,
                            color: PieceColor::White,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn,
                            color: PieceColor::White,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn,
                            color: PieceColor::White,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn,
                            color: PieceColor::White,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn,
                            color: PieceColor::White,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn,
                            color: PieceColor::White,
                        }),
                    ],
                    [None, None, None, None, None, None, None, None],
                    [None, None, None, None, None, None, None, None],
                    [None, None, None, None, None, None, None, None],
                    [None, None, None, None, None, None, None, None],
                    [
                        Some(Piece {
                            piece_type: PieceType::Pawn,
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn,
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn,
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn,
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn,
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn,
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn,
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn,
                            color: PieceColor::Black,
                        }),
                    ],
                    [
                        Some(Piece {
                            piece_type: PieceType::Rook,
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Knight,
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Bishop,
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Queen,
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::King,
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Bishop,
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Knight,
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Rook,
                            color: PieceColor::Black,
                        }),
                    ],
                ]
            },
        }
    }
}
*/
