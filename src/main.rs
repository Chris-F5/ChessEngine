mod actions;
mod best_action_finder;
mod board_state;
mod gui;
mod resource_loader;

pub use actions::{find_legal_actions, in_check, Action, ActionType, GameEndState};
pub use board_state::{BoardPosition, BoardState, Capturable, Piece, PieceColor, PieceType};

use best_action_finder::BestActionFinder;
use ggez::{
    event::{self, EventHandler, MouseButton},
    graphics,
    input::mouse::set_cursor_type,
    Context, ContextBuilder, GameResult,
};
use gui::{GUIState, WINDOW_HEIGHT, WINDOW_WIDTH};
use resource_loader::{get_resource_path, PieceSetImages};

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("chess engine", "Christopher Lang")
        .window_setup(ggez::conf::WindowSetup::default().title("Chess Engine"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .add_resource_path(get_resource_path())
        .build()
        .expect("error creating ggez context");

    let mut game = ChessGame::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct ChessGame {
    board_state: BoardState,
    gui_state: GUIState,
    chess_computer: BestActionFinder,
    game_over: bool,
}

impl ChessGame {
    pub fn new(ctx: &mut Context) -> ChessGame {
        let new_game = ChessGame {
            board_state: BoardState::default(),
            gui_state: GUIState::new(
                resource_loader::load_white_piece_set(ctx),
                resource_loader::load_black_piece_set(ctx),
                resource_loader::load_board_image(ctx),
            ),
            chess_computer: BestActionFinder::new(),
            game_over: false,
        };
        new_game
    }

    fn play_move(&mut self, action: Action, ctx: &mut Context) {
        self.gui_state
            .update_last_played_move(Some(action), self.board_state.color_turn);
        action.play_move(&mut self.board_state);
        println!("{:?}", self.board_state);
        if let Some(game_end) = find_legal_actions(&self.board_state, false).1 {
            self.draw(ctx).unwrap();
            match game_end {
                GameEndState::Draw => {
                    gui::show_draw_message();
                }
                GameEndState::Win(PieceColor::White) => {
                    gui::show_player_wins_message();
                }
                GameEndState::Win(PieceColor::Black) => {
                    gui::show_computer_wins_message();
                }
            }
            self.game_over = true
        }
    }
}

impl EventHandler for ChessGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.game_over {
            match self.board_state.color_turn {
                PieceColor::White => {
                    if let Some(action) = self.gui_state.check_for_action() {
                        self.play_move(action, ctx);
                    }
                }
                PieceColor::Black => {
                    let chess_computer_state = self.chess_computer.get_state();
                    match chess_computer_state {
                        best_action_finder::State::Idle => {
                            set_cursor_type(ctx, ggez::input::mouse::MouseCursor::Wait);
                            self.chess_computer.start_finding_move(&self.board_state);
                        }
                        best_action_finder::State::Thinking(progress) => {
                            self.gui_state.update_progress_bar(progress)
                        }
                        best_action_finder::State::Finished(Err(err)) => {
                            panic!("error finding move: {}", err)
                        }
                        best_action_finder::State::Finished(Ok(action)) => {
                            set_cursor_type(ctx, ggez::input::mouse::MouseCursor::Default);
                            self.play_move(action, ctx)
                        }
                    }
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
        if button == MouseButton::Left
            && self.board_state.color_turn == PieceColor::White
            && !self.game_over
        {
            self.gui_state.click(x, y, &self.board_state);
        }
    }
}
