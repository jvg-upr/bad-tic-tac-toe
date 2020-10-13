mod game;
mod ai;

use game::{Player, Tile, Board};

fn main() {
    let mut board = [None; 9];
    let mut input = String::new();

    let mut x = 0;

    while board.game_state(Player::X).is_none() {
        game::print_board(&board);

        if x % 2 == 0 {
            board = ai::play(Player::O, board);
        } else {
            std::io::stdin().read_line(&mut input).unwrap();

            let index = input.trim().parse::<usize>().unwrap() - 1;
            input.clear();

            board[index] = Some(Player::X);
        }
        x += 1;
    }

    game::print_board(&board);
}
