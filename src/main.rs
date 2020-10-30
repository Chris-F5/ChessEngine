mod board_state;
use board_state::BoardState;

use ggez::{
    event::{self, EventHandler},
    graphics, Context, ContextBuilder, GameResult,
};

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_setup(ggez::conf::WindowSetup::default().title("Chess Engine"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 500.0))
        .build()
        .expect("error creating ggez context");

    let mut my_game = ChessGame::new(&mut ctx);

    println!("{:?}", my_game.board_state);

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct ChessGame {
    board_state: BoardState,
}

impl ChessGame {
    pub fn new(_ctx: &mut Context) -> ChessGame {
        ChessGame {
            board_state: Default::default(),
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
        // TODO: draw chess board
        graphics::present(ctx)
    }
}
