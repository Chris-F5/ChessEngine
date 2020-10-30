use std::fmt;
use std::string;

#[derive(Copy, Clone)]
pub struct BoardPosition {
    x: u8,
    y: u8,
}

impl BoardPosition {
    pub fn new(x: u8, y: u8) -> Self {
        BoardPosition { x: x, y: y }
    }
}

pub enum PieceColor {
    White,
    Black,
}

impl string::ToString for PieceColor {
    fn to_string(&self) -> String {
        match self {
            PieceColor::White => String::from("W"),
            PieceColor::Black => String::from("B"),
        }
    }
}

pub enum PieceType {
    Pawn { en_passant: bool },
    Bishop,
    Knight,
    Rook { moved: bool },
    Queen,
    King { moved: bool },
}

impl string::ToString for PieceType {
    fn to_string(&self) -> String {
        match self {
            PieceType::Pawn { en_passant: false } => String::from("i"),
            PieceType::Pawn { en_passant: true } => String::from("ï"),
            PieceType::Bishop => String::from("Ø"),
            PieceType::Knight => String::from("F"),
            PieceType::Rook { moved: false } => String::from("T"),
            PieceType::Rook { moved: true } => String::from("I"),
            PieceType::Queen => String::from("Q"),
            PieceType::King { moved: false } => String::from("K"),
            PieceType::King { moved: true } => String::from("k"),
        }
    }
}

pub struct Piece {
    pub piece_type: PieceType,
    pub color: PieceColor,
}

impl string::ToString for Piece {
    fn to_string(&self) -> String {
        let mut string = self.color.to_string();
        string.push_str(&*self.piece_type.to_string());
        string
    }
}

pub struct BoardState {
    pieces: [[Option<Piece>; 8]; 8],
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

        for y in 0..8 {
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
            if y != 7 {
                board_string.push('\n');
            }
        }

        write!(f, "{}", board_string)
    }
}

impl Default for BoardState {
    fn default() -> BoardState {
        BoardState {
            pieces: {
                [
                    [
                        Some(Piece {
                            piece_type: PieceType::Rook { moved: false },
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
                            piece_type: PieceType::King { moved: false },
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
                            piece_type: PieceType::Rook { moved: false },
                            color: PieceColor::Black,
                        }),
                    ],
                    [
                        Some(Piece {
                            piece_type: PieceType::Pawn { en_passant: false },
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn { en_passant: false },
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn { en_passant: false },
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn { en_passant: false },
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn { en_passant: false },
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn { en_passant: false },
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn { en_passant: false },
                            color: PieceColor::Black,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn { en_passant: false },
                            color: PieceColor::Black,
                        }),
                    ],
                    [None, None, None, None, None, None, None, None],
                    [None, None, None, None, None, None, None, None],
                    [None, None, None, None, None, None, None, None],
                    [None, None, None, None, None, None, None, None],
                    [
                        Some(Piece {
                            piece_type: PieceType::Pawn { en_passant: false },
                            color: PieceColor::White,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn { en_passant: false },
                            color: PieceColor::White,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn { en_passant: false },
                            color: PieceColor::White,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn { en_passant: false },
                            color: PieceColor::White,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn { en_passant: false },
                            color: PieceColor::White,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn { en_passant: false },
                            color: PieceColor::White,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn { en_passant: false },
                            color: PieceColor::White,
                        }),
                        Some(Piece {
                            piece_type: PieceType::Pawn { en_passant: false },
                            color: PieceColor::White,
                        }),
                    ],
                    [
                        Some(Piece {
                            piece_type: PieceType::Rook { moved: false },
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
                            piece_type: PieceType::King { moved: false },
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
                            piece_type: PieceType::Rook { moved: false },
                            color: PieceColor::White,
                        }),
                    ],
                ]
            },
        }
    }
}
