use bad_tic_tac_toe::game::{player, board};
use bad_tic_tac_toe::ai;

fn main() {
    let mut board = board::new();
    let mut input = String::new();

    let mut x = 0;

    while board::game_state(player::X, &board).is_none() {
        board::print(&board);

        if x % 2 == 0 {
            board = ai::play(player::O, board);
        } else {
            std::io::stdin().read_line(&mut input).unwrap();

            let index = input.trim().parse::<usize>().unwrap() - 1;
            input.clear();

            board[index] = Some(player::X);
        }
        x += 1;
    }

    board::print(&board);
}
