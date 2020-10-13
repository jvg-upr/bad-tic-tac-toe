use crate::game::{Player, Tile, Board, GameResult};

// returns (index, score) pair, the index corresponding to the index of the best move
// and the score corresponding to the score that move got.
pub fn minmax(player: Player, board: &[Tile; 9]) -> (Option<usize>, i32) {

    match board.game_state(player) {
        Some(GameResult::Win) => (None, 1),
        Some(GameResult::Draw) => (None, 0),
        Some(GameResult::Lose) => (None, -1),
        None => (0..9)
            .filter(|&x| board[x].is_none())
            .map(|x| {
                let mut b = board.clone();
                b[x] = Some(player);
                (Some(x), -minmax(!player, &b).1)
            })
            .max_by_key(|x| x.1)
            .unwrap_or((None, 0)),
    }
}

// takes a board and a player and makes the best possible move on the board.
pub fn play(player: Player, mut board: [Tile; 9]) -> [Tile; 9] {
    let best_play = minmax(player, &board).0.expect("Finished game");
    board[best_play] = Some(player);
    board
}
