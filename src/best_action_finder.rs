mod endgame_table_search;
mod evaluator;
mod minimax;

pub use endgame_table_search::EndgameTableSearcher;
pub use evaluator::{Evaluator, Score};

use crate::{Action, BoardState};

pub struct BestActionFinder {
    tables: EndgameTableSearcher,
}
impl BestActionFinder {
    pub fn new() -> BestActionFinder {
        BestActionFinder {
            tables: EndgameTableSearcher::new(),
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

        if self.tables.win_loss_check(board_state).is_some() {
            // endgame
            println!("endgame search");
            minimax::find_move_with_minimax(
                board_state,
                1,
                &Evaluator::new(EndgameTableSearcher::new()),
                progress_update,
            )
        } else {
            // not endgame
            println!("non-endgame search");
            minimax::find_move_with_minimax(
                board_state,
                7,
                &Evaluator::new(EndgameTableSearcher::new()),
                progress_update,
            )
        }
    }
}
