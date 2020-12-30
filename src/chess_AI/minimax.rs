use crate::{BoardState, PieceColor, PossibleMove, PossibleMoveIter};
use std::cmp::max;
use std::cmp::min;

use super::{full_evaluate, Score};

type Depth = u8;
const DEPTH: Depth = 4;

pub fn minimax(board_state: &BoardState) -> PossibleMove {
    // computer always plays black
    let moves = PossibleMoveIter::find_possible_moves(board_state, PieceColor::Black);

    let mut favourite_move = None;
    let mut favourite_move_score = Score::MIN;
    for possible_move in moves {
        let mut new_board_state = board_state.clone();
        possible_move.play_move(&mut new_board_state, PieceColor::Black);
        let child_score = min_move(DEPTH - 1, &new_board_state);
        if let Some(child_score) = child_score {
            if child_score > favourite_move_score || favourite_move.is_none() {
                favourite_move_score = child_score;
                favourite_move = Some(possible_move);
            }
        }
    }
    favourite_move.unwrap()
}

fn max_move(depth: Depth, board_state: &BoardState) -> Option<Score> {
    if depth == 0 {
        return Some(full_evaluate(board_state));
    }
    let moves = PossibleMoveIter::find_possible_moves(board_state, PieceColor::Black);
    if moves.can_take_king() {
        return None;
    }
    let mut score = Score::MIN;
    for possible_move in moves {
        let mut new_board_state = board_state.clone();
        possible_move.play_move(&mut new_board_state, PieceColor::Black);
        let child_score = min_move(depth - 1, &new_board_state);
        if let Some(child_score) = child_score {
            score = max(score, child_score);
        }
    }
    Some(score)
}
fn min_move(depth: Depth, board_state: &BoardState) -> Option<Score> {
    if depth == 0 {
        return Some(full_evaluate(board_state));
    }
    let moves = PossibleMoveIter::find_possible_moves(board_state, PieceColor::White);
    if moves.can_take_king() {
        return None;
    }
    let mut score = Score::MAX;
    for possible_move in moves {
        let mut new_board_state = board_state.clone();
        possible_move.play_move(&mut new_board_state, PieceColor::White);
        let child_score = max_move(depth - 1, &new_board_state);
        if let Some(child_score) = child_score {
            score = min(score, child_score);
        }
    }
    Some(score)
}
