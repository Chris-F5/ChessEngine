use crate::{BoardPosition, BoardState, Piece, PieceColor, PieceType};

#[derive(Copy, Clone)]
pub enum MoveType {
    SimpleMove {
        from: BoardPosition,
        to: BoardPosition,
        piece: Piece,
    },
    Castling {
        color: PieceColor,
        kings_side: bool,
    },
    EnPassant {
        color: PieceColor,
        from: BoardPosition,
        to: BoardPosition,
    },
}

#[derive(Copy, Clone)]
pub struct PossibleMove {
    move_type: MoveType,
}

impl PossibleMove {
    pub fn play_move(&self, board_state: &mut BoardState) {
        match self.move_type {
            MoveType::SimpleMove { from, to, piece } => {
                *board_state.get_mut(to) = Some(piece);
                *board_state.get_mut(from) = None;
            }
            MoveType::Castling { color, kings_side } => {
                let y_row = if color == PieceColor::White { 0 } else { 7 };
                *board_state.get_mut(BoardPosition::new(4, y_row)) = None;
                if kings_side {
                    *board_state.get_mut(BoardPosition::new(7, y_row)) = None;
                    *board_state.get_mut(BoardPosition::new(4, y_row)) = None;
                    *board_state.get_mut(BoardPosition::new(5, y_row)) =
                        Some(Piece::new(color, PieceType::Rook { moved: true }));
                    *board_state.get_mut(BoardPosition::new(6, y_row)) =
                        Some(Piece::new(color, PieceType::King { moved: true }));
                } else {
                    *board_state.get_mut(BoardPosition::new(0, y_row)) = None;
                    *board_state.get_mut(BoardPosition::new(4, y_row)) = None;
                    *board_state.get_mut(BoardPosition::new(3, y_row)) =
                        Some(Piece::new(color, PieceType::Rook { moved: true }));
                    *board_state.get_mut(BoardPosition::new(2, y_row)) =
                        Some(Piece::new(color, PieceType::King { moved: true }));
                }
            }
            MoveType::EnPassant { color, from, to } => {
                *board_state.get_mut(to) =
                    Some(Piece::new(color, PieceType::Pawn { en_passant: false }));
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
    pub fn find_possible_moves(
        board_state: &mut BoardState,
        color: PieceColor,
    ) -> PossibleMoveIter {
        let mut new = PossibleMoveIter {
            index: 0,
            possible_moves: Vec::with_capacity(50),
        };
        new.populate(board_state, color);
        new
    }

    fn push(&mut self, new_move: PossibleMove) {
        self.possible_moves.push(new_move);
    }
    fn populate(&mut self, board_state: &mut BoardState, color: PieceColor) {
        // any of this color pawns that could have be taken with en passant can no longer
        let this_color_en_passant_row = if color == PieceColor::White { 3 } else { 4 };
        for x in 0..8 {
            let piece = board_state.get_mut(BoardPosition::new(x, this_color_en_passant_row));
            if let Some(piece) = piece {
                if let PieceType::Pawn { en_passant: true } = piece.piece_type {
                    piece.piece_type = PieceType::Pawn { en_passant: false };
                }
            }
        }

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
        let ofset_board_pos = |x: i8, y: i8| -> BoardPosition {
            BoardPosition::new(
                (board_position.x as i8 + x) as u8,
                (board_position.y as i8 + y) as u8,
            )
        };
        let mut try_move =
            |self_access: &mut Self, x: i8, y: i8, result_piece: PieceType| -> bool {
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
                            },
                        });
                        return true;
                    }
                }
                false
            };
        let mut try_capture =
            |self_access: &mut Self, x: i8, y: i8, result_piece: PieceType| -> bool {
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
                                },
                            });
                            return true;
                        }
                    }
                }
                false
            };
        let selected_piece = board_state.get(board_position);
        let y_dir = if color == PieceColor::White { 1 } else { -1 };
        if let Some(selected_piece) = selected_piece {
            if selected_piece.color == color {
                match selected_piece.piece_type {
                    PieceType::Pawn { en_passant: _ } => {
                        if (board_position.y == 1 && color == PieceColor::Black)
                            || (board_position.y == 6 && color == PieceColor::White)
                        {
                            try_move(self, 0, 1, PieceType::Queen);
                            try_capture(self, 1, 1, PieceType::Queen);
                            try_capture(self, -1, 1, PieceType::Queen);
                        } else {
                            try_capture(self, 1, 1, PieceType::Pawn { en_passant: false });
                            try_capture(self, -1, 1, PieceType::Pawn { en_passant: false });
                            if try_move(self, 0, 1, PieceType::Pawn { en_passant: false })
                                && (board_position.y == 1 || board_position.y == 6)
                            {
                                try_move(self, 0, 2, PieceType::Pawn { en_passant: true });
                            } else if (board_position.y == 3 && color == PieceColor::Black)
                                || (board_position.y == 4 && color == PieceColor::White)
                            {
                                let left_piece_pos = ofset_board_pos(-1, 0);
                                if let Some(left_piece) = board_state.get(left_piece_pos) {
                                    if let PieceType::Pawn { en_passant: true } =
                                        left_piece.piece_type
                                    {
                                        self.push(PossibleMove {
                                            move_type: MoveType::EnPassant {
                                                from: board_position,
                                                to: ofset_board_pos(-1, 1),
                                                color: color,
                                            },
                                        });
                                    }
                                }
                                let right_piece_pos = ofset_board_pos(1, 0);
                                if let Some(right_piece) = board_state.get(right_piece_pos) {
                                    if let PieceType::Pawn { en_passant: true } =
                                        right_piece.piece_type
                                    {
                                        self.push(PossibleMove {
                                            move_type: MoveType::EnPassant {
                                                from: board_position,
                                                to: ofset_board_pos(1, 1),
                                                color: color,
                                            },
                                        });
                                    }
                                }
                            }
                        }
                    }
                    PieceType::Knight => {
                        try_move(self, 1, 2, PieceType::Knight);
                        try_capture(self, 1, 2, PieceType::Knight);
                        try_move(self, 2, 1, PieceType::Knight);
                        try_capture(self, 2, 1, PieceType::Knight);

                        try_move(self, 1, -2, PieceType::Knight);
                        try_capture(self, 1, -2, PieceType::Knight);
                        try_move(self, 2, -1, PieceType::Knight);
                        try_capture(self, 2, -1, PieceType::Knight);

                        try_move(self, -1, 2, PieceType::Knight);
                        try_capture(self, -1, 2, PieceType::Knight);
                        try_move(self, -2, 1, PieceType::Knight);
                        try_capture(self, -2, 1, PieceType::Knight);

                        try_move(self, -1, -2, PieceType::Knight);
                        try_capture(self, -1, -2, PieceType::Knight);
                        try_move(self, -2, -1, PieceType::Knight);
                        try_capture(self, -2, -1, PieceType::Knight);
                    }
                    PieceType::Bishop => {
                        let mut target_x = 1;
                        let mut target_y = 1;
                        while try_move(self, target_x, target_y, PieceType::Bishop) {
                            target_x += 1;
                            target_y += 1;
                        }
                        try_capture(self, target_x, target_y, PieceType::Bishop);

                        let mut target_x = -1;
                        let mut target_y = 1;
                        while try_move(self, target_x, target_y, PieceType::Bishop) {
                            target_x -= 1;
                            target_y += 1;
                        }
                        try_capture(self, target_x, target_y, PieceType::Bishop);

                        let mut target_x = 1;
                        let mut target_y = -1;
                        while try_move(self, target_x, target_y, PieceType::Bishop) {
                            target_x += 1;
                            target_y -= 1;
                        }
                        try_capture(self, target_x, target_y, PieceType::Bishop);

                        let mut target_x = -1;
                        let mut target_y = -1;
                        while try_move(self, target_x, target_y, PieceType::Bishop) {
                            target_x -= 1;
                            target_y -= 1;
                        }
                        try_capture(self, target_x, target_y, PieceType::Bishop);
                    }
                    PieceType::Rook { moved } => {
                        let mut target_x = 1;
                        while try_move(self, target_x, 0, PieceType::Rook { moved: true }) {
                            target_x += 1;
                        }
                        try_capture(self, target_x, 0, PieceType::Rook { moved: true });
                        let mut target_x = -1;
                        while try_move(self, target_x, 0, PieceType::Rook { moved: true }) {
                            target_x -= 1;
                        }
                        try_capture(self, target_x, 0, PieceType::Rook { moved: true });
                        let mut target_y = 1;
                        while try_move(self, 0, target_y, PieceType::Rook { moved: true }) {
                            target_y += 1;
                        }
                        try_capture(self, 0, target_y, PieceType::Rook { moved: true });
                        let mut target_y = -1;
                        while try_move(self, 0, target_y, PieceType::Rook { moved: true }) {
                            target_y -= 1;
                        }
                        try_capture(self, 0, target_y, PieceType::Rook { moved: true });
                    }
                    PieceType::Queen => {
                        let mut target_x = 1;
                        while try_move(self, target_x, 0, PieceType::Queen) {
                            target_x += 1;
                        }
                        try_capture(self, target_x, 0, PieceType::Queen);
                        let mut target_x = -1;
                        while try_move(self, target_x, 0, PieceType::Queen) {
                            target_x -= 1;
                        }
                        try_capture(self, target_x, 0, PieceType::Queen);
                        let mut target_y = 1;
                        while try_move(self, 0, target_y, PieceType::Queen) {
                            target_y += 1;
                        }
                        try_capture(self, 0, target_y, PieceType::Queen);
                        let mut target_y = -1;
                        while try_move(self, 0, target_y, PieceType::Queen) {
                            target_y -= 1;
                        }
                        try_capture(self, 0, target_y, PieceType::Queen);

                        let mut target_x = 1;
                        let mut target_y = 1;
                        while try_move(self, target_x, target_y, PieceType::Queen) {
                            target_x += 1;
                            target_y += 1;
                        }
                        try_capture(self, target_x, target_y, PieceType::Queen);

                        let mut target_x = -1;
                        let mut target_y = 1;
                        while try_move(self, target_x, target_y, PieceType::Queen) {
                            target_x -= 1;
                            target_y += 1;
                        }
                        try_capture(self, target_x, target_y, PieceType::Queen);

                        let mut target_x = 1;
                        let mut target_y = -1;
                        while try_move(self, target_x, target_y, PieceType::Queen) {
                            target_x += 1;
                            target_y -= 1;
                        }
                        try_capture(self, target_x, target_y, PieceType::Queen);

                        let mut target_x = -1;
                        let mut target_y = -1;
                        while try_move(self, target_x, target_y, PieceType::Queen) {
                            target_x -= 1;
                            target_y -= 1;
                        }
                        try_capture(self, target_x, target_y, PieceType::Queen);
                    }
                    PieceType::King { moved } => {
                        try_move(self, 0, 1, PieceType::King { moved: true });
                        try_move(self, 1, 1, PieceType::King { moved: true });
                        try_move(self, 1, 0, PieceType::King { moved: true });
                        try_move(self, 1, -1, PieceType::King { moved: true });
                        try_move(self, 0, -1, PieceType::King { moved: true });
                        try_move(self, -1, -1, PieceType::King { moved: true });
                        try_move(self, -1, 0, PieceType::King { moved: true });
                        try_move(self, -1, 1, PieceType::King { moved: true });

                        try_capture(self, 0, 1, PieceType::King { moved: true });
                        try_capture(self, 1, 1, PieceType::King { moved: true });
                        try_capture(self, 1, 0, PieceType::King { moved: true });
                        try_capture(self, 1, -1, PieceType::King { moved: true });
                        try_capture(self, 0, -1, PieceType::King { moved: true });
                        try_capture(self, -1, -1, PieceType::King { moved: true });
                        try_capture(self, -1, 0, PieceType::King { moved: true });
                        try_capture(self, -1, 1, PieceType::King { moved: true });

                        // castling
                        if moved == false {
                            if board_state
                                .get(BoardPosition::new(5, board_position.y))
                                .is_none()
                                && board_state
                                    .get(BoardPosition::new(6, board_position.y))
                                    .is_none()
                            {
                                if let Some(rook) =
                                    board_state.get(BoardPosition::new(7, board_position.y))
                                {
                                    if let PieceType::Rook { moved: false } = rook.piece_type {
                                        if !positions_in_check(
                                            &board_state,
                                            vec![
                                                BoardPosition::new(4, board_position.y),
                                                BoardPosition::new(5, board_position.y),
                                            ],
                                            color,
                                        ) {
                                            self.push(PossibleMove {
                                                move_type: MoveType::Castling {
                                                    color,
                                                    kings_side: true,
                                                },
                                            });
                                        }
                                    }
                                }
                            }
                            if board_state
                                .get(BoardPosition::new(1, board_position.y))
                                .is_none()
                                && board_state
                                    .get(BoardPosition::new(2, board_position.y))
                                    .is_none()
                                && board_state
                                    .get(BoardPosition::new(3, board_position.y))
                                    .is_none()
                            {
                                if let Some(rook) =
                                    board_state.get(BoardPosition::new(0, board_position.y))
                                {
                                    if let PieceType::Rook { moved: false } = rook.piece_type {
                                        if !positions_in_check(
                                            &board_state,
                                            vec![
                                                BoardPosition::new(4, board_position.y),
                                                BoardPosition::new(3, board_position.y),
                                            ],
                                            color,
                                        ) {
                                            self.push(PossibleMove {
                                                move_type: MoveType::Castling {
                                                    color,
                                                    kings_side: false,
                                                },
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn positions_in_check(
    board_state: &BoardState,
    positions: Vec<BoardPosition>,
    color: PieceColor,
) -> bool {
    let in_or_pass_check = false;
    let possible_opponent_moves =
        PossibleMoveIter::find_possible_moves(&mut board_state.clone(), color.opposite_color());
    for possible_move in possible_opponent_moves {
        match possible_move.move_type {
            MoveType::SimpleMove {
                from: _,
                to,
                piece: _,
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
