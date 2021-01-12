use super::find_legal_actions;
use crate::{Action, ActionType, BoardPosition, BoardState, Piece, PieceColor, PieceType};

fn test_moves(actions: Vec<Action>) -> BoardState {
    let mut board_state = BoardState::default();
    for action in actions {
        let legal_moves = find_legal_actions(&board_state);
        assert!(
            legal_moves.contains(&action),
            format!(
                "did not recognise {:?} as legal move with board state: {:?}",
                action, board_state
            )
        );
        action.play_move(&mut board_state);
    }
    board_state
}

#[test]
fn en_passant() {
    // e4 a6 e5 f5 exf6?

    test_moves(vec![
        Action::new(ActionType::SimpleMove {
            from: BoardPosition::from_text("e2"),
            to: BoardPosition::from_text("e4"),
        }),
        Action::new(ActionType::SimpleMove {
            from: BoardPosition::from_text("a7"),
            to: BoardPosition::from_text("a6"),
        }),
        Action::new(ActionType::SimpleMove {
            from: BoardPosition::from_text("e4"),
            to: BoardPosition::from_text("e5"),
        }),
        Action::new(ActionType::SimpleMove {
            from: BoardPosition::from_text("f7"),
            to: BoardPosition::from_text("f5"),
        }),
        Action::new(ActionType::EnPassant {
            from: BoardPosition::from_text("e5"),
            to: BoardPosition::from_text("f6"),
        }),
    ]);
}

#[test]
fn queening_test() {
    // f4 a6 f5 a5 f6 a4 fxg7 gf6 gxh8?

    let board_state = test_moves(vec![
        Action::new(ActionType::SimpleMove {
            from: BoardPosition::from_text("f2"),
            to: BoardPosition::from_text("f4"),
        }),
        Action::new(ActionType::SimpleMove {
            from: BoardPosition::from_text("a7"),
            to: BoardPosition::from_text("a6"),
        }),
        Action::new(ActionType::SimpleMove {
            from: BoardPosition::from_text("f4"),
            to: BoardPosition::from_text("f5"),
        }),
        Action::new(ActionType::SimpleMove {
            from: BoardPosition::from_text("a6"),
            to: BoardPosition::from_text("a5"),
        }),
        Action::new(ActionType::SimpleMove {
            from: BoardPosition::from_text("f5"),
            to: BoardPosition::from_text("f6"),
        }),
        Action::new(ActionType::SimpleMove {
            from: BoardPosition::from_text("a5"),
            to: BoardPosition::from_text("a4"),
        }),
        Action::new(ActionType::SimpleMove {
            from: BoardPosition::from_text("f6"),
            to: BoardPosition::from_text("g7"),
        }),
        Action::new(ActionType::SimpleMove {
            from: BoardPosition::from_text("g8"),
            to: BoardPosition::from_text("f6"),
        }),
        Action::new(ActionType::SimpleMove {
            from: BoardPosition::from_text("g7"),
            to: BoardPosition::from_text("h8"),
        }),
    ]);
    assert!(
        board_state
            .get(BoardPosition::from_text("h8"))
            .unwrap()
            .piece_type
            == PieceType::Queen,
        "queen was not got from queening {:?}",
        board_state
    )
}
