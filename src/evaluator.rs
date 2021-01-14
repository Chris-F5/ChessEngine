use crate::{BoardPosition, BoardState, PieceColor, PieceType};

const PAWN_VALUE: Score = 100;
const PAWN_SQUARE_TABLE: [[Score; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [50, 50, 50, 50, 50, 50, 50, 50],
    [10, 10, 20, 30, 30, 20, 10, 10],
    [5, 5, 10, 25, 25, 10, 5, 5],
    [0, 0, 0, 20, 20, 0, 0, 0],
    [5, -5, -10, 0, 0, -10, -5, 5],
    [5, 10, 10, -20, -20, 10, 10, 5],
    [0, 0, 0, 0, 0, 0, 0, 0],
];

const KNIGHT_VALUE: Score = 320;
const KNIGHT_SQUARE_TABLE: [[Score; 8]; 8] = [
    [-50, -40, -30, -30, -30, -30, -40, -50],
    [-40, -20, 0, 0, 0, 0, -20, -40],
    [-30, 0, 10, 15, 15, 10, 0, -30],
    [-30, 5, 15, 20, 20, 15, 5, -30],
    [-30, 0, 15, 20, 20, 15, 0, -30],
    [-30, 5, 10, 15, 15, 10, 5, -30],
    [-40, -20, 0, 5, 5, 0, -20, -40],
    [-50, -40, -30, -30, -30, -30, -40, -50],
];
const BISHOP_VALUE: Score = 330;
const BISHOP_SQUARE_TABLE: [[Score; 8]; 8] = [
    [-20, -10, -10, -10, -10, -10, -10, -20],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-10, 0, 5, 10, 10, 5, 0, -10],
    [-10, 5, 5, 10, 10, 5, 5, -10],
    [-10, 0, 10, 10, 10, 10, 0, -10],
    [-10, 10, 10, 10, 10, 10, 10, -10],
    [-10, 5, 0, 0, 0, 0, 5, -10],
    [-20, -10, -10, -10, -10, -10, -10, -20],
];
const ROOK_VALUE: Score = 500;
const ROOK_SQUARE_TABLE: [[Score; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [5, 10, 10, 10, 10, 10, 10, 5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [0, 0, 0, 5, 5, 0, 0, 0],
];
const QUEEN_VALUE: Score = 900;
const QUEEN_SQUARE_TABLE: [[Score; 8]; 8] = [
    [-20, -10, -10, -5, -5, -10, -10, -20],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-10, 0, 5, 5, 5, 5, 0, -10],
    [-5, 0, 5, 5, 5, 5, 0, -5],
    [0, 0, 5, 5, 5, 5, 0, -5],
    [-10, 5, 5, 5, 5, 5, 0, -10],
    [-10, 0, 5, 0, 0, 0, 0, -10],
    [-20, -10, -10, -5, -5, -10, -10, -20],
];
const KING_SQUARE_TABLE: [[Score; 8]; 8] = [
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-20, -30, -30, -40, -40, -30, -30, -20],
    [-10, -20, -20, -20, -20, -20, -20, -10],
    [20, 20, 0, 0, 0, 0, 20, 20],
    [20, 30, 10, 0, 0, 10, 30, 20],
];

pub type Score = i16;

pub struct Evaluator();

impl Evaluator {
    pub fn quick_evaluate(board_state: &BoardState) -> Score {
        let mut score = 0;
        score += Self::square_tables(board_state);
        score
    }
    pub fn full_evaluate(board_state: &BoardState) -> Score {
        let mut score = 0;
        score += Self::square_tables(board_state);
        score
    }
    pub fn score_for_draw() -> Score {
        0
    }
    pub fn score_for_checkmate(color: PieceColor) -> Score {
        match color {
            PieceColor::White => Score::MIN,
            PieceColor::Black => Score::MAX,
        }
    }

    fn square_tables(board_state: &BoardState) -> Score {
        let mut score = 0;
        for y in 0..8 {
            for x in 0..8 {
                let piece_option = board_state.get(BoardPosition::new(x, y));
                if let Some(piece) = piece_option {
                    match piece.color {
                        PieceColor::White => match piece.piece_type {
                            PieceType::Pawn => {
                                score -= PAWN_VALUE
                                    + PAWN_SQUARE_TABLE[inverse_axis(y) as usize][x as usize]
                            }
                            PieceType::Knight => {
                                score -= KNIGHT_VALUE
                                    + KNIGHT_SQUARE_TABLE[inverse_axis(y) as usize][x as usize]
                            }
                            PieceType::Bishop => {
                                score -= BISHOP_VALUE
                                    + BISHOP_SQUARE_TABLE[inverse_axis(y) as usize][x as usize]
                            }
                            PieceType::Rook => {
                                score -= ROOK_VALUE
                                    + ROOK_SQUARE_TABLE[inverse_axis(y) as usize][x as usize]
                            }
                            PieceType::Queen => {
                                score -= QUEEN_VALUE
                                    + QUEEN_SQUARE_TABLE[inverse_axis(y) as usize][x as usize]
                            }
                            PieceType::King => {
                                score -= KING_SQUARE_TABLE[inverse_axis(y) as usize][x as usize]
                            }
                        },
                        PieceColor::Black => match piece.piece_type {
                            PieceType::Pawn => {
                                score += PAWN_VALUE + PAWN_SQUARE_TABLE[y as usize][x as usize]
                            }
                            PieceType::Knight => {
                                score += KNIGHT_VALUE + KNIGHT_SQUARE_TABLE[y as usize][x as usize]
                            }
                            PieceType::Bishop => {
                                score += BISHOP_VALUE + BISHOP_SQUARE_TABLE[y as usize][x as usize]
                            }
                            PieceType::Rook => {
                                score += ROOK_VALUE + ROOK_SQUARE_TABLE[y as usize][x as usize]
                            }
                            PieceType::Queen => {
                                score += QUEEN_VALUE + QUEEN_SQUARE_TABLE[y as usize][x as usize]
                            }
                            PieceType::King => score += KING_SQUARE_TABLE[y as usize][x as usize],
                        },
                    }
                }
            }
        }
        score
    }
}

fn inverse_axis(n: u8) -> u8 {
    7 - n
}
