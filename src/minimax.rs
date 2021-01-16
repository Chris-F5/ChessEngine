use crate::{find_legal_actions, Action, BoardState, Evaluator, GameEndState, PieceColor, Score};

pub fn find_move_with_minimax<F>(
    board_state: &BoardState,
    depth: u8,
    progress_update: &mut F,
) -> Option<Action>
where
    F: FnMut(f32),
{
    assert!(depth != 0, "root depth for minimax cant be 0");
    let beta = Evaluator::score_for_checkmate(PieceColor::Black);
    let mut alpha = Evaluator::score_for_checkmate(PieceColor::White);
    let mut best_move = None;
    let legal_actions = find_legal_actions(&board_state, false).0;
    let length = legal_actions.len();
    let mut i = 0;
    for action in legal_actions {
        println!("{}/{}", i, length);
        progress_update(i as f32 / length as f32);

        let mut new_board_state = board_state.clone();
        action.play_move(&mut new_board_state);
        // TODO: varable depth
        let score = min(&new_board_state, depth - 1, alpha, beta);
        if best_move.is_none() || score > alpha {
            alpha = score;
            best_move = Some(action);
        }
        i += 1;
    }
    progress_update(1.0);
    best_move
}

fn min(board_state: &BoardState, depth: u8, alpha: Score, beta: Score) -> Score {
    let mut beta = beta;
    let (legal_actions, game_end_option) = if depth == 1 {
        find_legal_actions(&board_state, true)
    } else {
        find_legal_actions(&board_state, false)
    };
    if let Some(game_end) = game_end_option {
        return match game_end {
            GameEndState::Draw => Evaluator::score_for_draw(),
            GameEndState::Win(PieceColor::White) => {
                Evaluator::score_for_checkmate(PieceColor::White)
            }
            GameEndState::Win(PieceColor::Black) => {
                // Prioritise nearer checkmates
                (Evaluator::score_for_checkmate(PieceColor::Black) - 100) + depth as i16
            }
        };
    }

    if depth == 0 {
        return Evaluator::full_evaluate(&board_state);
    }

    let mut possible_board_states: Vec<(BoardState, Score)> =
        Vec::with_capacity(legal_actions.len());
    for i in 0..legal_actions.len() {
        possible_board_states.push((board_state.clone(), 0));
        legal_actions[i].play_move(&mut possible_board_states[i].0);
        possible_board_states[i].1 = Evaluator::quick_evaluate(&possible_board_states[i].0);
    }
    possible_board_states.sort_by(|a, b| a.1.cmp(&b.1));
    let mut i = 0;
    for new_board_state in possible_board_states {
        let depth_loss = depth_loss(i, depth);
        let score = max(&new_board_state.0, depth - depth_loss, alpha, beta);
        if score <= alpha {
            return alpha;
        }
        if score < beta {
            beta = score;
        }
        i += 1;
    }
    return beta;
}

fn max(board_state: &BoardState, depth: u8, alpha: Score, beta: Score) -> Score {
    let mut alpha = alpha;
    let (legal_actions, game_end_option) = if depth == 1 {
        find_legal_actions(&board_state, true)
    } else {
        find_legal_actions(&board_state, false)
    };
    if let Some(game_end) = game_end_option {
        return match game_end {
            GameEndState::Draw => Evaluator::score_for_draw(),
            GameEndState::Win(PieceColor::White) => {
                Evaluator::score_for_checkmate(PieceColor::White)
            }
            GameEndState::Win(PieceColor::Black) => {
                // Prioritise nearer checkmates
                (Evaluator::score_for_checkmate(PieceColor::Black) - 100) + depth as i16
            }
        };
    }
    if depth == 0 {
        return Evaluator::full_evaluate(&board_state);
    }
    let mut possible_board_states: Vec<(BoardState, Score)> =
        Vec::with_capacity(legal_actions.len());
    for i in 0..legal_actions.len() {
        possible_board_states.push((board_state.clone(), 0));
        legal_actions[i].play_move(&mut possible_board_states[i].0);
        possible_board_states[i].1 = Evaluator::quick_evaluate(&possible_board_states[i].0);
    }
    possible_board_states.sort_by(|a, b| b.1.cmp(&a.1));

    let mut i = 0;
    for new_board_state in possible_board_states {
        let depth_loss = depth_loss(i, depth);
        let score = min(&new_board_state.0, depth - depth_loss, alpha, beta);
        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
        i += 1;
    }
    return alpha;
}
fn depth_loss(i: u16, depth: u8) -> u8 {
    if i < 3 || depth < 2 {
        1
    } else if i < 5 || depth < 3 {
        2
    } else {
        3
    }
}
