pub enum GameResult {
    Win,
    Draw,
    Lose,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

impl std::ops::Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

pub type Tile = Option<Player>;

pub trait Board {
    fn is_win(&self, player: Player) -> bool;
    fn is_draw(&self) -> bool;
    fn is_lose(&self, player: Player) -> bool;
    fn game_state(&self, player: Player) -> Option<GameResult>;
}

impl Board for [Tile; 9] {
    fn is_win(&self, player: Player) -> bool {
        // Use array to avoid heap allocations
        let mut v = [false; 9];
        (0..9).for_each(|x| v[x] = self[x] == Some(player));

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

    fn is_lose(&self, player: Player) -> bool {
        self.is_win(!player)
    }

    fn is_draw(&self) -> bool {
        !self.is_win(Player::X) && !self.is_lose(Player::X) && self.iter().all(|x| x.is_some())
    }

    fn game_state(&self, player: Player) -> Option<GameResult> {
        if self.is_win(player) {
            Some(GameResult::Win)
        } else if self.is_lose(player) {
            Some(GameResult::Lose)
        } else if self.is_draw() {
            Some(GameResult::Draw)
        } else {
            None
        }
    }
}

pub fn print_board(board: &[Tile; 9]) {
    let tile_to_char = |x: &Tile| match x {
        Some(Player::X) => 'X',
        Some(Player::O) => 'O',
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