/*pub trait GameState: Clone {
    type Action;
    fn apply_action(&mut self, action: &Self::Action);
    fn alpha_to_move(&self) -> bool;
}

pub trait RuleSet<State>
where
    State: GameState,
{
    fn find_possible_actions(game_state: &State) -> Vec<State::Action>;
}

pub trait Evaluator<GameState> {
    type Score: PartialOrd + Copy;
    fn full_evaluate(game_state: &GameState) -> Self::Score;
    fn score_for_cant_move() -> Self::Score;
}*/

use crate::in_check;
use crate::{find_legal_actions, Action, BoardState, Evaluator, Score};

pub fn find_move_with_minimax(board_state: &BoardState, depth: u8) -> Option<Action> {
    assert!(depth != 0, "root depth for minimax cant be 0");
    let possible_actions = find_legal_actions(board_state);
    if possible_actions.is_empty() {
        return None;
    }
    if board_state.alpha_to_move() {
        let mut best_score = None;
        let mut best_action = None;
        for action in possible_actions {
            let mut new_board_state = board_state.clone();
            action.play_move(&mut new_board_state);
            let this_score = minimax(&new_board_state, depth - 1);
            if best_score == None || this_score > best_score.unwrap() {
                best_score = Some(this_score);
                best_action = Some(action);
            }
        }
        return best_action;
    } else {
        let mut best_score = None;
        let mut best_action = None;
        for action in possible_actions {
            let mut new_board_state = board_state.clone();
            action.play_move(&mut new_board_state);
            let this_score = minimax(&new_board_state, depth - 1);
            if best_score == None || this_score < best_score.unwrap() {
                best_score = Some(this_score);
                best_action = Some(action);
            }
        }
        return best_action;
    }
}

fn minimax(game_state: &BoardState, depth: u8) -> Score {
    if depth == 0 {
        return Evaluator::full_evaluate(&game_state);
    }

    let possible_actions = find_legal_actions(game_state);

    if possible_actions.is_empty() {
        if in_check(game_state) {
            return Evaluator::score_for_checkmate(game_state.color_turn);
        } else {
            return Evaluator::score_for_cant_move();
        }
    }

    if game_state.alpha_to_move() {
        let mut best_score = None;
        for action in possible_actions {
            let mut new_board_state = game_state.clone();
            action.play_move(&mut new_board_state);
            let this_score = minimax(&new_board_state, depth - 1);
            if best_score == None || this_score > best_score.unwrap() {
                best_score = Some(this_score);
            }
        }
        return best_score.unwrap();
    } else {
        let mut best_score = None;
        for action in possible_actions {
            let mut new_board_state = game_state.clone();
            action.play_move(&mut new_board_state);
            let this_score = minimax(&new_board_state, depth - 1);
            if best_score == None || this_score < best_score.unwrap() {
                best_score = Some(this_score);
            }
        }
        return best_score.unwrap();
    }
}
