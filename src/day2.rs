use crate::AocDay;

pub struct Day2 {
    input: Option<Vec<Round>>,
}

impl Day2 {

    pub fn new() -> Day2 {
        Day2 { input: None }
    }

}

impl AocDay for Day2 {

    fn day(&self) -> i32 {
        2
    }

    fn init(&mut self, input: &Vec<String>) {
        let mut rounds_vec = Vec::new();
        for val in input.clone() {
            let round = parse_round(val);
            rounds_vec.push(round);
        }
        self.input = Some(rounds_vec);
    }

    fn part1(&self) -> String {
        let mut score: i32 = 0;
        let input = self.input.as_ref().unwrap();
        for round in input.clone() {
            let result = score_round(round);
            score += round.me as i32 + result as i32;
        }
        score.to_string()
    }

    fn part2(&self) -> String {
        let mut score: i32 = 0;
        let input = self.input.as_ref().unwrap();
        for round in input {
            let result = move_to_result(round.me);
            let me = get_move(round.op, result).unwrap();
            score += me as i32 + result as i32;
        }
        score.to_string()
    }

}

#[derive(Clone, Copy)]
enum Move {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

#[derive(Clone, Copy)]
enum WinLose {
    LOST = 0,
    TIED = 3,
    WON = 6,
}

#[derive(Clone, Copy)]
struct Round {
    op: Move,
    me: Move,
}

fn parse_round(str: String) -> Round {
    let split: Vec<&str> = str.split(" ").collect();
    let op = parse_move(split[0]).unwrap();
    let me = parse_move(split[1]).unwrap();
    return Round { op, me };
}

fn parse_move(move_str: &str) -> Result<Move, &str> {
    return match move_str {
        "A" | "X" => Ok(Move::ROCK),
        "B" | "Y" => Ok(Move::PAPER),
        "C" | "Z" => Ok(Move::SCISSORS),
        _ => Err(move_str)
    }
}

fn move_to_result(_move: Move) -> WinLose {
    return match _move {
        Move::ROCK => WinLose::LOST,
        Move::PAPER => WinLose::TIED,
        Move::SCISSORS => WinLose::WON,
    }
}

fn get_move(op: Move, result: WinLose) -> Option<Move> {
    return match (op, result) {
        (n, WinLose::TIED) => Some(n),
        (Move::ROCK, WinLose::LOST) => Some(Move::SCISSORS),
        (Move::ROCK, WinLose::WON) => Some(Move::PAPER),
        (Move::PAPER, WinLose::LOST) => Some(Move::ROCK),
        (Move::PAPER, WinLose::WON) => Some(Move::SCISSORS),
        (Move::SCISSORS, WinLose::LOST) => Some(Move::PAPER),
        (Move::SCISSORS, WinLose::WON) => Some(Move::ROCK),
    }
}

fn score_round(round: Round) -> WinLose {
    return match round {
        Round { op: Move::ROCK, me: Move::ROCK } => WinLose::TIED,
        Round { op: Move::ROCK, me: Move::PAPER } => WinLose::WON,
        Round { op: Move::ROCK, me: Move::SCISSORS } => WinLose::LOST,
        Round { op: Move::PAPER, me: Move::ROCK } => WinLose::LOST,
        Round { op: Move::PAPER, me: Move::PAPER } => WinLose::TIED,
        Round { op: Move::PAPER, me: Move::SCISSORS } => WinLose::WON,
        Round { op: Move::SCISSORS, me: Move::ROCK } => WinLose::WON,
        Round { op: Move::SCISSORS, me: Move::PAPER } => WinLose::LOST,
        Round { op: Move::SCISSORS, me: Move::SCISSORS } => WinLose::TIED,
    }
}
