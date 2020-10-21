pub type Player = bool;

pub const X: Player = true;
pub const O: Player = false;

mod board {
    pub type Tile = Option<Player>;

    pub type Board = [Tile; 9];

    pub fn new() -> Board {
        [None; 9]
    }

    pub fn is_win(player: Player, board: &Board) -> bool {
        // Use array to avoid heap allocations
        let mut v = [false; 9];
        (0..9).for_each(|x| v[x] = board[x].map_or(false, |p| p == player));

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

    pub fn is_lose(player: Player, board: &Board) -> bool {
        is_win(!player, board)
    }

    pub fn is_draw(board: &Board) -> bool {
        !is_win(player::X, board) && !is_lose(player::X, board) && board.iter().all(|x| x.is_some())
    }

    pub fn game_state(player: Player, board: &Board) -> Option<i8> {
        if is_win(player, board) {
            Some(1)
        } else if is_lose(player, board) {
            Some(-1)
        } else if is_draw(board) {
            Some(0)
        } else {
            None
        }
    }

    pub fn print(board: &Board) {
        let tile_to_char = |x: &Tile| match x {
            Some(player::X) => 'X',
            Some(player::O) => 'O',
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
}
