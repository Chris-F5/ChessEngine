mod endgame_table_search;
mod evaluator;
mod minimax;

use crate::{Action, BoardState};
use endgame_table_search::EndgameTableSearcher;
use evaluator::{Evaluator, Score};
use minimax::Minimax;

pub struct BestActionFinder {
    evaluator: Evaluator,
}
impl BestActionFinder {
    pub fn new() -> BestActionFinder {
        BestActionFinder {
            evaluator: Evaluator::new(EndgameTableSearcher::new()),
        }
    }
    pub fn find_best_move<F>(
        &self,
        board_state: &BoardState,
        progress_update: &mut F,
    ) -> Option<Action>
    where
        F: FnMut(f32),
    {
        progress_update(0.0);

        if self.evaluator.is_in_endgame(board_state) {
            println!("endgame search");
            // Depth of one is required because the endgame tables do all the hard work in the endgame
            let minimax = Minimax::new(1, &self.evaluator);
            minimax.find_maximising_move(board_state)
        } else {
            println!("non-endgame search");
            let minimax = Minimax::new(7, &self.evaluator);
            minimax.find_maximising_move(board_state)
        }
    }
}
