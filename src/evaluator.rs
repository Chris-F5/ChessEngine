use crate::{BoardPosition, BoardState, PieceColor, PieceType};

pub type Score = i16;

pub struct Evaluator();

impl Evaluator {
    pub fn quick_evaluate(board_state: &BoardState) -> Score {
        Self::full_evaluate(board_state)
    }
    pub fn full_evaluate(board_state: &BoardState) -> Score {
        let mut score = 0;
        for y in 0..8 {
            for x in 0..8 {
                let piece_option = board_state.get(BoardPosition::new(x, y));
                if let Some(piece) = piece_option {
                    let color_multiplier = if piece.color == PieceColor::White {
                        -1
                    } else {
                        1
                    };
                    match piece.piece_type {
                        PieceType::Pawn => score += 1 * color_multiplier,
                        PieceType::Knight => score += 3 * color_multiplier,
                        PieceType::Bishop => score += 3 * color_multiplier,
                        PieceType::Rook => score += 5 * color_multiplier,
                        PieceType::Queen => score += 9 * color_multiplier,
                        _ => (),
                    }
                }
            }
        }
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
}
