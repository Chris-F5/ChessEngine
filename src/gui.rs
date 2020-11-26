use crate::{
    BoardPosition, BoardState, MoveType, Piece, PieceColor, PieceSetImages, PossibleMove,
    PossibleMoveIter,
};
use ggez::{
    graphics,
    nalgebra::{Point2, Vector2},
    Context,
};

const PIECE_SCALE: f32 = 0.9;
const BOARD_SIZE: f32 = 600.0;
const BOARD_POS_SIZE: f32 = BOARD_SIZE / 8.0;
const BOARD_X_OFSET: f32 = 10.0;
const BOARD_Y_OFSET: f32 = 10.0;
const HIGHLIGHT_COLOR: graphics::Color = graphics::Color::new(1.0, 1.0, 1.0, 0.3);

enum Sellection {
    None,
    Selected(BoardPosition),
}

fn screen_space_to_board_pos(x: f32, y: f32) -> Option<BoardPosition> {
    let x = x - BOARD_X_OFSET;
    let y = y - BOARD_Y_OFSET;
    if x > 0.0 && y > 0.0 {
        let board_pos = BoardPosition::new(
            (x / BOARD_POS_SIZE as f32) as u8,
            (y / BOARD_POS_SIZE as f32) as u8,
        );
        if board_pos.is_valid() {
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
        board_pos.x as f32 * BOARD_POS_SIZE + BOARD_X_OFSET,
        (7 - board_pos.y) as f32 * BOARD_POS_SIZE + BOARD_Y_OFSET,
    )
}

pub struct GUIState {
    board_state: BoardState,
    sellection: Sellection,
    white_piece_images: PieceSetImages,
    black_piece_images: PieceSetImages,
    board_image: graphics::Image,
    possible_moves_from_selection: Vec<PossiblePlayerMove>,
}
impl GUIState {
    pub fn new(
        white_piece_images: PieceSetImages,
        black_piece_images: PieceSetImages,
        board_image: graphics::Image,
    ) -> GUIState {
        GUIState {
            board_state: Default::default(),
            sellection: Sellection::None,
            white_piece_images,
            black_piece_images,
            board_image,
            possible_moves_from_selection: Vec::with_capacity(20),
        }
    }

    fn get_piece_image(&self, piece: Piece) -> &graphics::Image {
        match piece.color {
            PieceColor::White => self.white_piece_images.get_image(piece.piece_type),
            PieceColor::Black => self.black_piece_images.get_image(piece.piece_type),
        }
    }
    pub fn draw(&self, ctx: &mut Context) {
        self.draw_board(ctx);
        self.draw_highlighted_squares(ctx);
        self.draw_pieces(ctx);
    }
    fn draw_board(&self, ctx: &mut Context) {
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
    }
    fn draw_pieces(&self, ctx: &mut Context) {
        for y in 0..8 {
            for x in 0..8 {
                let piece = self.board_state.get(BoardPosition::new(x, y));
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
    pub fn click(&mut self, x: f32, y: f32) {
        let board_pos = screen_space_to_board_pos(x, y);
        if let Some(board_pos) = board_pos {
            if let Sellection::Selected(selection) = self.sellection {
                if selection == board_pos {
                    self.deselect();
                } else if self.try_move_to(board_pos) {
                    self.deselect();
                    println!("{:?}", self.board_state);
                }
            } else {
                if let Some(piece) = self.board_state.get(board_pos) {
                    if piece.color == PieceColor::White {
                        self.select(board_pos);
                    }
                }
            }
        } else {
            self.deselect();
        }
    }
    fn select(&mut self, pos: BoardPosition) {
        self.sellection = Sellection::Selected(pos);

        let all_possible_moves =
            PossibleMoveIter::find_possible_moves(&mut self.board_state, PieceColor::White);
        for possible_move in all_possible_moves {
            let player_possible_move = PossiblePlayerMove::from(possible_move);
            if player_possible_move.from == pos {
                self.possible_moves_from_selection
                    .push(player_possible_move);
            }
        }
    }
    fn try_move_to(&mut self, pos: BoardPosition) -> bool {
        for possible_move in self.possible_moves_from_selection.iter() {
            if possible_move.to == pos {
                possible_move.this_move.play_move(&mut self.board_state);
                return true;
            }
        }
        false
    }
    fn deselect(&mut self) {
        self.sellection = Sellection::None;
        self.possible_moves_from_selection.clear();
    }
}

struct PossiblePlayerMove {
    this_move: PossibleMove,
    // the "to" and "from" are the squares that you have to click on to make the piece move
    from: BoardPosition,
    to: BoardPosition,
}

impl PossiblePlayerMove {
    pub fn from(the_move: PossibleMove) -> PossiblePlayerMove {
        PossiblePlayerMove {
            this_move: the_move,
            from: PossiblePlayerMove::find_from(the_move.get_move_type()),
            to: PossiblePlayerMove::find_to(the_move.get_move_type()),
        }
    }
    fn find_from(move_type: MoveType) -> BoardPosition {
        match move_type {
            MoveType::SimpleMove {
                from,
                to: _,
                piece: _,
            } => from,
            MoveType::Castling {
                color: _,
                kings_side: _,
            } => BoardPosition::new(4, 0),
            MoveType::EnPassant {
                from,
                to: _,
                color: _,
            } => from,
        }
    }
    fn find_to(move_type: MoveType) -> BoardPosition {
        match move_type {
            MoveType::SimpleMove {
                from: _,
                to,
                piece: _,
            } => to,
            MoveType::Castling {
                color: _,
                kings_side,
            } => {
                if kings_side {
                    BoardPosition::new(6, 0)
                } else {
                    BoardPosition::new(2, 0)
                }
            }
            MoveType::EnPassant {
                from: _,
                to,
                color: _,
            } => to,
        }
    }
}
