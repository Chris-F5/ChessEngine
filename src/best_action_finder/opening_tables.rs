use crate::{Action, ActionType, BoardPosition, BoardState};
use std::collections::HashMap;

pub struct OpeningTables {
    entrys: HashMap<BoardState, Action>,
}

impl OpeningTables {
    pub fn new() -> OpeningTables {
        let mut entrys = HashMap::new();
        // Kings pawn opening e5
        entrys.insert(
            BoardState::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"),
            Action::new(ActionType::SimpleMove {
                from: BoardPosition::from_text("e7"),
                to: BoardPosition::from_text("e5"),
            }),
        );
        // Kings pawn opening kings knight nc6
        entrys.insert(
            BoardState::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2"),
            Action::new(ActionType::SimpleMove {
                from: BoardPosition::from_text("b8"),
                to: BoardPosition::from_text("c6"),
            }),
        );
        // Giuoco Piano Game
        entrys.insert(
            BoardState::from_fen(
                "r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 3 3",
            ),
            Action::new(ActionType::SimpleMove {
                from: BoardPosition::from_text("f8"),
                to: BoardPosition::from_text("c5"),
            }),
        );
        // Ruy López Morphy Defense
        entrys.insert(
            BoardState::from_fen(
                "r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 3 3",
            ),
            Action::new(ActionType::SimpleMove {
                from: BoardPosition::from_text("a7"),
                to: BoardPosition::from_text("a6"),
            }),
        );
        // Indian Game
        entrys.insert(
            BoardState::from_fen("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1"),
            Action::new(ActionType::SimpleMove {
                from: BoardPosition::from_text("g8"),
                to: BoardPosition::from_text("f6"),
            }),
        );
        // Indian Game East Indian Defense
        entrys.insert(
            BoardState::from_fen("rnbqkb1r/pppppppp/5n2/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq c3 0 2"),
            Action::new(ActionType::SimpleMove {
                from: BoardPosition::from_text("e7"),
                to: BoardPosition::from_text("e6"),
            }),
        );
        OpeningTables { entrys }
    }
    pub fn try_find_move(&self, board_state: &BoardState) -> Option<Action> {
        if self.entrys.contains_key(board_state) {
            Some(self.entrys[board_state])
        } else {
            None
        }
    }
}
