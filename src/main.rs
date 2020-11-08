mod board_drawer;
mod board_state;
mod possible_move_iter;
mod resource_loader;

pub use board_state::{BoardPosition, BoardState, Piece, PieceColor, PieceType};

use board_drawer::GUIState;
use ggez::{
    event::{self, EventHandler, MouseButton},
    graphics,
    nalgebra::{Point2, Vector2},
    Context, ContextBuilder, GameResult,
};
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
    gui_state: board_drawer::GUIState,
}

impl ChessGame {
    pub fn new(ctx: &mut Context) -> ChessGame {
        ChessGame {
            gui_state: GUIState::new(
                resource_loader::load_white_piece_set(ctx),
                resource_loader::load_black_piece_set(ctx),
                resource_loader::load_board_image(ctx),
            ),
        }
    }
}

impl EventHandler for ChessGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // TODO: recieve user input and update chess board accordingly
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.gui_state.draw(ctx);
        graphics::present(ctx)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            self.gui_state.click(x, y)
        }
    }
}
