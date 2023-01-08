use std::collections::{HashSet, VecDeque};
use std::fmt::Display;
use std::ptr::null;

use regex::Regex;

use crate::AocDay;

pub struct Day5 {
    stacks: Option<Vec<VecDeque<char>>>,
    moves: Option<Vec<Move>>,
}

impl Day5 {

    pub fn new() -> Day5 {
        Day5 { stacks: None, moves: None }
    }

    fn stacks(&self) -> Vec<VecDeque<char>> {
        self.stacks.as_ref().unwrap().iter()
            .map(|x| x.clone())
            .collect()

    }

}

impl AocDay for Day5 {

    fn day(&self) -> i32 {
        5
    }

    fn init(&mut self, input: &Vec<String>) {
        let char_matrix = input.iter()
            .take_while(|x| !x.is_empty())
            .map(|x| {
                x.chars().collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();

        let mut stacks: Vec<VecDeque<char>> = Vec::new();
        for val in transpose(char_matrix) {
            if *val.last().unwrap() != ' ' {
                let stack = val.into_iter()
                    .rev()
                    .skip(1)
                    .filter(|x| *x != ' ')
                    .collect::<VecDeque<char>>();
                stacks.push(stack);
            }
        }

        let mut moves: Vec<Move> = Vec::new();
        for val in input {
            if let Some(_move) = Move::parse(val) {
                moves.push(_move);
            }
        }

        self.stacks = Some(stacks);
        self.moves = Some(moves);
    }

    fn part1(&self) -> String {
        let mut vec = self.stacks();
        for _move in self.moves.as_ref().unwrap() {
            for _ in 0 .. _move.qty {
                let chr = vec[_move.from].pop_back().unwrap();
                vec[_move.to].push_back(chr);
            }
        }
        vec.iter().map(|x| x.back().unwrap()).collect()
    }

    fn part2(&self) -> String {
        let mut vec = self.stacks();
        let mut tmp = VecDeque::new();
        for _move in self.moves.as_ref().unwrap() {
            for _ in 0 .. _move.qty {
                let chr = vec[_move.from].pop_back().unwrap();
                tmp.push_front(chr);
            }
            vec[_move.to].append(&mut tmp);
        }
        vec.iter().map(|x| x.back().unwrap()).collect()
    }

}

fn transpose<T>(input: Vec<Vec<T>>) -> Vec<Vec<T>> where T: Clone {
    let len = input.iter().map(Vec::len).max().unwrap();
    let mut output: Vec<Vec<T>> = Vec::with_capacity(len);
    for i in 0..len {
        let mut i_vec = Vec::with_capacity(input.len());
        for j in 0..input.len() {
            i_vec.push(input[j][i].clone());
        }
        output.push(i_vec);
    }
    return output;
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d)$").unwrap();
}

#[derive(Clone,Copy,Debug)]
struct Move {
    qty: usize,
    from: usize,
    to: usize,
}

impl Move {

    fn new(qty: usize, from: usize, to: usize) -> Move {
        Move { qty, from, to }
    }

    fn parse(input: &str) -> Option<Move> {
        RE.captures(input).map(|captures| {
            let args: Vec<usize> = captures.iter()
                .skip(1)
                .map(|x| {
                    x.unwrap().as_str().parse::<usize>().unwrap()
                })
                .collect();

            Move::new(args[0], args[1] - 1, args[2] - 1)
        })
    }

}
