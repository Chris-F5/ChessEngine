mod board_state;
mod possible_move_iter;
mod resource_loader;

pub use board_state::{BoardPosition, BoardState, Piece, PieceColor, PieceType};

use ggez::{
    event::{self, EventHandler},
    graphics,
    nalgebra::{Point2, Vector2},
    Context, ContextBuilder, GameResult,
};
use resource_loader::PieceSetImages;
use std::{env, path};

const WINDOW_WIDTH: f32 = 620.0;
const WINDOW_HEIGHT: f32 = 620.0;
const PIECE_SCALE: f32 = 0.9;
const BOARD_SIZE: f32 = 600.0;
const BOARD_POS_SIZE: f32 = BOARD_SIZE / 8.0;
const BOARD_X_OFSET: f32 = 10.0;
const BOARD_Y_OFSET: f32 = 10.0;

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

    println!("{:?}", my_game.board_state);

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct ChessGame {
    board_state: BoardState,
    white_piece_images: PieceSetImages,
    black_piece_images: PieceSetImages,
    board_image: graphics::Image,
}

impl ChessGame {
    pub fn new(ctx: &mut Context) -> ChessGame {
        ChessGame {
            board_state: Default::default(),
            white_piece_images: resource_loader::load_white_piece_set(ctx),
            black_piece_images: resource_loader::load_black_piece_set(ctx),
            board_image: resource_loader::load_board_image(ctx),
        }
    }
    fn get_piece_image(&self, piece: Piece) -> &graphics::Image {
        match piece.color {
            PieceColor::White => self.white_piece_images.get_image(piece.piece_type),
            PieceColor::Black => self.black_piece_images.get_image(piece.piece_type),
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
        // draw board
        graphics::draw(
            ctx,
            &self.board_image,
            graphics::DrawParam::new()
                .dest(Point2::new(BOARD_X_OFSET, BOARD_Y_OFSET))
                .scale(Vector2::new(
                    BOARD_SIZE / self.board_image.width() as f32,
                    BOARD_SIZE / self.board_image.height() as f32,
                )),
        )
        .unwrap();

        for y in 0..8 {
            for x in 0..8 {
                let piece = self.board_state.get(BoardPosition::new(x, y));
                if let Some(piece) = piece {
                    let piece_image = self.get_piece_image(*piece);
                    graphics::draw(
                        ctx,
                        piece_image,
                        graphics::DrawParam::new()
                            .dest(Point2::new(
                                x as f32 * BOARD_POS_SIZE
                                    + BOARD_X_OFSET
                                    + BOARD_POS_SIZE * PIECE_SCALE * ((1.0 - PIECE_SCALE) / 2.0),
                                y as f32 * BOARD_POS_SIZE
                                    + BOARD_Y_OFSET
                                    + BOARD_POS_SIZE * PIECE_SCALE * ((1.0 - PIECE_SCALE) / 2.0),
                            ))
                            .scale(Vector2::new(
                                BOARD_POS_SIZE * PIECE_SCALE / piece_image.width() as f32,
                                BOARD_POS_SIZE * PIECE_SCALE / piece_image.height() as f32,
                            )),
                    )
                    .unwrap();
                }
            }
        }
        graphics::present(ctx)
    }
}
