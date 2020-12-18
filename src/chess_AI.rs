mod evaluate;
mod minimax;

pub use evaluate::full_evaluate;
pub use evaluate::Score;

use super::{BoardState, PossibleMove};

pub fn find_best_move(board_state: &BoardState) -> PossibleMove {
    minimax::minimax(board_state)
}
