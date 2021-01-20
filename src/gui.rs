use crate::{
    find_legal_actions, Action, ActionType, BoardPosition, BoardState, Piece, PieceColor,
    PieceSetImages,
};
use dialog::DialogBox;
use ggez::{
    graphics,
    graphics::Rect,
    nalgebra::{Point2, Vector2},
    Context,
};

const PIECE_SCALE: f32 = 0.9;
const PIECE_SIZE: f32 = PIECE_SCALE * BOARD_POS_SIZE;
const BOARD_SIZE: f32 = 600.0;
const BOARD_POS_SIZE: f32 = BOARD_SIZE / 8.0;
const BOARD_MARGINS: f32 = 10.0;
const HIGHLIGHT_COLOR: graphics::Color = graphics::Color::new(1.0, 1.0, 1.0, 0.3);
const PROGRESS_BAR_HEIGHT: f32 = 20.0;
const PROGRESS_BAR_Y_MARGINS: f32 = 0.0;
const PROGRESS_BAR_X_MARGINS: f32 = 10.0;

const ARROW_WIDTH: f32 = 7.0;
const ARROW_COLOR: graphics::Color =
    graphics::Color::new(240.0 / 255.0, 14.0 / 255.0, 52.0 / 255.0, 1.0);

pub const WINDOW_WIDTH: f32 = BOARD_SIZE + 2.0 * BOARD_MARGINS;
pub const WINDOW_HEIGHT: f32 =
    BOARD_SIZE + BOARD_MARGINS + 2.0 * PROGRESS_BAR_Y_MARGINS + PROGRESS_BAR_HEIGHT;

enum Sellection {
    None,
    Selected(BoardPosition),
}

fn screen_pos_to_board_pos(x: f32, y: f32) -> Option<BoardPosition> {
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

fn board_pos_to_screen_pos(board_pos: BoardPosition) -> Point2<f32> {
    Point2::new(
        board_pos.x as f32 * BOARD_POS_SIZE + BOARD_MARGINS + (BOARD_POS_SIZE / 2.0),
        (7 - board_pos.y) as f32 * BOARD_POS_SIZE + BOARD_MARGINS + (BOARD_POS_SIZE / 2.0),
    )
}

fn board_pos_to_screen_rect(board_pos: BoardPosition) -> Rect {
    let centered_pos = board_pos_to_screen_pos(board_pos);
    Rect::new(
        centered_pos.x - (BOARD_POS_SIZE / 2.0),
        centered_pos.y - (BOARD_POS_SIZE / 2.0),
        BOARD_POS_SIZE,
        BOARD_POS_SIZE,
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
    last_played_move: Option<PlayerAction>,
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
            last_played_move: None,
        }
    }

    pub fn update_last_played_move(&mut self, action: Option<Action>, color: PieceColor) {
        self.last_played_move = match action {
            Some(action) => Some(PlayerAction::new(action, color)),
            None => None,
        }
    }

    pub fn check_for_action(&mut self) -> Option<Action> {
        match self.pending_move {
            Some(pending_move) => {
                self.pending_move = None;
                self.deselect();
                Some(pending_move)
            }
            None => None,
        }
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
        self.draw_last_played_action(ctx);
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
    fn draw_piece(&self, ctx: &mut Context, board_pos: BoardPosition, piece: &Piece) {
        let piece_image = self.get_piece_image(*piece);
        let centered_pos = board_pos_to_screen_pos(board_pos);
        graphics::draw(
            ctx,
            piece_image,
            graphics::DrawParam::new()
                .dest(Point2::new(
                    centered_pos.x - (PIECE_SIZE / 2.0),
                    centered_pos.y - (PIECE_SIZE / 2.0),
                ))
                .scale(Vector2::new(
                    PIECE_SIZE / piece_image.width() as f32,
                    PIECE_SIZE / piece_image.height() as f32,
                )),
        )
        .unwrap();
    }
    fn draw_pieces(&self, ctx: &mut Context, board_state: &BoardState) {
        for y in 0..8 {
            for x in 0..8 {
                let board_pos = BoardPosition::new(x, y);
                let piece = board_state.get(board_pos);
                if let Some(piece) = piece {
                    self.draw_piece(ctx, board_pos, &piece);
                }
            }
        }
    }
    fn draw_highlighted_squares(&self, ctx: &mut Context) {
        let mut drawn_a_square = false;
        let mut highlight_mesh = graphics::MeshBuilder::new();
        let mut highlight_square = |pos: BoardPosition| {
            let rect = board_pos_to_screen_rect(pos);
            highlight_mesh.rectangle(graphics::DrawMode::fill(), rect, HIGHLIGHT_COLOR);
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
    fn draw_last_played_action(&self, ctx: &mut Context) {
        if let Some(action) = &self.last_played_move {
            let screen_space_from = board_pos_to_screen_pos(action.from);
            let screen_space_to = board_pos_to_screen_pos(action.to);
            self.draw_arrow(ctx, screen_space_from, screen_space_to);
        }
    }
    fn draw_arrow(&self, ctx: &mut Context, from: Point2<f32>, to: Point2<f32>) {
        let arrow_vector = Vector2::new(to.x - from.x, to.y - from.y);
        let right_head_vector = Vector2::new(
            arrow_vector.y - arrow_vector.x,
            -arrow_vector.x - arrow_vector.y,
        )
        .normalize()
            * 30.0;
        let left_head_vector = Vector2::new(
            -arrow_vector.y - arrow_vector.x,
            arrow_vector.x - arrow_vector.y,
        )
        .normalize()
            * 30.0;
        let left_head = Point2::new(to.x + left_head_vector.x, to.y + left_head_vector.y);
        let right_head = Point2::new(to.x + right_head_vector.x, to.y + right_head_vector.y);
        let line_mesh =
            graphics::Mesh::new_line(ctx, &[from, to], ARROW_WIDTH, ARROW_COLOR).unwrap();
        let arrow_head_mesh =
            graphics::Mesh::new_line(ctx, &[left_head, to, right_head], ARROW_WIDTH, ARROW_COLOR)
                .unwrap();
        graphics::draw(ctx, &line_mesh, graphics::DrawParam::new()).unwrap();
        graphics::draw(ctx, &arrow_head_mesh, graphics::DrawParam::new()).unwrap();
    }
    pub fn click(&mut self, x: f32, y: f32, board_state: &BoardState) {
        let board_pos = screen_pos_to_board_pos(x, y);
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
            let player_action = PlayerAction::new(action, board_state.color_turn);
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
    pub fn new(action: Action, color: PieceColor) -> PlayerAction {
        PlayerAction {
            this_action: action,
            from: PlayerAction::find_from(action, color),
            to: PlayerAction::find_to(action, color),
        }
    }
    fn find_from(action: Action, color: PieceColor) -> BoardPosition {
        match action.get_action_type() {
            ActionType::SimpleMove { from, to: _ } => from,
            ActionType::Castling { kings_side: _ } => match color {
                PieceColor::White => BoardPosition::new(4, 0),
                PieceColor::Black => BoardPosition::new(4, 7),
            },
            ActionType::EnPassant { from, to: _ } => from,
        }
    }
    fn find_to(action: Action, color: PieceColor) -> BoardPosition {
        match action.get_action_type() {
            ActionType::SimpleMove { from: _, to } => to,
            ActionType::Castling { kings_side } => {
                let rank = match color {
                    PieceColor::White => 0,
                    PieceColor::Black => 7,
                };
                if kings_side {
                    BoardPosition::new(6, rank)
                } else {
                    BoardPosition::new(2, rank)
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
