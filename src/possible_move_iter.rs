use crate::{BoardPosition, BoardState, Piece, PieceColor, PieceType};

#[derive(Copy, Clone)]
pub enum MoveType {
    // A move where one piece moves to another position on the board
    // without changing its type.
    SimpleMove {
        from: BoardPosition,
        to: BoardPosition,
    },
    // A move where one piece moves to another position on the board
    // and changes to another piece. e.g. pawn moves forward 2 and
    // becomes pawn that can be captured with en passant.
    ChangeMove {
        from: BoardPosition,
        to: BoardPosition,
        new_piece: Piece,
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
        // TODO: impl function
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
    pub fn find_possible_moves(board_state: &BoardState, color: PieceColor) -> PossibleMoveIter {
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
    fn populate(&mut self, board_state: &BoardState, color: PieceColor) {
        for y in 0..8 {
            for x in 0..8 {
                self.push_piece_moves(board_state, BoardPosition::new(x, y), color);
            }
        }
    }
    fn push_piece_moves(
        &mut self,
        board_state: &BoardState,
        board_position: BoardPosition,
        only_for_color: PieceColor,
    ) {
        let ofset_board_pos = |x: i8, y: i8| -> BoardPosition {
            BoardPosition::new(
                (board_position.x as i8 + x) as u8,
                (board_position.y as i8 + y) as u8,
            )
        };
        let selected_piece = board_state.get(board_position);
        let y_dir = if only_for_color == PieceColor::White {
            -1
        } else {
            1
        };
        match selected_piece {
            Some(selected_piece) => {
                if selected_piece.color == only_for_color {
                    match selected_piece.piece_type {
                        PieceType::Pawn { en_passant } => {
                            let forward_one_position = ofset_board_pos(0, y_dir);
                            if board_state.get(forward_one_position).is_none() {
                                self.push(PossibleMove {
                                    move_type: MoveType::SimpleMove {
                                        from: board_position,
                                        to: forward_one_position,
                                    },
                                });
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
            None => (),
        }
    }
}
