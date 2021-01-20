mod endgame_table_search;
mod evaluator;
mod minimax;
mod opening_tables;

use crate::{Action, BoardState};
use endgame_table_search::EndgameTableSearcher;
use evaluator::{Evaluator, Score};
use minimax::Minimax;
use opening_tables::OpeningTables;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;

pub struct BestActionFinder {
    state_receiver: Receiver<State>,
    command_sender: Sender<Command>,
    state: State,
    best_action_finder_thread: thread::JoinHandle<()>,
}
impl BestActionFinder {
    pub fn new() -> BestActionFinder {
        let (command_sender, command_receiver) = channel();
        let (state_sender, state_receiver) = channel();
        let best_action_finder_thread =
            thread::spawn(move || Self::action_finding_loop(state_sender, command_receiver));
        BestActionFinder {
            state: State::Idle,
            command_sender,
            state_receiver,
            best_action_finder_thread,
        }
    }
    pub fn start_finding_move(&mut self, board_state: &BoardState) {
        self.state = State::Thinking(0.0);
        self.command_sender
            .send(Command::FindBestMove(board_state.clone()))
            .unwrap();
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
    fn action_finding_loop(state_sender: Sender<State>, command_receiver: Receiver<Command>) {
        let evaluator = Evaluator::new(EndgameTableSearcher::new());
        let opening_tables = OpeningTables::new();
        loop {
            let command = command_receiver
                .recv()
                .expect("There was an error with the action finding thread.");
            match command {
                Command::FindBestMove(board_state) => {
                    let mut update_progress = |progress: f32| {
                        state_sender.send(State::Thinking(progress)).unwrap();
                    };
                    let action = if let Some(action) = opening_tables.try_find_move(&board_state) {
                        Ok(action)
                    } else {
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
                        action_result
                    };
                    state_sender.send(State::Finished(action)).unwrap();
                }
                Command::Terminate => {
                    break;
                }
            }
        }
    }
}

impl Drop for BestActionFinder {
    fn drop(&mut self) {
        self.command_sender.send(Command::Terminate).unwrap();
    }
}

#[derive(Clone, Copy)]
pub enum State {
    Idle,
    Thinking(f32),
    Finished(Result<Action, &'static str>),
}
enum Command {
    FindBestMove(BoardState),
    Terminate,
}
