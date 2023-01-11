use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::io::repeat;
use std::ops;
use std::str::FromStr;

use crate::AocDay;
use crate::parser::Parser;

use lazy_static::lazy_static;

pub struct Day9 {
    moves: Option<Vec<Move>>,
}

impl Day9 {

    pub fn new() -> Day9 {
        Day9 { moves: None }
    }

}

impl AocDay for Day9 {

    fn day(&self) -> i32 {
        9
    }

    fn init(&mut self, input: &Vec<String>) {
        let moves = input
            .into_iter()
            .map(|line| line.parse::<Move>().unwrap())
            .collect::<Vec<_>>();

        self.moves = Some(moves);
    }

    fn part1(&self) -> String {
        let mut tails = HashSet::new();
        let mut head = Pos::origin();
        let mut tail = Pos::origin();
        tails.insert(tail.clone());
        for mv in self.moves.as_ref().unwrap().clone() {
            for _ in 0 .. mv.scale {
                head += mv.dir;
                tail.chase(head);
                tails.insert(tail.clone());
            }
        }
        tails.len().to_string()
    }

    fn part2(&self) -> String {
        const LEN: usize = 10;
        let mut tails = HashSet::new();
        let mut chain = [Pos::origin(); LEN];
        tails.insert(Pos::origin());
        for mv in self.moves.as_ref().unwrap().clone() {
            for _ in 0 .. mv.scale {
                chain[0] += mv.dir;
                for i in 1 .. LEN {
                    chain[i].chase(chain[i - 1]);
                }
                tails.insert(chain[LEN - 1].clone());
            }
        }
        tails.len().to_string()
    }

}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Pos {
    x: i16,
    y: i16,
}

impl Pos {

    fn origin() -> Pos {
        Pos { x: 0, y: 0 }
    }

    fn new(x: i16, y: i16) -> Pos {
        Pos { x, y }
    }

    fn transpose_dir(&mut self, dir: Direction) {
        let vec = dir.vector();
        self.x += vec.x;
        self.y += vec.y;
    }

    fn transpose(&mut self, mv: Move) {
        let dir = mv.dir.vector();
        let scale = mv.scale;
        self.x += dir.x * scale as i16;
        self.y += dir.y * scale as i16;
    }

    fn adjacent(&self, other: Pos) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn signum(&self) -> Pos {
        Pos { x: self.x.signum(), y: self.y.signum() }
    }

    fn chase(&mut self, other: Pos) {
        if !self.adjacent(other) {
            self.x += (other.x - self.x).signum();
            self.y += (other.y - self.y).signum();
        }
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl ops::Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::AddAssign<Pos> for Pos {
    fn add_assign(&mut self, rhs: Pos) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::AddAssign<Direction> for Pos {
    fn add_assign(&mut self, rhs: Direction) {
        let pos = rhs.vector();
        self.x += pos.x;
        self.y += pos.y;
    }
}

impl ops::Neg for Pos {
    type Output = Pos;

    fn neg(self) -> Self::Output {
        Pos { x: -self.x, y: -self.y }
    }
}

impl ops::Mul<i16> for Pos {
    type Output = Pos;

    fn mul(self, rhs: i16) -> Self::Output {
        Pos { x: self.x * rhs, y: self.y * rhs }
    }
}

impl ops::Sub<Pos> for Pos {
    type Output = Pos;

    fn sub(self, rhs: Pos) -> Self::Output {
        Pos { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl ops::Add<Move> for Move {
    type Output = Pos;

    fn add(self, rhs: Move) -> Self::Output {
        self.vector() + rhs.vector()
    }
}

impl ops::Add<Move> for Pos {
    type Output = Pos;

    fn add(self, rhs: Move) -> Self::Output {
        self + (rhs.dir.vector() * rhs.scale)
    }
}

impl ops::Mul<i16> for Move {
    type Output = Move;

    fn mul(self, rhs: i16) -> Self::Output {
        Move::new(self.dir, self.scale * rhs)
    }
}

#[derive(Copy, Clone, Debug)]
struct Move {
    dir: Direction,
    scale: i16,
}

impl Move {

    fn new(dir: Direction, scale: i16) -> Move {
        Move { dir, scale }
    }

    fn vector(&self) -> Pos {
        let Pos { x, y } = self.dir.vector();
        let scale = self.scale;
        Pos { x: x * scale, y: y * scale }
    }

}

lazy_static! {
    static ref MOVE_PARSER: Parser = Parser::new(r"([UDLR]) (\d+)");
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, scale) = MOVE_PARSER.parse2::<Direction, i16>(s).unwrap();
        Ok(Move { dir, scale })
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.dir, self.scale)
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Direction {

    fn vector(&self) -> Pos {
        match self {
            Direction::UP => Pos::new(0, 1),
            Direction::DOWN => Pos::new(0, -1),
            Direction::LEFT => Pos::new(-1, 0),
            Direction::RIGHT => Pos::new(1, 0),
        }
    }

}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "up" | "u" => Ok(Direction::UP),
            "down" | "d" => Ok(Direction::DOWN),
            "left" | "l" => Ok(Direction::LEFT),
            "right" | "r" => Ok(Direction::RIGHT),
            _ => Err(String::from(s)),
        }
    }
}

impl ops::Mul<i16> for Direction {
    type Output = Move;

    fn mul(self, rhs: i16) -> Self::Output {
        Move::new(self, rhs)
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Direction::UP => "UP",
            Direction::DOWN => "DOWN",
            Direction::LEFT => "LEFT",
            Direction::RIGHT => "RIGHT",
        };

        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {

    use crate::AocDay;
    use crate::day9::Day9;

    const EX1: &str = "R 4,U 4,L 3,D 1,R 4,D 1,L 5,R 2";
    const EX2: &str = "R 5,U 8,L 8,D 3,R 17,D 10,L 25,U 20";

    #[test]
    fn example1() {
        assert_eq!(init(EX1).part1(), "13");
    }

    #[test]
    fn example2() {
        assert_eq!(init(EX2).part2(), "36");
    }

    fn init(input: &str) -> Day9 {
        let mut day = Day9::new();
        day.init(&input.split(",").map(|s| s.to_string()).collect());
        day
    }

}
