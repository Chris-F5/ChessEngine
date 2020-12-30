mod board_state;
mod chess_AI;
mod gui;
mod resource_loader;
mod rules;

pub use board_state::{BoardPosition, BoardState, Piece, PieceColor, PieceType};
pub use rules::{color_in_check_mate, MoveType, PossibleMove, PossibleMoveIter};

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
        ChessGame {
            board_state: BoardState::default(),
            gui_state: GUIState::new(
                resource_loader::load_white_piece_set(ctx),
                resource_loader::load_black_piece_set(ctx),
                resource_loader::load_board_image(ctx),
            ),
        }
    }
}

impl EventHandler for ChessGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if let Some(player_move) = self.gui_state.check_for_move() {
            player_move.play_move(&mut self.board_state, PieceColor::White);
            self.draw(ctx).unwrap();

            if color_in_check_mate(&self.board_state, PieceColor::Black) {
                gui::show_player_wins_message();
                ggez::event::quit(ctx);
            } else {
                ggez::input::mouse::set_cursor_type(ctx, ggez::input::mouse::MouseCursor::Wait);
                let ai_move = chess_AI::find_best_move(&self.board_state);
                ai_move.play_move(&mut self.board_state, PieceColor::Black);
                self.draw(ctx).unwrap();
                ggez::input::mouse::set_cursor_type(ctx, ggez::input::mouse::MouseCursor::Default);

                if color_in_check_mate(&self.board_state, PieceColor::White) {
                    gui::show_computer_wins_message();
                    ggez::event::quit(ctx);
                }
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
