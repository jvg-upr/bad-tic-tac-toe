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

fn main() {
    println!("Hello, world!");
}
