use std::fmt;
use std::string;

pub trait Capturable {
    fn can_take(&self, color: PieceColor) -> bool;
}

impl Capturable for Option<Piece> {
    fn can_take(&self, color: PieceColor) -> bool {
        if let Some(piece) = self {
            piece.color != color
        } else {
            false
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct BoardPosition {
    pub x: u8,
    pub y: u8,
}

impl fmt::Debug for BoardPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = match self.x {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => panic!("cant debug invalid board position"),
        };
        let y = match self.y {
            0 => '1',
            1 => '2',
            2 => '3',
            3 => '4',
            4 => '5',
            5 => '6',
            6 => '7',
            7 => '8',
            _ => panic!("cant debug invalid board position"),
        };
        write!(f, "{}{}", x, y)
    }
}

impl BoardPosition {
    pub fn new(x: u8, y: u8) -> BoardPosition {
        BoardPosition { x: x, y: y }
    }
    pub fn from_text(string: &str) -> BoardPosition {
        assert!(string.len() == 2);
        let x = match string.chars().nth(0).unwrap() {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => panic!("invalid board position string"),
        };
        let y = match string.chars().nth(1).unwrap() {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => panic!("invalid board position string"),
        };
        BoardPosition { x: x, y: y }
    }
    pub fn bound_check(&self) -> bool {
        self.x <= 7 && self.y <= 7
    }
    pub fn x_bound_check(&self) -> bool {
        self.x <= 7
    }
    pub fn y_bound_check(&self) -> bool {
        self.y <= 7
    }
    pub fn directional_ofset(&self, x: i8, y: i8, color: PieceColor) -> BoardPosition {
        let y_dir = if color == PieceColor::White { 1 } else { -1 };
        BoardPosition::new((self.x as i8 + x) as u8, (self.y as i8 + (y * y_dir)) as u8)
    }
    pub fn nondirectional_ofset(&self, x: i8, y: i8) -> BoardPosition {
        BoardPosition::new((self.x as i8 + x) as u8, (self.y as i8 + y) as u8)
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

#[derive(Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq)]
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
        match self.color {
            PieceColor::White => match self.piece_type {
                PieceType::Pawn => String::from("P"),
                PieceType::Bishop => String::from("B"),
                PieceType::Knight => String::from("N"),
                PieceType::Rook => String::from("R"),
                PieceType::Queen => String::from("Q"),
                PieceType::King => String::from("K"),
            },
            PieceColor::Black => match self.piece_type {
                PieceType::Pawn => String::from("p"),
                PieceType::Bishop => String::from("b"),
                PieceType::Knight => String::from("n"),
                PieceType::Rook => String::from("r"),
                PieceType::Queen => String::from("q"),
                PieceType::King => String::from("k"),
            },
        }
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
    pub color_turn: PieceColor,
}

impl PartialEq for BoardState {
    fn eq(&self, other: &BoardState) -> bool {
        for y in 0..8 {
            for x in 0..8 {
                if self.get(BoardPosition::new(x, y)) != other.get(BoardPosition::new(x, y)) {
                    return false;
                }
            }
        }

        if self.white_king_castle != other.white_king_castle
            || self.black_king_castle != other.black_king_castle
            || self.white_queen_castle != other.white_queen_castle
            || self.color_turn != other.color_turn
        {
            return false;
        }
        if self.en_passant_colunm < 8 && self.en_passant_colunm != other.en_passant_colunm {
            return false;
        }

        true
    }
}

impl BoardState {
    pub fn get(&self, pos: BoardPosition) -> &Option<Piece> {
        &self.pieces[pos.y as usize][pos.x as usize]
    }
    pub fn get_mut(&mut self, pos: BoardPosition) -> &mut Option<Piece> {
        &mut self.pieces[pos.y as usize][pos.x as usize]
    }
    pub fn alpha_to_move(&self) -> bool {
        match self.color_turn {
            PieceColor::White => false,
            PieceColor::Black => true,
        }
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
                    None => board_string.push_str("-"),
                }
                if x != 7 {
                    board_string.push(' ');
                }
            }
            if real_y != 7 {
                board_string.push('\n');
            }
        }

        write!(
            f,
            "pieces: \n{}\ncolor_turn: {:?}\nen_passant_colunm: {}\ncan_white_castle: {} {}\ncan_black_castle {} {} ",
            board_string,
            self.color_turn,
            self.en_passant_colunm,
            self.white_king_castle,
            self.white_queen_castle,
            self.black_king_castle,
            self.black_queen_castle
        )
    }
}

impl Default for BoardState {
    fn default() -> BoardState {
        BoardState {
            color_turn: PieceColor::White,
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
