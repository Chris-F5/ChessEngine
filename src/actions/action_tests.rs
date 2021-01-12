use super::find_legal_actions;
use crate::{Action, ActionType, BoardPosition, BoardState, Piece, PieceColor, PieceType};

fn test_move(board_state: BoardState, action: Action) -> BoardState {
    let legal_moves = find_legal_actions(&board_state);
    assert!(
        legal_moves.contains(&action),
        format!(
            "did not recognise {:?} as legal move with board state: {:?}",
            action, board_state
        )
    );
    let mut new_board_state = board_state.clone();
    action.play_move(&mut new_board_state);
    new_board_state
}

#[test]
fn en_passant() {
    test_move(
        BoardState::from_fen("rnbqkbnr/1pppp1pp/p7/4Pp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3"),
        Action::new(ActionType::EnPassant {
            from: BoardPosition::from_text("e5"),
            to: BoardPosition::from_text("f6"),
        }),
    );
    test_move(
        BoardState::from_fen("rnbqkbnr/ppp1pppp/8/8/3pP2P/8/PPPP1PP1/RNBQKBNR b KQkq e3 0 3"),
        Action::new(ActionType::EnPassant {
            from: BoardPosition::from_text("d4"),
            to: BoardPosition::from_text("e3"),
        }),
    );
}

#[test]
fn queening_test() {
    let assert_queen_made = |board_fen: &str, from_str: &str, to_str: &str| {
        assert!(
            test_move(
                BoardState::from_fen(board_fen),
                Action::new(ActionType::SimpleMove {
                    from: BoardPosition::from_text(from_str),
                    to: BoardPosition::from_text(to_str),
                }),
            )
            .get(BoardPosition::from_text(to_str))
            .unwrap()
            .piece_type
                == PieceType::Queen,
            format!("queen was not made after queening")
        );
    };
    // White queening
    assert_queen_made(
        "rnbqkb1r/1pppppPp/5n2/8/p7/8/PPPPP1PP/RNBQKBNR w KQkq - 1 5",
        "g7",
        "h8",
    );
    assert_queen_made(
        "rnbqkb1r/1pppppPp/5n2/8/p7/8/PPPPP1PP/RNBQKBNR w KQkq - 1 5",
        "g7",
        "g8",
    );
    assert_queen_made(
        "rnbqkb1r/1pppppPp/5n2/8/p7/8/PPPPP1PP/RNBQKBNR w KQkq - 1 5",
        "g7",
        "f8",
    );
    // Black queening
    assert_queen_made(
        "rnbqkbnr/ppppp1pp/P7/8/8/7N/1PPPPPpP/RNBQKB1R b KQkq - 0 5",
        "g2",
        "h1",
    );
    assert_queen_made(
        "rnbqkbnr/ppppp1pp/P7/8/8/7N/1PPPPPpP/RNBQKB1R b KQkq - 0 5",
        "g2",
        "g1",
    );
    assert_queen_made(
        "rnbqkbnr/ppppp1pp/P7/8/8/7N/1PPPPPpP/RNBQKB1R b KQkq - 0 5",
        "g2",
        "f1",
    );
}

#[test]
fn castling() {
    let white_king_castled = test_move(
        BoardState::from_fen("r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4"),
        Action::new(ActionType::Castling { kings_side: true }),
    );
    assert!(
        white_king_castled
            .get(BoardPosition::from_text("g1"))
            .unwrap()
            .piece_type
            == PieceType::King,
    );
    assert!(
        white_king_castled
            .get(BoardPosition::from_text("f1"))
            .unwrap()
            .piece_type
            == PieceType::Rook,
    );
    assert!(white_king_castled
        .get(BoardPosition::from_text("e1"))
        .is_none());
    assert!(white_king_castled
        .get(BoardPosition::from_text("h1"))
        .is_none());
}
