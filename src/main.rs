type Player = bool;

type Tile = Option<Player>;

type Board = [Tile; 9];

enum GameResult {
    Win,
    Draw,
    Lose,
}

fn is_win(player: Player, board: &Board) -> bool {
    // Use array to avoid heap allocations
    let mut v = [false; 9];
    (0..9).for_each(|x| v[x] = board[x] == Some(player));

    // Horizontal win conditions
    let h1 = v[0] && v[1] && v[2];
    let h2 = v[3] && v[4] && v[5];
    let h3 = v[6] && v[7] && v[8];

    // Vertical win conditions
    let v1 = v[0] && v[3] && v[6];
    let v2 = v[1] && v[4] && v[7];
    let v3 = v[2] && v[5] && v[8];

    // Diagonal win conditions
    let d1 = v[0] && v[4] && v[8];
    let d2 = v[2] && v[4] && v[6];

    // Return true if any condition is met
    h1 || h2 || h3 || v1 || v2 || v3 || d1 || d2
}

fn is_lose(player: Player, board: &Board) -> bool {
    is_win(!player, board)
}

fn is_draw(board: &Board) -> bool {
    !is_win(true, board) && !is_lose(true, board) && board.iter().all(|x| x.is_some())
}

fn game_state(player: Player, board: &Board) -> Option<GameResult> {
    use GameResult::{Draw, Lose, Win};

    if is_win(player, board) {
        Some(Win)
    } else if is_lose(player, board) {
        Some(Lose)
    } else if is_draw(board) {
        Some(Draw)
    } else {
        None
    }
}

// returns (index, score) pair, the index corresponding to the index of the best move
// and the score corresponding to the score that move got.
fn minmax(player: Player, board: &Board) -> (Option<usize>, i32) {
    use GameResult::{Draw, Lose, Win};

    match game_state(player, board) {
        Some(Win) => (None, 1),
        Some(Draw) => (None, 0),
        Some(Lose) => (None, -1),
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
fn play(player: Player, mut board: Board) -> Board {
    let best_play = minmax(player, &board).0.expect("Finished game");
    board[best_play] = Some(player);
    board
}

fn print_board(board: &Board) {
    let tile_to_char = |x: &Tile| match x {
        Some(true) => 'X',
        Some(false) => 'O',
        None => ' ',
    };

    for (i, x) in board.iter().map(tile_to_char).enumerate() {
        print!("{}", x);
        if i % 3 != 2 {
            print!("|");
        } else if i != 8 {
            println!("\n-----");
        } else {
            println!("\n");
        }
    }
}


fn main() {
    println!("Hello, world!");
}
