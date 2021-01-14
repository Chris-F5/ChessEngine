mod actions;
mod board_state;
mod evaluator;
mod gui;
mod minimax;
mod resource_loader;

pub use actions::{find_legal_actions, in_check, Action, ActionType, GameEndState};
pub use board_state::{BoardPosition, BoardState, Capturable, Piece, PieceColor, PieceType};
pub use evaluator::{Evaluator, Score};

use ggez::{
    event::{self, EventHandler, MouseButton},
    graphics, Context, ContextBuilder, GameResult,
};
use gui::GUIState;
use resource_loader::PieceSetImages;
use std::{env, path};

const WINDOW_WIDTH: f32 = 620.0;
const WINDOW_HEIGHT: f32 = 620.0;

fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    let (mut ctx, mut event_loop) = ContextBuilder::new("chess engine", "Christopher Lang")
        .window_setup(ggez::conf::WindowSetup::default().title("Chess Engine"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .add_resource_path(resource_dir)
        .build()
        .expect("error creating ggez context");

    let mut my_game = ChessGame::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct ChessGame {
    board_state: BoardState,
    gui_state: GUIState,
}

impl ChessGame {
    pub fn new(ctx: &mut Context) -> ChessGame {
        let new_game = ChessGame {
            board_state: BoardState::default(),
            //::from_fen(
            //"r2qk3/4b1p1/3p4/8/P2QP3/8/8/R3K3 b Qq - 0 1",
            //"r1bq1knr/ppp1bppp/3p4/8/2PQ4/1P6/P4PPP/RNB1KB1R b KQ - 0 1",
            //"k6B/8/4P3/3r4/2p5/8/8/7K b - - 0 1",
            //"4r1k1/p4ppp/2p5/2pp4/6b1/3P4/PPP5/RNB1r1RK w - - 0 22",
            //),
            gui_state: GUIState::new(
                resource_loader::load_white_piece_set(ctx),
                resource_loader::load_black_piece_set(ctx),
                resource_loader::load_board_image(ctx),
            ),
        };
        new_game
    }
    fn play_move(&mut self, action: Action, ctx: &mut Context) {
        action.play_move(&mut self.board_state);
        println!("{}", Evaluator::full_evaluate(&self.board_state));
        println!("{:?}", self.board_state);
        if let Some(game_end) = find_legal_actions(&self.board_state, false).1 {
            self.draw(ctx).unwrap();
            match game_end {
                GameEndState::Draw => {
                    gui::show_draw_message();
                    ggez::event::quit(ctx);
                }
                GameEndState::Win(PieceColor::White) => {
                    gui::show_player_wins_message();
                    ggez::event::quit(ctx);
                }
                GameEndState::Win(PieceColor::Black) => {
                    gui::show_computer_wins_message();
                    ggez::event::quit(ctx);
                }
            }
        }
    }
}

impl EventHandler for ChessGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        /*if let Some(player_action) = self.gui_state.check_for_action() {
            player_action.play_move(&mut self.board_state);
            self.draw(ctx).unwrap();

            if in_check_mate(&self.board_state) {
                gui::show_player_wins_message();
                ggez::event::quit(ctx);
            } else {
                ggez::input::mouse::set_cursor_type(ctx, ggez::input::mouse::MouseCursor::Wait);
                let ai_action = minimax::find_move_with_minimax(&self.board_state, 4);
                if let Some(action) = ai_action {
                    action.play_move(&mut self.board_state);
                    self.draw(ctx).unwrap();
                    ggez::input::mouse::set_cursor_type(
                        ctx,
                        ggez::input::mouse::MouseCursor::Default,
                    );
                    if find_legal_actions(&self.board_state).is_empty() {
                        gui::show_draw_message();
                        ggez::event::quit(ctx);
                    }
                } else {
                    if in_check_mate(&self.board_state) {
                        gui::show_computer_wins_message();
                        ggez::event::quit(ctx);
                    } else {
                        gui::show_draw_message();
                        ggez::event::quit(ctx);
                    }
                }
            }
        }*/
        match self.board_state.color_turn {
            PieceColor::White => {
                if let Some(action) = self.gui_state.check_for_action() {
                    self.play_move(action, ctx);
                }
            }
            PieceColor::Black => {
                ggez::input::mouse::set_cursor_type(ctx, ggez::input::mouse::MouseCursor::Wait);
                let action = minimax::find_move_with_minimax(&self.board_state, 4).unwrap();
                self.play_move(action, ctx);
                ggez::input::mouse::set_cursor_type(ctx, ggez::input::mouse::MouseCursor::Default);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.gui_state.draw(ctx, &self.board_state);
        graphics::present(ctx)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            self.gui_state.click(x, y, &self.board_state);
        }
    }
}
