use super::Score;
use crate::{BoardState, GameEndState, PieceColor};
use shakmaty::fen::Fen;
use shakmaty::{CastlingMode, Chess};
use shakmaty_syzygy::{Dtz, Syzygy, Tablebase, Wdl};
use std::{env, path};

pub struct EndgameTableSearcher {
    tables: Tablebase<Chess>,
}

impl EndgameTableSearcher {
    pub fn new() -> EndgameTableSearcher {
        let mut tables = Tablebase::new();
        let mut resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            let mut path = path::PathBuf::from(manifest_dir);
            path.push("resources");
            path
        } else {
            path::PathBuf::from("./resources")
        };
        resource_dir.push("tables/3-4-5");
        tables.add_directory(resource_dir).unwrap();
        EndgameTableSearcher { tables }
    }
    fn get_shakmaty_state(&self, board_state: &BoardState) -> Option<Chess> {
        let fen_str = board_state.to_fen();
        let parsed_fen = fen_str.parse::<Fen>();
        match parsed_fen {
            Err(err) => {
                println!("error passing fen");
                None
            }
            Ok(parsed) => Some(parsed.position(CastlingMode::Standard).unwrap()),
        }
    }
    pub fn win_loss_check(&self, board_state: &BoardState) -> Option<GameEndState> {
        match self.get_shakmaty_state(board_state) {
            None => None,
            Some(shakmaty_state) => match self.tables.probe_wdl(&shakmaty_state) {
                Ok(Wdl::Draw) => Some(GameEndState::Draw),
                Ok(Wdl::Win) => Some(GameEndState::Win(board_state.color_turn)),
                Ok(Wdl::Loss) => Some(GameEndState::Win(board_state.color_turn.opposite_color())),
                Ok(Wdl::CursedWin) => Some(GameEndState::Win(board_state.color_turn)),
                Ok(Wdl::BlessedLoss) => {
                    Some(GameEndState::Win(board_state.color_turn.opposite_color()))
                }
                Err(err) => None,
            },
        }
    }
    pub fn evaluate_state(&self, board_state: &BoardState) -> Option<Score> {
        match self.get_shakmaty_state(board_state) {
            None => None,
            Some(shakmaty_state) => match self.tables.probe_dtz(&shakmaty_state) {
                Ok(dtz) => {
                    let mut dtz = dtz.0 as Score;
                    if board_state.color_turn == PieceColor::White {
                        if dtz > 0 {
                            Some(Score::MIN + dtz)
                        } else if dtz < 0 {
                            Some(Score::MAX + dtz)
                        } else {
                            Some(0)
                        }
                    } else {
                        if dtz > 0 {
                            Some(Score::MAX - dtz)
                        } else if dtz < 0 {
                            Some(Score::MIN - dtz)
                        } else {
                            Some(0)
                        }
                    }
                }
                Err(err) => None,
            },
        }
    }
}
