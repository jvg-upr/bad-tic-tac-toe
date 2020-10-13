use crate::game::{player, board};

// returns (index, score) pair, the index corresponding to the index of the best move
// and the score corresponding to the score that move got.
pub fn minmax(player: player::Player, board: &board::Board) -> (Option<usize>, i8) {
    let tile_score = |x| {
        let mut b = board.clone();
        b[x] = Some(player);
        -minmax(!player, &b).1
    };

    match board::game_state(player, board) {
        Some(score) => (None, score),
        None => (0..9)
            .filter(|&x| board[x].is_none())
            .map(|x| (Some(x), tile_score(x)))
            .max_by_key(|x| x.1)
            .unwrap_or((None, 0)),
    }
}

// takes a board and a player and makes the best possible move on the board.
pub fn play(player: player::Player, mut board: board::Board) -> board::Board {
    let best_play = minmax(player, &board).0.expect("Finished game");
    board[best_play] = Some(player);
    board
}
