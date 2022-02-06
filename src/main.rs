pub type Player = bool;

pub const X: Player = true;
pub const O: Player = false;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
pub struct Board([Option<bool>; 9]);

impl std::ops::Deref for Board {
    type Target = [Option<bool>; 9];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let it = self
            .iter()
            .map(|t| t.map(|b| if b { 'X' } else { 'O' }).unwrap_or('_'))
            .enumerate();
        for (i, e) in it {
            if i % 3 == 2 {
                writeln!(f, "{}", e)?;
            } else {
                write!(f, "{}|", e)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GameState {
    Win,
    Lose,
    Draw,
    InProgress,
}

pub fn game_state(board: &Board) -> GameState {
    let conditions = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    let mut x_win = conditions
        .iter()
        .map(|row| row.iter().all(|&x| board[x] == Some(X)));
    let mut y_win = conditions
        .iter()
        .map(|row| row.iter().all(|&x| board[x] == Some(O)));
    let mut in_progress = board.iter().map(|&x| x == None);

    if x_win.any(|x| x) {
        GameState::Win
    } else if y_win.any(|x| x) {
        GameState::Lose
    } else if in_progress.any(|x| x) {
        GameState::InProgress
    } else {
        GameState::Draw
    }
}

pub fn minmax(player: Player, board: Board) -> (i8, Board) {
    match game_state(&board) {
        GameState::Win => (1, board),
        GameState::Lose => (-1, board),
        GameState::Draw => (0, board),
        GameState::InProgress => {
            let tile_scores = board
                .iter()
                .enumerate()
                .filter(|(_, tile)| tile.is_none())
                .map(|(i, _)| {
                    let mut board = board.clone();
                    board[i] = Some(player);
                    (minmax(!player, board.clone()).0, board)
                });

            if player {
                tile_scores.max_by_key(|(score, _)| *score).unwrap()
            } else {
                tile_scores.min_by_key(|(score, _)| *score).unwrap()
            }
        }
    }
}

fn main() {
    let mut board = Default::default();
    let mut input = String::new();

    for &player in [true, false].iter().cycle() {
        if let GameState::InProgress = game_state(&board) {
            println!("{}", board);
            if player {
                board = minmax(player, board).1;
            } else {
                println!("move: [0-8]");

                input.clear();
                std::io::stdin().read_line(&mut input).unwrap();

                let index = input.trim().parse::<usize>().unwrap();
                board[index] = Some(player);
            }
        } else {
            break;
        }
    }

    println!("{}", board);
    println!("{:?}", game_state(&board));
}
