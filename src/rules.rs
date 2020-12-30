use crate::{BoardPosition, BoardState, Piece, PieceColor, PieceType};

#[derive(Copy, Clone)]
pub enum MoveType {
    SimpleMove {
        from: BoardPosition,
        to: BoardPosition,
        piece: Piece,
        cancel_king_castle: bool,
        cancel_queen_castle: bool,
        pawn_move_two: bool,
    },
    Castling {
        kings_side: bool,
    },
    EnPassant {
        from: BoardPosition,
        to: BoardPosition,
    },
}

#[derive(Copy, Clone)]
pub struct PossibleMove {
    move_type: MoveType,
}

impl PossibleMove {
    pub fn play_move(&self, board_state: &mut BoardState, color: PieceColor) {
        // reset the en_passant to some colunm that will never be reached by the possible move finder
        board_state.en_passant_colunm = 55;

        match self.move_type {
            MoveType::SimpleMove {
                from,
                to,
                piece,
                cancel_king_castle,
                cancel_queen_castle,
                pawn_move_two,
            } => {
                *board_state.get_mut(to) = Some(piece);
                *board_state.get_mut(from) = None;
                if pawn_move_two {
                    board_state.en_passant_colunm = to.x;
                } else {
                    if cancel_king_castle {
                        if color == PieceColor::White {
                            board_state.white_king_castle = false;
                        } else {
                            board_state.black_king_castle = false;
                        }
                    }
                    if cancel_queen_castle {
                        if color == PieceColor::White {
                            board_state.white_queen_castle = false;
                        } else {
                            board_state.black_queen_castle = false;
                        }
                    }
                }
            }
            MoveType::Castling { kings_side } => {
                let y_row = if color == PieceColor::White { 0 } else { 7 };
                *board_state.get_mut(BoardPosition::new(4, y_row)) = None;
                if kings_side {
                    *board_state.get_mut(BoardPosition::new(7, y_row)) = None;
                    *board_state.get_mut(BoardPosition::new(5, y_row)) =
                        Some(Piece::new(color, PieceType::Rook));
                    *board_state.get_mut(BoardPosition::new(6, y_row)) =
                        Some(Piece::new(color, PieceType::King));
                } else {
                    *board_state.get_mut(BoardPosition::new(0, y_row)) = None;
                    *board_state.get_mut(BoardPosition::new(3, y_row)) =
                        Some(Piece::new(color, PieceType::Rook));
                    *board_state.get_mut(BoardPosition::new(2, y_row)) =
                        Some(Piece::new(color, PieceType::King));
                }
            }
            MoveType::EnPassant { from, to } => {
                *board_state.get_mut(to) = Some(Piece::new(color, PieceType::Pawn));
                *board_state.get_mut(from) = None;
                *board_state.get_mut(BoardPosition::new(to.x, from.y)) = None;
            }
        }
    }
    pub fn get_move_type(&self) -> MoveType {
        self.move_type
    }
}

pub struct PossibleMoveIter {
    index: usize,
    possible_moves: Vec<PossibleMove>,
    king_pos: BoardPosition,
    can_take_king: bool,
}

impl Iterator for PossibleMoveIter {
    type Item = PossibleMove;
    fn next(&mut self) -> Option<PossibleMove> {
        if self.index >= self.possible_moves.len() {
            None
        } else {
            self.index += 1;
            Some(self.possible_moves[self.index - 1])
        }
    }
}

impl PossibleMoveIter {
    pub fn find_possible_moves(board_state: &BoardState, color: PieceColor) -> PossibleMoveIter {
        let mut king_pos = None;
        for y in 0..8 {
            for x in 0..8 {
                if let Some(piece) = board_state.get(BoardPosition::new(x, y)) {
                    if piece.piece_type == PieceType::King && piece.color != color {
                        king_pos = Some(BoardPosition::new(x, y));
                    }
                }
            }
        }
        if let Some(king_pos) = king_pos {
            let mut new = PossibleMoveIter {
                index: 0,
                possible_moves: Vec::with_capacity(50),
                king_pos,
                can_take_king: false,
            };
            new.populate(board_state, color);
            new
        } else {
            panic!("Cant find possible moves when there is no king of the other color");
        }
    }

    pub fn can_take_king(&self) -> bool {
        self.can_take_king
    }

    fn push(&mut self, new_move: PossibleMove) {
        if let MoveType::SimpleMove {
            from: _,
            to,
            piece: _,
            cancel_king_castle: _,
            cancel_queen_castle: _,
            pawn_move_two: _,
        } = new_move.move_type
        {
            if to == self.king_pos {
                self.can_take_king = true;
            }
        }
        self.possible_moves.push(new_move);
    }
    fn populate(&mut self, board_state: &BoardState, color: PieceColor) {
        for y in 0..8 {
            for x in 0..8 {
                self.push_piece_moves(&board_state, BoardPosition::new(x, y), color);
            }
        }
    }
    fn push_piece_moves(
        &mut self,
        board_state: &BoardState,
        board_position: BoardPosition,
        color: PieceColor,
    ) {
        let y_dir = if color == PieceColor::White { 1 } else { -1 };
        let ofset_board_pos = |x: i8, y: i8| -> BoardPosition {
            BoardPosition::new(
                (board_position.x as i8 + x) as u8,
                (board_position.y as i8 + (y * y_dir)) as u8,
            )
        };
        let try_move = |self_access: &mut Self,
                        x: i8,
                        y: i8,
                        result_piece: PieceType,
                        cancel_king_castle: bool,
                        cancel_queen_castle: bool,
                        pawn_move_two: bool|
         -> bool {
            let target_board_pos = ofset_board_pos(x, y);
            if target_board_pos.is_valid() {
                let target_piece = board_state.get(target_board_pos);
                if target_piece.is_none() {
                    self_access.push(PossibleMove {
                        move_type: MoveType::SimpleMove {
                            from: board_position,
                            to: target_board_pos,
                            piece: Piece {
                                piece_type: result_piece,
                                color: color,
                            },
                            cancel_king_castle,
                            cancel_queen_castle,
                            pawn_move_two,
                        },
                    });
                    return true;
                }
            }
            false
        };
        let try_capture = |self_access: &mut Self,
                           x: i8,
                           y: i8,
                           result_piece: PieceType,
                           cancel_king_castle: bool,
                           cancel_queen_castle: bool|
         -> bool {
            let target_board_pos = ofset_board_pos(x, y);
            if target_board_pos.is_valid() {
                let target_piece = board_state.get(target_board_pos);
                if let Some(piece) = target_piece {
                    if piece.color != color {
                        self_access.push(PossibleMove {
                            move_type: MoveType::SimpleMove {
                                from: board_position,
                                to: target_board_pos,
                                piece: Piece {
                                    piece_type: result_piece,
                                    color: color,
                                },
                                cancel_king_castle,
                                cancel_queen_castle,
                                pawn_move_two: false,
                            },
                        });
                        return true;
                    }
                }
            }
            false
        };
        let selected_piece = board_state.get(board_position);
        if let Some(selected_piece) = selected_piece {
            if selected_piece.color == color {
                match selected_piece.piece_type {
                    PieceType::Pawn => {
                        if (board_position.y == 1 && color == PieceColor::Black)
                            || (board_position.y == 6 && color == PieceColor::White)
                        {
                            try_move(self, 0, 1, PieceType::Queen, false, false, false);
                            try_capture(self, 1, 1, PieceType::Queen, false, false);
                            try_capture(self, -1, 1, PieceType::Queen, false, false);
                        } else {
                            try_capture(self, 1, 1, PieceType::Pawn, false, false);
                            try_capture(self, -1, 1, PieceType::Pawn, false, false);
                            if try_move(self, 0, 1, PieceType::Pawn, false, false, false)
                                && (board_position.y == 1 || board_position.y == 6)
                            {
                                try_move(self, 0, 2, PieceType::Pawn, false, false, true);
                            } else if (board_position.y == 3 && color == PieceColor::Black)
                                || (board_position.y == 4 && color == PieceColor::White)
                            {
                                if board_state.en_passant_colunm == board_position.x + 1 {
                                    self.push(PossibleMove {
                                        move_type: MoveType::EnPassant {
                                            from: board_position,
                                            to: ofset_board_pos(1, 1),
                                        },
                                    });
                                } else if board_position.x != 0
                                    && board_state.en_passant_colunm == board_position.x - 1
                                {
                                    self.push(PossibleMove {
                                        move_type: MoveType::EnPassant {
                                            from: board_position,
                                            to: ofset_board_pos(-1, 1),
                                        },
                                    });
                                }
                            }
                        }
                    }
                    PieceType::Knight => {
                        let mut move_or_capture = |x_move: i8, y_move: i8| {
                            if !try_move(
                                self,
                                x_move,
                                y_move,
                                PieceType::Knight,
                                false,
                                false,
                                false,
                            ) {
                                try_capture(self, x_move, y_move, PieceType::Knight, false, false);
                            }
                        };
                        move_or_capture(1, 2);
                        move_or_capture(2, 1);

                        move_or_capture(1, -2);
                        move_or_capture(2, -1);

                        move_or_capture(-1, 2);
                        move_or_capture(-2, 1);

                        move_or_capture(-1, -2);
                        move_or_capture(-2, -1);
                    }
                    PieceType::Bishop => {
                        let mut move_in_dir = |x_dir: i8, y_dir: i8| {
                            let mut target_x = x_dir;
                            let mut target_y = y_dir;
                            while try_move(
                                self,
                                target_x,
                                target_y,
                                PieceType::Bishop,
                                false,
                                false,
                                false,
                            ) {
                                target_x += x_dir;
                                target_y += y_dir;
                            }
                            try_capture(self, target_x, target_y, PieceType::Bishop, false, false);
                        };

                        move_in_dir(1, 1);
                        move_in_dir(1, -1);
                        move_in_dir(-1, 1);
                        move_in_dir(-1, -1);
                    }
                    PieceType::Rook => {
                        let mut move_in_dir = |x_dir: i8, y_dir: i8| {
                            let mut target_x = x_dir;
                            let mut target_y = y_dir;
                            while try_move(
                                self,
                                target_x,
                                target_y,
                                PieceType::Rook,
                                false,
                                false,
                                false,
                            ) {
                                target_x += x_dir;
                                target_y += y_dir;
                            }
                            try_capture(self, target_x, target_y, PieceType::Rook, false, false);
                        };
                        move_in_dir(1, 0);
                        move_in_dir(-1, 0);
                        move_in_dir(0, 1);
                        move_in_dir(0, -1);
                    }
                    PieceType::Queen => {
                        let mut move_in_dir = |x_dir: i8, y_dir: i8| {
                            let mut target_x = x_dir;
                            let mut target_y = y_dir;
                            while try_move(
                                self,
                                target_x,
                                target_y,
                                PieceType::Queen,
                                false,
                                false,
                                false,
                            ) {
                                target_x += x_dir;
                                target_y += y_dir;
                            }
                            try_capture(self, target_x, target_y, PieceType::Queen, false, false);
                        };
                        move_in_dir(1, 0);
                        move_in_dir(-1, 0);
                        move_in_dir(0, 1);
                        move_in_dir(0, -1);

                        move_in_dir(1, 1);
                        move_in_dir(1, -1);
                        move_in_dir(-1, 1);
                        move_in_dir(-1, -1);
                    }
                    PieceType::King => {
                        let mut move_or_capture = |x_move: i8, y_move: i8| {
                            if !try_move(self, x_move, y_move, PieceType::King, false, false, false)
                            {
                                try_capture(self, x_move, y_move, PieceType::King, false, false);
                            }
                        };

                        move_or_capture(0, 1);
                        move_or_capture(1, 1);
                        move_or_capture(1, 0);
                        move_or_capture(1, -1);
                        move_or_capture(0, -1);
                        move_or_capture(-1, -1);
                        move_or_capture(-1, 0);
                        move_or_capture(-1, 1);

                        // castling
                        if ((board_state.white_king_castle && color == PieceColor::White)
                            || (board_state.black_king_castle && color == PieceColor::Black))
                            && board_state
                                .get(BoardPosition::new(5, board_position.y))
                                .is_none()
                            && board_state
                                .get(BoardPosition::new(6, board_position.y))
                                .is_none()
                            && !positions_in_check(
                                &board_state,
                                vec![
                                    BoardPosition::new(4, board_position.y),
                                    BoardPosition::new(5, board_position.y),
                                ],
                                color,
                            )
                        {
                            self.push(PossibleMove {
                                move_type: MoveType::Castling { kings_side: true },
                            });
                        }

                        if ((board_state.white_queen_castle && color == PieceColor::White)
                            || (board_state.black_queen_castle && color == PieceColor::Black))
                            && board_state
                                .get(BoardPosition::new(3, board_position.y))
                                .is_none()
                            && board_state
                                .get(BoardPosition::new(2, board_position.y))
                                .is_none()
                            && board_state
                                .get(BoardPosition::new(1, board_position.y))
                                .is_none()
                            && !positions_in_check(
                                &board_state,
                                vec![
                                    BoardPosition::new(3, board_position.y),
                                    BoardPosition::new(2, board_position.y),
                                    BoardPosition::new(1, board_position.y),
                                ],
                                color,
                            )
                        {
                            self.push(PossibleMove {
                                move_type: MoveType::Castling { kings_side: false },
                            });
                        }
                    }
                }
            }
        }
    }
}

// Used for castling only
fn positions_in_check(
    board_state: &BoardState,
    positions: Vec<BoardPosition>,
    color: PieceColor,
) -> bool {
    let mut board_state = board_state.clone();
    for position in positions.iter() {
        // put a king here so pawns can take it
        *board_state.get_mut(*position) = Some(Piece::new(color, PieceType::King));
    }
    let possible_opponent_moves =
        PossibleMoveIter::find_possible_moves(&mut board_state, color.opposite_color());
    for possible_move in possible_opponent_moves {
        match possible_move.move_type {
            MoveType::SimpleMove {
                from: _,
                to,
                piece: _,
                cancel_king_castle: _,
                cancel_queen_castle: _,
                pawn_move_two: _,
            } => {
                for position in positions.iter() {
                    if *position == to {
                        return true;
                    }
                }
            }
            _ => (),
        }
    }
    false
}

pub fn color_in_check(board_state: &BoardState, color: PieceColor) -> bool {
    // TODO: optomize?

    let mut king_pos = None;
    for y in 0..8 {
        for x in 0..8 {
            if let Some(piece) = board_state.get(BoardPosition::new(x, y)) {
                if piece.piece_type == PieceType::King && piece.color == color {
                    king_pos = Some(BoardPosition::new(x, y));
                }
            }
        }
    }
    if let Some(king_pos) = king_pos {
        let possible_opponent_moves =
            PossibleMoveIter::find_possible_moves(board_state, color.opposite_color());
        for possible_move in possible_opponent_moves {
            match possible_move.move_type {
                MoveType::SimpleMove {
                    from: _,
                    to,
                    piece: _,
                    cancel_king_castle: _,
                    cancel_queen_castle: _,
                    pawn_move_two: _,
                } => {
                    if king_pos == to {
                        return true;
                    }
                }
                _ => (),
            }
        }
        return false;
    } else {
        panic!("this color has no king");
    }
}

pub fn color_in_check_mate(board_state: &BoardState, color: PieceColor) -> bool {
    let my_moves = PossibleMoveIter::find_possible_moves(board_state, color);
    for possible_move in my_moves {
        let mut new_board_state = board_state.clone();
        possible_move.play_move(&mut new_board_state, color);
        if !color_in_check(&new_board_state, color) {
            println!("{:?} can move because -> \n{:?}", color, new_board_state);
            return false;
        }
    }
    return true;
}
