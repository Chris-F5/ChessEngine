use crate::{BoardPosition, BoardState, PieceColor, PieceType};

pub type Score = i16;

pub fn full_evaluate(board_state: &BoardState) -> Score {
    let mut score = 0;
    for y in 0..8 {
        for x in 0..8 {
            let piece_option = board_state.get(BoardPosition::new(x, y));
            if let Some(piece) = piece_option {
                let color_multiplier = if let PieceColor::White = piece.color {
                    -1
                } else {
                    1
                };
                match piece.piece_type {
                    PieceType::Pawn { en_passant: _ } => score += 1 * color_multiplier,
                    PieceType::Knight => score += 3 * color_multiplier,
                    PieceType::Bishop => score += 3 * color_multiplier,
                    PieceType::Rook { moved: _ } => score += 5 * color_multiplier,
                    PieceType::Queen => score += 9 * color_multiplier,
                    _ => (),
                }
            }
        }
    }
    score
}
