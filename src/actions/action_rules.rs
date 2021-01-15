use crate::{
    Action, ActionType, BoardPosition, BoardState, Capturable, Piece, PieceColor, PieceType,
};

pub trait ActionRule {
    fn update_actions(board_state: &BoardState, actions: &mut Vec<Action>);
}

fn move_or_capture(
    board_state: &BoardState,
    from: BoardPosition,
    to: BoardPosition,
) -> (Option<Action>, bool) {
    debug_assert!(
        board_state
            .get(from)
            .expect("cant move from an empty square")
            .color
            == board_state.color_turn,
        "cant move from a square of the wrong color"
    );
    let to_piece = board_state.get(to);
    if to_piece.is_none() {
        (
            Some(Action::new(ActionType::SimpleMove { from, to })),
            false,
        )
    } else if to_piece.can_take(board_state.color_turn) {
        (Some(Action::new(ActionType::SimpleMove { from, to })), true)
    } else {
        (None, true)
    }
}

pub struct PawnActions;
impl PawnActions {
    fn move_one(board_state: &BoardState, pos: BoardPosition) -> Option<Action> {
        let forward_pos = pos.directional_ofset(0, 1, board_state.color_turn);
        if board_state.get(forward_pos).is_none() {
            Some(Action {
                action_type: ActionType::SimpleMove {
                    from: pos,
                    to: forward_pos,
                },
            })
        } else {
            None
        }
    }
    fn move_one_or_two(
        board_state: &BoardState,
        pos: BoardPosition,
    ) -> Option<(Action, Option<Action>)> {
        let forward_one_pos = pos.directional_ofset(0, 1, board_state.color_turn);
        if board_state.get(forward_one_pos).is_none() {
            let forward_one_action = Action {
                action_type: ActionType::SimpleMove {
                    from: pos,
                    to: forward_one_pos,
                },
            };
            let forward_two_pos = pos.directional_ofset(0, 2, board_state.color_turn);
            if board_state.get(forward_two_pos).is_none() {
                let forward_two_action = Action {
                    action_type: ActionType::SimpleMove {
                        from: pos,
                        to: forward_two_pos,
                    },
                };
                Some((forward_one_action, Some(forward_two_action)))
            } else {
                Some((forward_one_action, None))
            }
        } else {
            None
        }
    }
    fn capture(board_state: &BoardState, pos: BoardPosition) -> (Option<Action>, Option<Action>) {
        let left_capture_pos = pos.directional_ofset(-1, 1, board_state.color_turn);
        let left_capture_action = if left_capture_pos.x_bound_check()
            && board_state
                .get(left_capture_pos)
                .can_take(board_state.color_turn)
        {
            Some(Action::new(ActionType::SimpleMove {
                from: pos,
                to: left_capture_pos,
            }))
        } else {
            None
        };

        let right_capture_pos = pos.directional_ofset(1, 1, board_state.color_turn);
        let right_capture_action = if right_capture_pos.x_bound_check()
            && board_state
                .get(right_capture_pos)
                .can_take(board_state.color_turn)
        {
            Some(Action::new(ActionType::SimpleMove {
                from: pos,
                to: right_capture_pos,
            }))
        } else {
            None
        };
        (left_capture_action, right_capture_action)
    }
    fn en_passant(
        board_state: &BoardState,
        pos: BoardPosition,
    ) -> (Option<Action>, Option<Action>) {
        let left_en_passant_action = if pos.x != 0 && board_state.en_passant_colunm == pos.x - 1 {
            let left_forward_pos = pos.directional_ofset(-1, 1, board_state.color_turn);
            Some(Action::new(ActionType::EnPassant {
                from: pos,
                to: left_forward_pos,
            }))
        } else {
            None
        };
        let right_en_passant_action = if pos.x != 7 && board_state.en_passant_colunm == pos.x + 1 {
            let right_forward_pos = pos.directional_ofset(1, 1, board_state.color_turn);
            Some(Action::new(ActionType::EnPassant {
                from: pos,
                to: right_forward_pos,
            }))
        } else {
            None
        };
        (left_en_passant_action, right_en_passant_action)
    }
}
impl ActionRule for PawnActions {
    fn update_actions(board_state: &BoardState, actions: &mut Vec<Action>) {
        let y_range = if board_state.color_turn == PieceColor::White {
            1..8
        } else {
            0..7
        };
        for y in y_range {
            let can_move_two = (y == 1 && board_state.color_turn == PieceColor::White)
                || (y == 6 && board_state.color_turn == PieceColor::Black);
            let can_en_passant = (y == 4 && board_state.color_turn == PieceColor::White)
                || (y == 3 && board_state.color_turn == PieceColor::Black);
            for x in 0..8 {
                let pos = BoardPosition::new(x, y);
                if let Some(piece) = board_state.get(pos) {
                    if piece.color == board_state.color_turn && piece.piece_type == PieceType::Pawn
                    {
                        if can_move_two {
                            if let Some(move_actions) =
                                PawnActions::move_one_or_two(&board_state, pos)
                            {
                                actions.push(move_actions.0);
                                if let Some(move_two_action) = move_actions.1 {
                                    actions.push(move_two_action);
                                }
                            }
                        } else {
                            if let Some(action) = PawnActions::move_one(&board_state, pos) {
                                actions.push(action);
                            }
                            if can_en_passant {
                                let en_passant_actions = PawnActions::en_passant(&board_state, pos);
                                if let Some(left_en_passant_action) = en_passant_actions.0 {
                                    actions.push(left_en_passant_action);
                                } else if let Some(right_en_passant_action) = en_passant_actions.1 {
                                    actions.push(right_en_passant_action);
                                }
                            }
                        }
                        let capture_actions = PawnActions::capture(&board_state, pos);
                        if let Some(left_capture_action) = capture_actions.0 {
                            actions.push(left_capture_action)
                        }
                        if let Some(right_capture_action) = capture_actions.1 {
                            actions.push(right_capture_action)
                        }
                    }
                }
            }
        }
    }
}
pub struct KnightActions;
impl ActionRule for KnightActions {
    fn update_actions(board_state: &BoardState, actions: &mut Vec<Action>) {
        let mut try_knight_move = move |from: BoardPosition, to: BoardPosition| {
            if to.bound_check() {
                let possible_action = move_or_capture(board_state, from, to).0;
                if let Some(action) = possible_action {
                    actions.push(action);
                }
            }
        };
        for y in 0..8 {
            for x in 0..8 {
                let board_pos = BoardPosition::new(x, y);
                if let Some(piece) = board_state.get(board_pos) {
                    if piece.piece_type == PieceType::Knight
                        && piece.color == board_state.color_turn
                    {
                        try_knight_move(board_pos, board_pos.nondirectional_ofset(1, 2));
                        try_knight_move(board_pos, board_pos.nondirectional_ofset(2, 1));
                        try_knight_move(board_pos, board_pos.nondirectional_ofset(-1, 2));
                        try_knight_move(board_pos, board_pos.nondirectional_ofset(-2, 1));
                        try_knight_move(board_pos, board_pos.nondirectional_ofset(1, -2));
                        try_knight_move(board_pos, board_pos.nondirectional_ofset(2, -1));
                        try_knight_move(board_pos, board_pos.nondirectional_ofset(-1, -2));
                        try_knight_move(board_pos, board_pos.nondirectional_ofset(-2, -1));
                    }
                }
            }
        }
    }
}
pub struct DiagonalActions;
impl ActionRule for DiagonalActions {
    fn update_actions(board_state: &BoardState, actions: &mut Vec<Action>) {
        let mut try_move = move |from: BoardPosition, to: BoardPosition| -> bool {
            if to.bound_check() {
                let (possible_action, stop) = move_or_capture(board_state, from, to);
                if let Some(action) = possible_action {
                    actions.push(action);
                    stop
                } else {
                    true
                }
            } else {
                true
            }
        };
        for y in 0..8 {
            for x in 0..8 {
                let board_pos = BoardPosition::new(x, y);
                if let Some(piece) = board_state.get(board_pos) {
                    if piece.color == board_state.color_turn
                        && (piece.piece_type == PieceType::Bishop
                            || piece.piece_type == PieceType::Queen)
                    {
                        let mut x_to = x;
                        let mut y_to = y;
                        loop {
                            x_to += 1;
                            y_to += 1;
                            if try_move(board_pos, BoardPosition::new(x_to, y_to)) {
                                break;
                            }
                        }
                        let mut x_to = x;
                        let mut y_to = y;
                        loop {
                            if x_to != 0 {
                                x_to -= 1;
                            } else {
                                break;
                            }
                            y_to += 1;
                            if try_move(board_pos, BoardPosition::new(x_to, y_to)) {
                                break;
                            }
                        }
                        let mut x_to = x;
                        let mut y_to = y;
                        loop {
                            x_to += 1;
                            if y_to != 0 {
                                y_to -= 1;
                            } else {
                                break;
                            }
                            if try_move(board_pos, BoardPosition::new(x_to, y_to)) {
                                break;
                            }
                        }
                        let mut x_to = x;
                        let mut y_to = y;
                        loop {
                            if x_to != 0 {
                                x_to -= 1;
                            } else {
                                break;
                            }
                            if y_to != 0 {
                                y_to -= 1;
                            } else {
                                break;
                            }
                            if try_move(board_pos, BoardPosition::new(x_to, y_to)) {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct StraightActions;
impl ActionRule for StraightActions {
    fn update_actions(board_state: &BoardState, actions: &mut Vec<Action>) {
        let mut try_move = move |from: BoardPosition, to: BoardPosition| -> bool {
            if to.bound_check() {
                let (possible_action, stop) = move_or_capture(board_state, from, to);
                if let Some(action) = possible_action {
                    actions.push(action);
                    stop
                } else {
                    true
                }
            } else {
                true
            }
        };
        for y in 0..8 {
            for x in 0..8 {
                let board_pos = BoardPosition::new(x, y);
                if let Some(piece) = board_state.get(board_pos) {
                    if piece.color == board_state.color_turn
                        && (piece.piece_type == PieceType::Rook
                            || piece.piece_type == PieceType::Queen)
                    {
                        let mut x_to = x;
                        loop {
                            x_to += 1;
                            if try_move(board_pos, BoardPosition::new(x_to, y)) {
                                break;
                            }
                        }
                        let mut x_to = x;
                        loop {
                            if x_to != 0 {
                                x_to -= 1;
                            } else {
                                break;
                            }
                            if try_move(board_pos, BoardPosition::new(x_to, y)) {
                                break;
                            }
                        }
                        let mut y_to = y;
                        loop {
                            y_to += 1;
                            if try_move(board_pos, BoardPosition::new(x, y_to)) {
                                break;
                            }
                        }
                        let mut y_to = y;
                        loop {
                            if y_to != 0 {
                                y_to -= 1;
                            } else {
                                break;
                            }
                            if try_move(board_pos, BoardPosition::new(x, y_to)) {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}
pub struct KingActions;
impl ActionRule for KingActions {
    fn update_actions(board_state: &BoardState, actions: &mut Vec<Action>) {
        let mut try_move = move |from: BoardPosition, to: BoardPosition| {
            if to.bound_check() {
                let possible_action = move_or_capture(board_state, from, to).0;
                if let Some(action) = possible_action {
                    actions.push(action);
                }
            }
        };
        for y in 0..8 {
            for x in 0..8 {
                let board_pos = BoardPosition::new(x, y);
                if let Some(piece) = board_state.get(board_pos) {
                    if piece.color == board_state.color_turn && piece.piece_type == PieceType::King
                    {
                        try_move(board_pos, board_pos.nondirectional_ofset(0, 1));
                        try_move(board_pos, board_pos.nondirectional_ofset(1, 1));
                        try_move(board_pos, board_pos.nondirectional_ofset(1, 0));
                        try_move(board_pos, board_pos.nondirectional_ofset(1, -1));
                        try_move(board_pos, board_pos.nondirectional_ofset(0, -1));
                        try_move(board_pos, board_pos.nondirectional_ofset(-1, -1));
                        try_move(board_pos, board_pos.nondirectional_ofset(-1, 0));
                        try_move(board_pos, board_pos.nondirectional_ofset(-1, 1));
                    }
                }
            }
        }
    }
}
pub struct CastlingActions;
impl CastlingActions {
    fn positions_in_check(board_state: &BoardState, positions: Vec<BoardPosition>) -> bool {
        debug_assert!(positions.len() > 0);
        // TODO: optomise by reversing search?

        let mut board_state = board_state.clone();

        // fill with kings so pawns can capture
        for position in positions.clone() {
            *board_state.get_mut(position) =
                Some(Piece::new(board_state.color_turn, PieceType::King));
        }
        let mut possible_opponent_moves = Vec::new();
        PawnActions::update_actions(&board_state, &mut possible_opponent_moves);
        KnightActions::update_actions(&board_state, &mut possible_opponent_moves);
        DiagonalActions::update_actions(&board_state, &mut possible_opponent_moves);
        StraightActions::update_actions(&board_state, &mut possible_opponent_moves);
        KingActions::update_actions(&board_state, &mut possible_opponent_moves);
        for possible_move in possible_opponent_moves {
            match possible_move.action_type {
                ActionType::SimpleMove { from: _, to } => {
                    for king_pos in positions.clone() {
                        if king_pos == to {
                            return true;
                        }
                    }
                }
                _ => (),
            }
        }
        return false;
    }
}
impl ActionRule for CastlingActions {
    fn update_actions(board_state: &BoardState, actions: &mut Vec<Action>) {
        if board_state.color_turn == PieceColor::White {
            if board_state.white_king_castle {
                if !Self::positions_in_check(
                    &board_state,
                    vec![BoardPosition::new(4, 0), BoardPosition::new(5, 0)],
                ) {
                    actions.push(Action::new(ActionType::Castling { kings_side: true }));
                }
            }
            if board_state.white_queen_castle {
                if !Self::positions_in_check(
                    &board_state,
                    vec![BoardPosition::new(4, 0), BoardPosition::new(3, 0)],
                ) {
                    actions.push(Action::new(ActionType::Castling { kings_side: true }));
                }
            }
        } else {
            if board_state.black_king_castle {
                if !Self::positions_in_check(
                    &board_state,
                    vec![BoardPosition::new(4, 7), BoardPosition::new(5, 7)],
                ) {
                    actions.push(Action::new(ActionType::Castling { kings_side: true }));
                }
            }
            if board_state.black_queen_castle {
                if !Self::positions_in_check(
                    &board_state,
                    vec![BoardPosition::new(4, 7), BoardPosition::new(3, 7)],
                ) {
                    actions.push(Action::new(ActionType::Castling { kings_side: true }));
                }
            }
        }
    }
}
pub struct RemoveIllegalActions;
impl ActionRule for RemoveIllegalActions {
    fn update_actions(board_state: &BoardState, actions: &mut Vec<Action>) {
        actions.retain(|action| {
            let mut new_board_state = board_state.clone();
            action.play_move(&mut new_board_state);
            !in_check(&new_board_state)
        });
    }
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
        let mut possible_opponent_moves = Vec::new();
        PawnActions::update_actions(&board_state, &mut possible_opponent_moves);
        KnightActions::update_actions(&board_state, &mut possible_opponent_moves);
        DiagonalActions::update_actions(&board_state, &mut possible_opponent_moves);
        StraightActions::update_actions(&board_state, &mut possible_opponent_moves);
        KingActions::update_actions(&board_state, &mut possible_opponent_moves);
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
        panic!("this color has no king {:?}", board_state);
    }
}
pub struct RemoveUnsafeActions;
impl RemoveUnsafeActions {
    fn is_safe(action: &Action, board_state: &BoardState) -> bool {
        match action.action_type {
            ActionType::SimpleMove { from: _, to } => board_state.get(to).is_none(),
            ActionType::EnPassant { from: _, to: _ } => false,
            ActionType::Castling { kings_side: _ } => true,
        }
    }
}
impl ActionRule for RemoveUnsafeActions {
    fn update_actions(board_state: &BoardState, actions: &mut Vec<Action>) {
        actions.retain(|action| Self::is_safe(&action, board_state));
    }
}
