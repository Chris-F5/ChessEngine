mod action_rules;

#[cfg(test)]
mod action_tests;

use crate::{BoardPosition, BoardState, Piece, PieceColor, PieceType};
use action_rules::ActionRule;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ActionType {
    SimpleMove {
        from: BoardPosition,
        to: BoardPosition,
    },
    Castling {
        kings_side: bool,
    },
    EnPassant {
        from: BoardPosition,
        to: BoardPosition,
    },
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Action {
    action_type: ActionType,
}

impl Action {
    pub fn new(action_type: ActionType) -> Action {
        Action { action_type }
    }

    pub fn play_move(&self, board_state: &mut BoardState) {
        // reset the en_passant to some colunm that will never be reached by the possible move finder
        board_state.en_passant_colunm = 55;
        let color = board_state.color_turn;
        board_state.color_turn = color.opposite_color();

        match self.action_type {
            ActionType::SimpleMove { from, to } => {
                let from_piece_ref = board_state.get_mut(from);
                let from_piece = from_piece_ref.clone();
                *from_piece_ref = None;
                let mut result_piece = from_piece;
                match from_piece.unwrap().piece_type {
                    PieceType::Pawn => {
                        if (from.y == 1 && to.y == 3) || (from.y == 6 && to.y == 4) {
                            board_state.en_passant_colunm = to.x;
                        } else if to.y == 0 || to.y == 7 {
                            result_piece = Some(Piece::new(color, PieceType::Queen));
                        }
                    }
                    PieceType::King => {
                        if color == PieceColor::White {
                            board_state.white_king_castle = false;
                            board_state.white_queen_castle = false;
                        } else {
                            board_state.black_king_castle = false;
                            board_state.black_queen_castle = false;
                        }
                    }
                    PieceType::Rook => {
                        if color == PieceColor::White {
                            if from.y == 0 {
                                if from.x == 0 {
                                    board_state.white_queen_castle = false;
                                } else if from.x == 7 {
                                    board_state.white_king_castle = false;
                                }
                            }
                        } else {
                            if from.y == 0 {
                                if from.x == 0 {
                                    board_state.black_queen_castle = false;
                                } else if from.x == 7 {
                                    board_state.black_king_castle = false;
                                }
                            }
                        }
                    }
                    _ => (),
                }
                *board_state.get_mut(to) = result_piece;
            }
            ActionType::Castling { kings_side } => {
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
            ActionType::EnPassant { from, to } => {
                *board_state.get_mut(to) = Some(Piece::new(color, PieceType::Pawn));
                *board_state.get_mut(from) = None;
                *board_state.get_mut(BoardPosition::new(to.x, from.y)) = None;
            }
        }
    }
    pub fn get_action_type(&self) -> ActionType {
        self.action_type
    }
}
pub fn find_legal_actions(board_state: &BoardState) -> Vec<Action> {
    let mut legal_actions = Vec::with_capacity(35);
    action_rules::PawnActions::update_actions(&board_state, &mut legal_actions);
    action_rules::KnightActions::update_actions(&board_state, &mut legal_actions);
    action_rules::DiagonalActions::update_actions(&board_state, &mut legal_actions);
    action_rules::StraightActions::update_actions(&board_state, &mut legal_actions);
    action_rules::KingActions::update_actions(&board_state, &mut legal_actions);
    action_rules::CastlingActions::update_actions(&board_state, &mut legal_actions);
    action_rules::RemoveIllegalActions::update_actions(&board_state, &mut legal_actions);
    legal_actions
}

pub fn in_check(board_state: &BoardState) -> bool {
    // TODO: optomize?

    let king_color = board_state.color_turn.opposite_color();
    let mut king_pos = None;
    for y in 0..8 {
        for x in 0..8 {
            if let Some(piece) = board_state.get(BoardPosition::new(x, y)) {
                if piece.piece_type == PieceType::King && piece.color == king_color {
                    king_pos = Some(BoardPosition::new(x, y));
                }
            }
        }
    }

    if let Some(king_pos) = king_pos {
        let possible_opponent_moves = find_legal_actions(&board_state);
        for possible_move in possible_opponent_moves {
            match possible_move.action_type {
                ActionType::SimpleMove { from: _, to } => {
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

pub fn in_check_mate(board_state: &BoardState) -> bool {
    // TODO: optomize?

    // if Im in check and I cant play any moves then Im in checkmate

    if find_legal_actions(board_state).is_empty() {
        let mut opponent_turn_board_state = board_state.clone();
        opponent_turn_board_state.color_turn =
            opponent_turn_board_state.color_turn.opposite_color();
        if in_check(&opponent_turn_board_state) {
            return true;
        }
    }
    return false;
}
