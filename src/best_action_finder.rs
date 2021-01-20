mod endgame_table_search;
mod evaluator;
mod minimax;

use crate::{Action, BoardState};
use endgame_table_search::EndgameTableSearcher;
use evaluator::{Evaluator, Score};
use minimax::Minimax;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;

pub struct BestActionFinder {
    state_receiver: Receiver<State>,
    board_sender: Sender<BoardState>,
    state: State,
}
impl BestActionFinder {
    pub fn new() -> BestActionFinder {
        let (board_sender, board_receiver) = channel();
        let (state_sender, state_receiver) = channel();
        thread::spawn(move || Self::action_finding_loop(state_sender, board_receiver));
        BestActionFinder {
            state: State::Idle,
            board_sender,
            state_receiver,
        }
    }
    pub fn start_finding_move(&mut self, board_state: &BoardState) {
        self.state = State::Thinking(0.0);
        self.board_sender.send(board_state.clone()).unwrap();
    }
    pub fn get_state(&mut self) -> State {
        match &self.state_receiver.try_recv() {
            Ok(State::Thinking(progress)) => {
                self.state = State::Thinking(*progress);
                self.state
            }
            Ok(State::Finished(result)) => {
                self.state = State::Idle;
                return State::Finished(*result);
            }
            Ok(State::Idle) => panic!("best move finder is idle"),
            Err(TryRecvError::Empty) => self.state,
            Err(TryRecvError::Disconnected) => panic!("move finder thread hung up"),
        }
    }
    fn action_finding_loop(state_sender: Sender<State>, board_receiver: Receiver<BoardState>) {
        let evaluator = Evaluator::new(EndgameTableSearcher::new());
        loop {
            let board_state = board_receiver.recv().expect(
                "There was an error with the action finding thread. Try restarting the program.",
            );
            let mut update_progress = |progress: f32| {
                state_sender.send(State::Thinking(progress)).unwrap();
            };
            let action_result = if evaluator.is_in_endgame(&board_state) {
                println!("endgame search");
                // Depth of one is required because the endgame tables do all the hard work in the endgame
                let minimax = Minimax::new(1, &evaluator);
                minimax.find_maximising_move(&board_state, &mut update_progress)
            } else {
                println!("non-endgame search");
                let minimax = Minimax::new(7, &evaluator);
                minimax.find_maximising_move(&board_state, &mut update_progress)
            };
            state_sender.send(State::Finished(action_result)).unwrap();
        }
    }
}

#[derive(Clone, Copy)]
pub enum State {
    Idle,
    Thinking(f32),
    Finished(Result<Action, &'static str>),
}
