use super::{Evaluator, Score};
use crate::{find_legal_actions, Action, BoardState, GameEndState};

pub struct Minimax<'a> {
    max_depth: u8,
    evaluator: &'a Evaluator,
}

impl<'a> Minimax<'a> {
    pub fn new(depth: u8, evaluator: &Evaluator) -> Minimax {
        assert!(depth != 0, "depth for minimax cant be 0");
        Minimax {
            max_depth: depth,
            evaluator,
        }
    }
    pub fn find_maximising_move<F>(
        &self,
        board_state: &BoardState,
        update_progress: &mut F,
    ) -> Result<Action, &'static str>
    where
        F: FnMut(f32),
    {
        let beta = Score::MAX;
        let mut alpha = Score::MIN;
        let mut best_move = None;
        let legal_actions = find_legal_actions(&board_state, false).0;
        let mut action_number = 0;
        let action_count = legal_actions.len();
        for action in legal_actions {
            let mut new_board_state = board_state.clone();
            action.play_move(&mut new_board_state);
            let score = self.min(&new_board_state, self.max_depth - 1, alpha, beta);
            if best_move.is_none() || score > alpha {
                alpha = score;
                best_move = Some(action);
            }
            action_number += 1;
            update_progress(action_number as f32 / action_count as f32);
        }
        match best_move {
            Some(best_move) => Ok(best_move),
            None => Err("cant minimax an illegal board state"),
        }
    }

    fn min(&self, board_state: &BoardState, depth: u8, alpha: Score, beta: Score) -> Score {
        if depth == 0 {
            return self.evaluator.full_evaluate(&board_state);
        }
        let mut beta = beta;
        // only safe moves are considered on the final depth
        let (legal_actions, game_end_option) = if depth == 1 {
            find_legal_actions(&board_state, true)
        } else {
            find_legal_actions(&board_state, false)
        };
        if let Some(game_end) = game_end_option {
            return self.evaluate_game_end_state(game_end, depth);
        }

        let mut child_nodes: Vec<MinimaxNode> = Vec::with_capacity(legal_actions.len());
        for action in legal_actions {
            let mut new_board_state = board_state.clone();
            action.play_move(&mut new_board_state);
            child_nodes.push(MinimaxNode::new(new_board_state, &self.evaluator));
        }

        child_nodes.sort();

        // node importance controls how much the computer will think about the node
        // the lower the value, the more important the position and the greater the depth
        let mut node_importance = 0;
        for node in child_nodes {
            let depth_loss = self.depth_loss(node_importance, depth);
            let score = self.max(&node.board_state, depth - depth_loss, alpha, beta);
            if score <= alpha {
                return alpha;
            }
            if score < beta {
                beta = score;
            }
            node_importance += 1;
        }
        return beta;
    }

    fn max(&self, board_state: &BoardState, depth: u8, alpha: Score, beta: Score) -> Score {
        if depth == 0 {
            return self.evaluator.full_evaluate(&board_state);
        }
        let mut alpha = alpha;
        // only safe moves are considered on the final depth
        let (legal_actions, game_end_option) = if depth == 1 {
            find_legal_actions(&board_state, true)
        } else {
            find_legal_actions(&board_state, false)
        };
        if let Some(game_end) = game_end_option {
            return self.evaluate_game_end_state(game_end, depth);
        }
        let mut child_nodes: Vec<MinimaxNode> = Vec::with_capacity(legal_actions.len());
        for action in legal_actions {
            let mut new_board_state = board_state.clone();
            action.play_move(&mut new_board_state);
            child_nodes.push(MinimaxNode::new(new_board_state, &self.evaluator));
        }
        child_nodes.sort();
        child_nodes.reverse();

        let mut node_importance = 0;
        for node in child_nodes {
            let depth_loss = self.depth_loss(node_importance, depth);
            let score = self.min(&node.board_state, depth - depth_loss, alpha, beta);
            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
            node_importance += 1;
        }
        return alpha;
    }
    fn evaluate_game_end_state(&self, game_end_state: GameEndState, depth: u8) -> Score {
        match game_end_state {
            GameEndState::Draw => 0,
            GameEndState::Win(color) => {
                let moves_into_future = self.max_depth - depth;
                self.evaluator.score_for_checkmate(color, moves_into_future)
            }
        }
    }
    fn depth_loss(&self, i: u16, depth: u8) -> u8 {
        if i < 3 || depth < 2 {
            1
        } else if i < 5 || depth < 3 {
            2
        } else {
            3
        }
    }
}

#[derive(Eq)]
struct MinimaxNode {
    board_state: BoardState,
    estimated_score: Score,
}

impl MinimaxNode {
    fn new(board_state: BoardState, evaluator: &Evaluator) -> MinimaxNode {
        let estimated_score = evaluator.quick_evaluate(&board_state);
        MinimaxNode {
            board_state,
            estimated_score,
        }
    }
}

impl PartialEq for MinimaxNode {
    fn eq(&self, other: &Self) -> bool {
        self.estimated_score == other.estimated_score
    }
}
impl PartialOrd for MinimaxNode {
    fn partial_cmp(&self, other: &MinimaxNode) -> Option<std::cmp::Ordering> {
        Some(self.estimated_score.cmp(&other.estimated_score))
    }
}
impl Ord for MinimaxNode {
    fn cmp(&self, other: &MinimaxNode) -> std::cmp::Ordering {
        self.estimated_score.cmp(&other.estimated_score)
    }
}
