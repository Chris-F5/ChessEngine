use crate::in_check;
use crate::{find_legal_actions, Action, BoardState, Evaluator, GameEndState, PieceColor, Score};

pub fn find_move_with_minimax(board_state: &BoardState, depth: u8) -> Option<Action> {
    assert!(depth != 0, "root depth for minimax cant be 0");
    let beta = Evaluator::score_for_checkmate(PieceColor::Black);
    let mut alpha = Evaluator::score_for_checkmate(PieceColor::White);
    let mut best_move = None;
    let legal_actions = find_legal_actions(&board_state, false).0;
    // TODO: sort actions based on quick evaluate
    for action in legal_actions {
        let mut new_board_state = board_state.clone();
        action.play_move(&mut new_board_state);
        // TODO: varable depth
        let score = min(&new_board_state, depth - 1, alpha, beta);
        if best_move.is_none() || score > alpha {
            alpha = score;
            best_move = Some(action);
        }
    }
    best_move
}

fn min(board_state: &BoardState, depth: u8, alpha: Score, beta: Score) -> Score {
    let mut beta = beta;
    if depth == 0 {
        return Evaluator::full_evaluate(&board_state);
    }

    let (legal_actions, game_end_option) = if depth == 1 {
        find_legal_actions(&board_state, true)
    } else {
        find_legal_actions(&board_state, false)
    };
    if let Some(game_end) = game_end_option {
        return match game_end {
            GameEndState::Draw => Evaluator::score_for_draw(),
            GameEndState::Win(color) => Evaluator::score_for_checkmate(color),
        };
    }
    // TODO: sort actions based on quick evaluate
    for action in legal_actions {
        let mut new_board_state = board_state.clone();
        action.play_move(&mut new_board_state);
        // TODO: varable depth
        let score = max(&new_board_state, depth - 1, alpha, beta);
        if score <= alpha {
            return alpha;
        }
        if score < beta {
            beta = score;
        }
    }
    return beta;
}

fn max(board_state: &BoardState, depth: u8, alpha: Score, beta: Score) -> Score {
    let mut alpha = alpha;
    if depth == 0 {
        return Evaluator::full_evaluate(&board_state);
    }
    let (legal_actions, game_end_option) = if depth == 1 {
        find_legal_actions(&board_state, true)
    } else {
        find_legal_actions(&board_state, false)
    };
    if let Some(game_end) = game_end_option {
        return match game_end {
            GameEndState::Draw => Evaluator::score_for_draw(),
            GameEndState::Win(color) => Evaluator::score_for_checkmate(color),
        };
    }
    // TODO: sort actions based on quick evaluate
    for action in legal_actions {
        let mut new_board_state = board_state.clone();
        action.play_move(&mut new_board_state);
        // TODO: varable depth
        let score = min(&new_board_state, depth - 1, alpha, beta);
        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }
    return alpha;
}
