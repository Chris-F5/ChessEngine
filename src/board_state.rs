use std::fmt;

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
    pub fn to_text(&self) -> String {
        let file = match self.x {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => panic!("invalid board position"),
        };
        let rank = match self.y {
            0 => '1',
            1 => '2',
            2 => '3',
            3 => '4',
            4 => '5',
            5 => '6',
            6 => '7',
            7 => '8',
            _ => panic!("invalid board position"),
        };
        format!("{}{}", file, rank).to_string()
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
    pub fn from_fen_char(c: char) -> Piece {
        match c {
            'p' => Piece::new(PieceColor::Black, PieceType::Pawn),
            'n' => Piece::new(PieceColor::Black, PieceType::Knight),
            'b' => Piece::new(PieceColor::Black, PieceType::Bishop),
            'r' => Piece::new(PieceColor::Black, PieceType::Rook),
            'q' => Piece::new(PieceColor::Black, PieceType::Queen),
            'k' => Piece::new(PieceColor::Black, PieceType::King),
            'P' => Piece::new(PieceColor::White, PieceType::Pawn),
            'N' => Piece::new(PieceColor::White, PieceType::Knight),
            'B' => Piece::new(PieceColor::White, PieceType::Bishop),
            'R' => Piece::new(PieceColor::White, PieceType::Rook),
            'Q' => Piece::new(PieceColor::White, PieceType::Queen),
            'K' => Piece::new(PieceColor::White, PieceType::King),
            _ => panic!("cant parse char '{}' as piece", c),
        }
    }
    pub fn to_unicode_char(&self) -> char {
        match self.color {
            PieceColor::White => match self.piece_type {
                PieceType::Pawn => '♙',
                PieceType::Knight => '♘',
                PieceType::Bishop => '♗',
                PieceType::Rook => '♖',
                PieceType::Queen => '♕',
                PieceType::King => '♔',
            },
            PieceColor::Black => match self.piece_type {
                PieceType::Pawn => '♟',
                PieceType::Knight => '♞',
                PieceType::Bishop => '♝',
                PieceType::Rook => '♜',
                PieceType::Queen => '♛',
                PieceType::King => '♚',
            },
        }
    }
    pub fn to_fen_char(&self) -> char {
        match self.color {
            PieceColor::White => match self.piece_type {
                PieceType::Pawn => 'P',
                PieceType::Knight => 'N',
                PieceType::Bishop => 'B',
                PieceType::Rook => 'R',
                PieceType::Queen => 'Q',
                PieceType::King => 'K',
            },
            PieceColor::Black => match self.piece_type {
                PieceType::Pawn => 'p',
                PieceType::Knight => 'n',
                PieceType::Bishop => 'b',
                PieceType::Rook => 'r',
                PieceType::Queen => 'q',
                PieceType::King => 'k',
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
    pub fn to_fen(&self) -> String {
        let mut board_string = String::with_capacity(62);
        for y in (0..8).rev() {
            for x in 0..8 {
                let piece_option = self.get(BoardPosition::new(x, y));
                board_string.push(match piece_option {
                    None => '1',
                    Some(piece) => piece.to_fen_char(),
                })
            }
            if y != 0 {
                board_string.push('/');
            }
        }
        format!(
            "{} {} - {} 0 0",
            board_string,
            match self.color_turn {
                PieceColor::White => "w",
                PieceColor::Black => "b",
            },
            if self.en_passant_colunm < 8 {
                match self.color_turn {
                    PieceColor::White => BoardPosition::new(self.en_passant_colunm, 5).to_text(),
                    PieceColor::Black => BoardPosition::new(self.en_passant_colunm, 2).to_text(),
                }
            } else {
                "-".to_string()
            }
        )
        .to_string()
    }
    pub fn from_fen(fen: &str) -> BoardState {
        let mut space_splitter = fen.split_whitespace();
        let pieces_str = space_splitter.next().unwrap();
        let turn_str = space_splitter.next().unwrap();
        let castling_str = space_splitter.next().unwrap();
        let en_passant_str = space_splitter.next().unwrap();
        // fullmove and halfmove number are ignored becuase they arent used
        let mut ranks_str = pieces_str.split("/");
        let mut pieces = [[None::<Piece>; 8]; 8];
        for y in (0..8).rev() {
            let mut rank_chars = ranks_str.next().unwrap().chars();
            let mut x = 0;
            loop {
                let piece_char_option = rank_chars.next();
                if let Some(piece_char) = piece_char_option {
                    let piece_number_option = piece_char.to_string().parse::<u8>();
                    if let Ok(n) = piece_number_option {
                        x += n;
                    } else {
                        pieces[y as usize][x as usize] = Some(Piece::from_fen_char(piece_char));
                        x += 1;
                    }
                } else {
                    break;
                }
            }
        }
        let color_turn = match turn_str {
            "w" => PieceColor::White,
            "b" => PieceColor::Black,
            _ => panic!("invalid fen color turn"),
        };
        let white_king_castle = castling_str.contains("K");
        let white_queen_castle = castling_str.contains("Q");
        let black_king_castle = castling_str.contains("k");
        let black_queen_castle = castling_str.contains("q");
        let en_passant_colunm = match en_passant_str {
            "-" => 55,
            _ => BoardPosition::from_text(en_passant_str).x,
        };
        BoardState {
            pieces,
            color_turn,
            white_king_castle,
            white_queen_castle,
            black_king_castle,
            black_queen_castle,
            en_passant_colunm,
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
                    Some(p) => board_string.push(p.to_unicode_char()),
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
        BoardState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}
