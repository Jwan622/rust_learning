use tic_tac_toe::board::Board;

#[test]
fn test_full_game() {
    let mut game = Board::new();
    assert!(game.make_move(0, 0, 'X'));
    assert!(game.make_move(1, 1, 'O'));
    assert!(game.make_move(0, 1, 'X'));
    assert!(game.make_move(2, 2, 'O'));
    assert!(game.make_move(0, 2, 'X'));

    assert_eq!(game.check_winner(), Some('X'));
}
