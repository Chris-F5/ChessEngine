use crate::{
    find_legal_actions, Action, ActionType, BoardPosition, BoardState, Piece, PieceColor,
    PieceSetImages,
};
use dialog::DialogBox;
use ggez::{
    graphics,
    nalgebra::{Point2, Vector2},
    Context,
};

const PIECE_SCALE: f32 = 0.9;
const BOARD_SIZE: f32 = 600.0;
const BOARD_POS_SIZE: f32 = BOARD_SIZE / 8.0;
const BOARD_MARGINS: f32 = 10.0;
const HIGHLIGHT_COLOR: graphics::Color = graphics::Color::new(1.0, 1.0, 1.0, 0.3);
const PROGRESS_BAR_HEIGHT: f32 = 20.0;
const PROGRESS_BAR_Y_MARGINS: f32 = 0.0;
const PROGRESS_BAR_X_MARGINS: f32 = 10.0;

pub const WINDOW_WIDTH: f32 = BOARD_SIZE + 2.0 * BOARD_MARGINS;
pub const WINDOW_HEIGHT: f32 =
    BOARD_SIZE + BOARD_MARGINS + 2.0 * PROGRESS_BAR_Y_MARGINS + PROGRESS_BAR_HEIGHT;

enum Sellection {
    None,
    Selected(BoardPosition),
}

fn screen_space_to_board_pos(x: f32, y: f32) -> Option<BoardPosition> {
    let x = x - BOARD_MARGINS;
    let y = y - BOARD_MARGINS;
    if x > 0.0 && y > 0.0 {
        let board_pos = BoardPosition::new(
            (x / BOARD_POS_SIZE as f32) as u8,
            (y / BOARD_POS_SIZE as f32) as u8,
        );
        if board_pos.bound_check() {
            let inverted_board_pos = BoardPosition::new(board_pos.x, 7 - board_pos.y);
            Some(inverted_board_pos)
        } else {
            None
        }
    } else {
        None
    }
}

fn board_pos_to_screen_space(board_pos: BoardPosition) -> (f32, f32) {
    (
        board_pos.x as f32 * BOARD_POS_SIZE + BOARD_MARGINS,
        (7 - board_pos.y) as f32 * BOARD_POS_SIZE + BOARD_MARGINS,
    )
}

pub struct GUIState {
    sellection: Sellection,
    white_piece_images: PieceSetImages,
    black_piece_images: PieceSetImages,
    board_image: graphics::Image,
    possible_moves_from_selection: Vec<PlayerAction>,
    pending_move: Option<Action>,
    progress_bar_percentage: f32,
}
impl GUIState {
    pub fn new(
        white_piece_images: PieceSetImages,
        black_piece_images: PieceSetImages,
        board_image: graphics::Image,
    ) -> GUIState {
        GUIState {
            sellection: Sellection::None,
            white_piece_images,
            black_piece_images,
            board_image,
            possible_moves_from_selection: Vec::with_capacity(20),
            pending_move: None,
            progress_bar_percentage: 1.0,
        }
    }

    pub fn check_for_action(&mut self) -> Option<Action> {
        let pending_move = self.pending_move;
        self.pending_move = None;
        pending_move
    }

    fn get_piece_image(&self, piece: Piece) -> &graphics::Image {
        match piece.color {
            PieceColor::White => self.white_piece_images.get_image(piece.piece_type),
            PieceColor::Black => self.black_piece_images.get_image(piece.piece_type),
        }
    }
    pub fn draw(&self, ctx: &mut Context, board_state: &BoardState) {
        self.draw_board(ctx);
        self.draw_highlighted_squares(ctx);
        self.draw_pieces(ctx, board_state);
        self.draw_progress_bar(ctx);
    }
    fn draw_board(&self, ctx: &mut Context) {
        graphics::draw(
            ctx,
            &self.board_image,
            graphics::DrawParam::new()
                .dest(Point2::new(BOARD_MARGINS, BOARD_MARGINS))
                .scale(Vector2::new(
                    BOARD_SIZE / self.board_image.width() as f32,
                    BOARD_SIZE / self.board_image.height() as f32,
                )),
        )
        .unwrap();
    }
    fn draw_pieces(&self, ctx: &mut Context, board_state: &BoardState) {
        for y in 0..8 {
            for x in 0..8 {
                let piece = board_state.get(BoardPosition::new(x, y));
                if let Some(piece) = piece {
                    let piece_image = self.get_piece_image(*piece);
                    let (screen_space_x, screen_space_y) =
                        board_pos_to_screen_space(BoardPosition::new(x, y));
                    graphics::draw(
                        ctx,
                        piece_image,
                        graphics::DrawParam::new()
                            .dest(Point2::new(
                                screen_space_x
                                    + BOARD_POS_SIZE * PIECE_SCALE * ((1.0 - PIECE_SCALE) / 2.0),
                                screen_space_y
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
    }
    fn draw_highlighted_squares(&self, ctx: &mut Context) {
        let mut drawn_a_square = false;
        let mut highlight_mesh = graphics::MeshBuilder::new();
        let mut highlight_square = |pos: BoardPosition| {
            let (screen_space_x, screen_space_y) = board_pos_to_screen_space(pos);
            highlight_mesh.rectangle(
                graphics::DrawMode::fill(),
                graphics::Rect::new(
                    screen_space_x,
                    screen_space_y,
                    BOARD_POS_SIZE - 1.0,
                    BOARD_POS_SIZE - 1.0,
                ),
                HIGHLIGHT_COLOR,
            );
            drawn_a_square = true;
        };
        if let Sellection::Selected(pos) = self.sellection {
            highlight_square(pos);
        }
        for possible_move in self.possible_moves_from_selection.iter() {
            highlight_square(possible_move.to);
        }

        if drawn_a_square {
            let mesh = highlight_mesh.build(ctx).unwrap();
            graphics::draw(ctx, &mesh, graphics::DrawParam::new()).unwrap();
        }
    }
    pub fn click(&mut self, x: f32, y: f32, board_state: &BoardState) {
        let board_pos = screen_space_to_board_pos(x, y);
        if let Some(board_pos) = board_pos {
            if let Sellection::Selected(selection) = self.sellection {
                if selection == board_pos {
                    self.deselect();
                } else if self.try_move_to(board_pos) {
                    self.deselect();
                }
            } else {
                if let Some(piece) = board_state.get(board_pos) {
                    if piece.color == PieceColor::White {
                        self.select(board_pos, board_state);
                    }
                }
            }
        } else {
            self.deselect();
        }
    }
    fn select(&mut self, pos: BoardPosition, board_state: &BoardState) {
        self.sellection = Sellection::Selected(pos);

        let all_possible_actions = find_legal_actions(board_state, false).0;

        for action in all_possible_actions {
            let player_action = PlayerAction::from(action);
            if player_action.from == pos {
                self.possible_moves_from_selection.push(player_action);
            }
        }
    }
    fn try_move_to(&mut self, pos: BoardPosition) -> bool {
        for possible_move in self.possible_moves_from_selection.iter() {
            if possible_move.to == pos {
                self.pending_move = Some(possible_move.this_action);
                return true;
            }
        }
        false
    }
    fn deselect(&mut self) {
        self.sellection = Sellection::None;
        self.possible_moves_from_selection.clear();
    }
    pub fn update_progress_bar(&mut self, percentage: f32) {
        assert!(percentage >= 0.0 && percentage <= 1.0);
        self.progress_bar_percentage = percentage;
    }
    fn draw_progress_bar(&self, ctx: &mut Context) {
        assert!(self.progress_bar_percentage >= 0.0 && self.progress_bar_percentage <= 1.0);
        let rect_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                PROGRESS_BAR_X_MARGINS,
                BOARD_MARGINS + BOARD_SIZE + PROGRESS_BAR_Y_MARGINS,
                (WINDOW_WIDTH - (2.0 * PROGRESS_BAR_X_MARGINS)) * self.progress_bar_percentage,
                PROGRESS_BAR_HEIGHT,
            ),
            if self.progress_bar_percentage == 1.0 {
                graphics::Color::new(50.0 / 255.0, 168.0 / 255.0, 82.0 / 255.0, 1.0)
            } else {
                graphics::Color::new(217.0 / 255.0, 123.0 / 255.0, 35.0 / 255.0, 1.0)
            },
        )
        .unwrap();
        graphics::draw(ctx, &rect_mesh, graphics::DrawParam::new()).unwrap();
    }
}

struct PlayerAction {
    this_action: Action,
    // the "to" and "from" are the squares that you have to click on to make the piece move
    from: BoardPosition,
    to: BoardPosition,
}

impl PlayerAction {
    pub fn from(action: Action) -> PlayerAction {
        PlayerAction {
            this_action: action,
            from: PlayerAction::find_from(action),
            to: PlayerAction::find_to(action),
        }
    }
    fn find_from(action: Action) -> BoardPosition {
        match action.get_action_type() {
            ActionType::SimpleMove { from, to: _ } => from,
            ActionType::Castling { kings_side: _ } => BoardPosition::new(4, 0),
            ActionType::EnPassant { from, to: _ } => from,
        }
    }
    fn find_to(action: Action) -> BoardPosition {
        match action.get_action_type() {
            ActionType::SimpleMove { from: _, to } => to,
            ActionType::Castling { kings_side } => {
                if kings_side {
                    BoardPosition::new(6, 0)
                } else {
                    BoardPosition::new(2, 0)
                }
            }
            ActionType::EnPassant { from: _, to } => to,
        }
    }
}

pub fn show_player_wins_message() {
    dialog::Message::new("Congratulations - You Win!")
        .title("You Win")
        .show()
        .expect("Could not display dialog box");
}
pub fn show_computer_wins_message() {
    dialog::Message::new("Computer Won. :(")
        .title("Computer Won")
        .show()
        .expect("Could not display dialog box");
}
pub fn show_draw_message() {
    dialog::Message::new("You drew with the computer.")
        .title("Draw")
        .show()
        .expect("Could not display dialog box");
}
