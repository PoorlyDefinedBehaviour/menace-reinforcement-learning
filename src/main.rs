use std::collections::HashMap;

// NOTE: The board could be represented by a single integer.
type Board = [[char; 3]; 3];

static N: usize = 4;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Bead {
    White,
    Lilac,
    Silver,
    Black,
    Gold,
    Green,
    Amber,
    Red,
    Pink,
}

#[derive(Debug, Clone, Copy)]
enum PlayMode {
    Best,
    Random,
}

#[derive(Debug)]
struct Matchbox {
    beads: HashMap<Bead, usize>,
}

impl Matchbox {
    fn new() -> Self {
        let mut beads = HashMap::new();
        for bead in [
            Bead::White,
            Bead::Lilac,
            Bead::Silver,
            Bead::Black,
            Bead::Gold,
            Bead::Green,
            Bead::Amber,
            Bead::Red,
            Bead::Pink,
        ] {
            beads.insert(bead, 4 * N);
        }

        Self { beads }
    }

    fn add(&mut self, bead: Bead) {
        *self.beads.entry(bead).or_insert(0) += 1;
    }

    fn remove(&mut self, bead: Bead) {
        if let Some(count) = self.beads.get_mut(&bead) {
            // Always keep at least one bead in the matchbox.
            *count = std::cmp::max(1, *count - 1);
        }
    }
}

fn main() {
    let mut p1 = Player::new('x', HashMap::new());
    let mut p2 = Player::new('o', HashMap::new());
    let cfg = TrainConfig {
        games: 100_000,
        p1_win_reward: 3,
        p1_loss_punishment: 1,
        p2_win_reward: 3,
        p2_loss_punishment: 1,
        p1_tie_reward: 1,
        p2_tie_reward: 1,
        p1_play_mode: PlayMode::Random,
        p2_play_mode: PlayMode::Random,
    };

    for i in 0..50 {
        let result = train(&mut p1, &mut p2, cfg.clone());
        println!("i={i} p1 p2 result {result:?}");
    }

    let result = train(
        &mut p1,
        &mut p2,
        TrainConfig {
            games: 100_000,
            p1_win_reward: 0,
            p1_loss_punishment: 0,
            p2_win_reward: 0,
            p2_loss_punishment: 0,
            p1_tie_reward: 0,
            p2_tie_reward: 0,
            p1_play_mode: PlayMode::Best,
            p2_play_mode: PlayMode::Best,
        },
    );

    println!("best play result {result:?}");

    loop {
        let mut board = [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']];
        let mut turn = true;

        loop {
            for row in board {
                println!("{row:?}");
            }
            println!("---");

            if has_won('x', &board) {
                println!("x won\n---");
                break;
            }

            if has_won('o', &board) {
                println!("o won\n---");
                break;
            }

            if is_board_complete(&board) {
                println!("tie\n---");
                break;
            }

            if turn {
                let (_, (i, j)) = get_ai_play(&board, &mut p1.matchboxes, PlayMode::Best);
                board[i][j] = 'o';
            } else {
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                let line = line.trim_end();
                let (i, j) = line.split_once(" ").unwrap();
                let i: usize = i.parse().unwrap();
                let j: usize = j.parse().unwrap();
                board[i][j] = 'x';
            }

            turn = !turn;
        }
    }
}

fn get_ai_play(
    board: &Board,
    matchboxes: &mut HashMap<Board, Matchbox>,
    mode: PlayMode,
) -> (Bead, (usize, usize)) {
    let matchbox = matchboxes.entry(*board).or_insert_with(Matchbox::new);

    let mut available_beads = Vec::new();
    let mut total_bead_count = 0;

    for (bead, &count) in matchbox.beads.iter() {
        let (i, j) = bead_to_position(*bead);
        if board[i][j] == ' ' {
            available_beads.push((*bead, count));
            total_bead_count += count;
        }
    }

    if available_beads.is_empty() {
        for i in 0..board.len() {
            for j in 0..board.len() {
                if board[i][j] == ' ' {
                    return (position_to_bead(i, j), (i, j));
                }
            }
        }
    }

    match mode {
        PlayMode::Best => {
            let (bead, _) = available_beads
                .into_iter()
                .max_by_key(|(_, count)| *count)
                .unwrap();
            (bead, bead_to_position(bead))
        }
        PlayMode::Random => {
            let mut r = rand::random_range(0..total_bead_count);
            for (bead, count) in available_beads {
                if r < count {
                    return (bead, bead_to_position(bead));
                }
                r -= count;
            }

            unreachable!();
        }
    }
}

fn has_won(player: char, board: &Board) -> bool {
    for row in board.iter() {
        if row.iter().all(|v| *v == player) {
            return true;
        }
    }

    for column in 0..board.len() {
        if (0..board.len())
            .map(|row| board[row][column])
            .all(|v| v == player)
        {
            return true;
        }
    }

    if board[0][0] == player && board[1][1] == player && board[2][2] == player {
        return true;
    }

    if board[0][2] == player && board[1][1] == player && board[2][0] == player {
        return true;
    }

    false
}

fn is_board_complete(board: &Board) -> bool {
    for row in board {
        for cell in row {
            if *cell == ' ' {
                return false;
            }
        }
    }

    true
}

fn bead_to_position(bead: Bead) -> (usize, usize) {
    match bead {
        Bead::White => (0, 0),
        Bead::Lilac => (0, 1),
        Bead::Silver => (0, 2),
        Bead::Black => (1, 0),
        Bead::Gold => (1, 1),
        Bead::Green => (1, 2),
        Bead::Amber => (2, 0),
        Bead::Red => (2, 1),
        Bead::Pink => (2, 2),
    }
}

fn position_to_bead(i: usize, j: usize) -> Bead {
    match (i, j) {
        (0, 0) => Bead::White,
        (0, 1) => Bead::Lilac,
        (0, 2) => Bead::Silver,
        (1, 0) => Bead::Black,
        (1, 1) => Bead::Gold,
        (1, 2) => Bead::Green,
        (2, 0) => Bead::Amber,
        (2, 1) => Bead::Red,
        (2, 2) => Bead::Pink,
        _ => unreachable!("unknown position i={i} j={j}"),
    }
}

struct Player {
    symbol: char,
    matchboxes: HashMap<Board, Matchbox>,
}

impl Player {
    fn new(symbol: char, matchboxes: HashMap<Board, Matchbox>) -> Self {
        Self { symbol, matchboxes }
    }
}

#[derive(Debug, Clone)]
struct TrainConfig {
    games: usize,
    p1_win_reward: usize,
    p1_loss_punishment: usize,
    p2_win_reward: usize,
    p2_loss_punishment: usize,
    p1_tie_reward: usize,
    p2_tie_reward: usize,
    p1_play_mode: PlayMode,
    p2_play_mode: PlayMode,
}

fn train(p1: &mut Player, p2: &mut Player, cfg: TrainConfig) -> TrainResult {
    assert_ne!(p1.symbol, p2.symbol, "players must have different symbols");

    let mut p1_wins = 0;
    let mut p2_wins = 0;
    let mut ties = 0;

    for _ in 0..cfg.games {
        let mut board = [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']];
        let mut choices_p2: Vec<(Board, Bead)> = Vec::new();
        let mut choices_p1: Vec<(Board, Bead)> = Vec::new();
        let mut turn = true;

        loop {
            if has_won(p1.symbol, &board) {
                for (board, bead) in choices_p1 {
                    let matchbox: &mut Matchbox = p1.matchboxes.get_mut(&board).unwrap();
                    for _ in 0..cfg.p1_win_reward {
                        matchbox.add(bead);
                    }
                }
                for (board, bead) in choices_p2 {
                    let matchbox: &mut Matchbox = p2.matchboxes.get_mut(&board).unwrap();
                    for _ in 0..cfg.p2_loss_punishment {
                        matchbox.remove(bead);
                    }
                }

                p1_wins += 1;

                break;
            }

            if has_won(p2.symbol, &board) {
                for (board, bead) in choices_p2 {
                    let matchbox: &mut Matchbox = p2.matchboxes.get_mut(&board).unwrap();
                    for _ in 0..cfg.p2_win_reward {
                        matchbox.add(bead);
                    }
                }
                for (board, bead) in choices_p1 {
                    let matchbox: &mut Matchbox = p1.matchboxes.get_mut(&board).unwrap();
                    for _ in 0..cfg.p1_loss_punishment {
                        matchbox.remove(bead);
                    }
                }

                p2_wins += 1;
                break;
            }

            if is_board_complete(&board) {
                for (board, bead) in choices_p1 {
                    let matchbox: &mut Matchbox = p1.matchboxes.get_mut(&board).unwrap();
                    for _ in 0..cfg.p1_tie_reward {
                        matchbox.add(bead);
                    }
                }

                for (board, bead) in choices_p2 {
                    let matchbox: &mut Matchbox = p2.matchboxes.get_mut(&board).unwrap();
                    for _ in 0..cfg.p2_tie_reward {
                        matchbox.add(bead);
                    }
                }

                ties += 1;
                break;
            }

            if turn {
                let (bead, (i, j)) = get_ai_play(&board, &mut p1.matchboxes, cfg.p1_play_mode);
                choices_p1.push((board, bead));
                board[i][j] = p1.symbol;
            } else {
                let (bead, (i, j)) = get_ai_play(&board, &mut p2.matchboxes, cfg.p2_play_mode);
                choices_p2.push((board, bead));
                board[i][j] = p2.symbol;
            }

            turn = !turn;
        }
    }

    TrainResult {
        p1_wins,
        p2_wins,
        ties,
    }
}

#[derive(Debug)]
struct TrainResult {
    p1_wins: usize,
    p2_wins: usize,
    ties: usize,
}
