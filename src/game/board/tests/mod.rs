use super::*;

#[test]
fn test_board_check() -> Result<(), TileSetError> {
    let mut board = Board::new();
    board.set(0, 0, &Sign::Circle)?;
    board.set(0, 1, &Sign::Circle)?;
    board.set(0, 2, &Sign::Circle)?;
    assert_eq!(board.check(6), GameState::CirclesWon);
    board = Board::new();
    board.set(0, 0, &Sign::Circle)?;
    board.set(0, 1, &Sign::Cross)?;
    board.set(0, 2, &Sign::Circle)?;
    assert_eq!(board.check(6), GameState::InProgress);
    board.set(1, 2, &Sign::Circle)?;
    board.set(2, 2, &Sign::Circle)?;
    assert_eq!(board.check(6), GameState::CirclesWon);
    board = Board::new();
    board.set(0, 0, &Sign::Circle)?;
    board.set(0, 1, &Sign::Cross)?;
    board.set(0, 2, &Sign::Circle)?;
    board.set(1, 0, &Sign::Cross)?;
    board.set(1, 1, &Sign::Cross)?;
    board.set(1, 2, &Sign::Circle)?;
    board.set(2, 0, &Sign::Circle)?;
    board.set(2, 1, &Sign::Circle)?;
    board.set(2, 2, &Sign::Cross)?;
    assert_eq!(board.check(9), GameState::Draw);
    board = Board::new();
    board.set(0, 2, &Sign::Cross)?;
    board.set(1, 1, &Sign::Cross)?;
    board.set(2, 0, &Sign::Cross)?;
    assert_eq!(board.check(6), GameState::CrossesWon);
    assert_eq!(board.set(2, 0, &Sign::Circle), Err(TileSetError { msg: String::from("Tile is not empty") } ));
    assert_eq!(board.set(3, 0, &Sign::Circle), Err(TileSetError { msg: String::from("No such tile (bad index)") } ));

    Ok(())
}

#[test]
fn test_tile_state_partial_eq() -> Result<(), TileSetError> {
    let mut tile1 = Tile::new();
    let mut tile2 = Tile::new();
    tile1.set(&Sign::Circle)?;
    tile2.set(&Sign::Circle)?;
    assert_eq!(tile1.state, tile2.state);
    let tile3 = Tile::new();
    assert_ne!(tile1.state, tile3.state);
    let mut tile4 = Tile::new();
    tile4.set(&Sign::Cross)?;
    assert_ne!(tile1.state, tile4.state);
    Ok(())
}