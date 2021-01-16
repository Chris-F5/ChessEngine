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
use gui::{GUIState, WINDOW_HEIGHT, WINDOW_WIDTH};
use resource_loader::PieceSetImages;
use std::{env, path};

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
            board_state: BoardState::from_fen("3kr3/4r3/8/8/8/8/7K/8 w - - 0 1"), //BoardState::default(),
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
        match self.board_state.color_turn {
            PieceColor::White => {
                if let Some(action) = self.gui_state.check_for_action() {
                    self.play_move(action, ctx);
                }
            }
            PieceColor::Black => {
                ggez::input::mouse::set_cursor_type(ctx, ggez::input::mouse::MouseCursor::Wait);
                let board_state = self.board_state.clone();
                let mut progress_update = |percentage: f32| {
                    self.gui_state.update_progress_bar(percentage);
                    self.draw(ctx).unwrap();
                };
                let action =
                    minimax::find_move_with_minimax(&board_state, 7, &mut progress_update).unwrap();
                self.play_move(action, ctx);
                ggez::input::mouse::set_cursor_type(ctx, ggez::input::mouse::MouseCursor::Default);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.gui_state.draw(ctx, &self.board_state);
        //GUIState::update_progress_bar(ctx, 0.7);
        graphics::present(ctx)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            self.gui_state.click(x, y, &self.board_state);
        }
    }
}
