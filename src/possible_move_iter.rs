use crate::{BoardPosition, BoardState, Piece, PieceColor, PieceType};

#[derive(Copy, Clone)]
pub enum MoveType {
    SimpleMove {
        from: BoardPosition,
        to: BoardPosition,
        piece: Piece,
    },
    Castling {
        kings_side: bool,
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
            MoveType::Castling { kings_side } => (),
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
        let selected_piece = board_state.get(board_position);
        let y_dir = if color == PieceColor::White { 1 } else { -1 };
        if let Some(selected_piece) = selected_piece {
            if selected_piece.color == color {
                match selected_piece.piece_type {
                    PieceType::Pawn { en_passant } => {
                        let forward_one_position = ofset_board_pos(0, y_dir);
                        if board_state.get(forward_one_position).is_none() {
                            if forward_one_position.y == 0 || forward_one_position.y == 7 {
                                // queening
                                self.push(PossibleMove {
                                    move_type: MoveType::SimpleMove {
                                        from: board_position,
                                        to: forward_one_position,
                                        piece: Piece {
                                            piece_type: PieceType::Queen,
                                            color: color,
                                        },
                                    },
                                });
                            } else {
                                // move forward one
                                self.push(PossibleMove {
                                    move_type: MoveType::SimpleMove {
                                        from: board_position,
                                        to: forward_one_position,
                                        piece: Piece {
                                            piece_type: PieceType::Pawn { en_passant: false },
                                            color: color,
                                        },
                                    },
                                });
                                let forward_two_position = ofset_board_pos(0, y_dir * 2);
                                if ((board_position.y == 1 && color == PieceColor::White)
                                    || (board_position.y == 6 && color == PieceColor::Black))
                                    && board_state.get(forward_two_position).is_none()
                                {
                                    // move forward 2
                                    self.push(PossibleMove {
                                        move_type: MoveType::SimpleMove {
                                            from: board_position,
                                            to: forward_two_position,
                                            piece: Piece {
                                                piece_type: PieceType::Pawn { en_passant: true },
                                                color: color,
                                            },
                                        },
                                    });
                                }
                            }
                        }

                        if board_position.x != 7 {
                            let right_capture_position = ofset_board_pos(1, y_dir);
                            let target_piece = board_state.get(right_capture_position);
                            if let Some(target_piece) = target_piece {
                                if target_piece.color != color {
                                    if board_position.y == 6 {
                                        self.push(PossibleMove {
                                            move_type: MoveType::SimpleMove {
                                                from: board_position,
                                                to: right_capture_position,
                                                piece: Piece {
                                                    piece_type: PieceType::Queen,
                                                    color: color,
                                                },
                                            },
                                        });
                                    } else {
                                        self.push(PossibleMove {
                                            move_type: MoveType::SimpleMove {
                                                from: board_position,
                                                to: right_capture_position,
                                                piece: Piece {
                                                    piece_type: PieceType::Pawn {
                                                        en_passant: false,
                                                    },
                                                    color: color,
                                                },
                                            },
                                        });
                                    }
                                }
                            }
                        }
                        if board_position.x != 0 {
                            let left_capture_position = ofset_board_pos(-1, y_dir);
                            let target_piece = board_state.get(left_capture_position);
                            if let Some(target_piece) = target_piece {
                                if target_piece.color != color {
                                    if board_position.y == 6 {
                                        self.push(PossibleMove {
                                            move_type: MoveType::SimpleMove {
                                                from: board_position,
                                                to: left_capture_position,
                                                piece: Piece {
                                                    piece_type: PieceType::Queen,
                                                    color: color,
                                                },
                                            },
                                        });
                                    } else {
                                        self.push(PossibleMove {
                                            move_type: MoveType::SimpleMove {
                                                from: board_position,
                                                to: left_capture_position,
                                                piece: Piece {
                                                    piece_type: PieceType::Pawn {
                                                        en_passant: false,
                                                    },
                                                    color: color,
                                                },
                                            },
                                        });
                                    }
                                }
                            }
                        }
                    }
                    PieceType::Bishop => (),
                    PieceType::Knight => (),
                    PieceType::Rook { moved } => (),
                    PieceType::Queen => (),
                    PieceType::King { moved } => (),
                }
            }
        }
    }
}
