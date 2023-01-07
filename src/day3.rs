use std::collections::HashSet;
use std::fmt::Display;
use std::ptr::null;

use crate::AocDay;

pub struct Day3 {
    input: Option<Vec<String>>,
}

impl Day3 {

    pub fn new() -> Day3 {
        Day3 { input: None }
    }

}

impl AocDay for Day3 {

    fn day(&self) -> i32 {
        3
    }

    fn init(&mut self, input: &Vec<String>) {
        let mut strings_vec = Vec::new();
        for val in input.clone() {
            strings_vec.push(val);
        }
        self.input = Some(strings_vec);
    }

    fn part1(&self) -> String {
        let mut sum: u32 = 0;
        for val in self.input.as_ref().unwrap() {
            let mid = val.len() / 2;
            let left = chars(&val[..mid]);
            let right = chars(&val[mid..]);

            let mut top = 0;
            for chr in right.into_iter() {
                if left.contains(&chr) {
                    let priority = priority(chr);
                    if priority > top {
                        top = priority;
                    }
                }
            }

            sum += top as u32;
        }

        sum.to_string()
    }

    /*
    fn part2(&self) -> String {
        let mut sum: u32 = 0;
        let mut iter = self.input.as_ref().unwrap().iter();
        while let Some(next) = iter.next() {
            let first = chars(next);
            let second = chars(iter.next().unwrap());
            let third = chars(iter.next().unwrap());

            let next = intersect(first, second);
            let last = intersect(next, third);

            if last.len() == 1 {
                let x = last.into_iter().next().unwrap();
                sum += priority(x) as u32;
            }
        }

        sum.to_string()
    }
     */

    fn part2(&self) -> String {
        let mut sum: u32 = 0;
        let mut round: u8 = 0;
        let mut set: HashSet<char> = HashSet::new();
        let mut iter = self.input.as_ref().unwrap().iter();
        while let Some(next) = iter.next() {
            round += 1;
            let mut char_set = chars(next);
            if round > 1 {
                char_set = intersect(set, char_set);
                if round == 3 {
                    if char_set.len() == 1 {
                        let x = char_set.iter().next().unwrap();
                        sum += priority(*x) as u32;
                    }
                    round = 0;
                }
            }
            set = char_set.clone();
        }

        sum.to_string()
    }

}

fn chars(str: &str) -> HashSet<char> {
    HashSet::from_iter(str.chars())
}

fn intersect(first: HashSet<char>, second: HashSet<char>) -> HashSet<char> {
    let mut output = HashSet::new();
    for chr in first.intersection(&second) {
        output.insert(*chr);
    }
    return output;
}

fn priority(chr: char) -> u8 {
    match chr {
        n @ 'a' ..= 'z' => n as u8 - 'a' as u8 + 1,
        n @ 'A' ..= 'Z' => n as u8 - 'A' as u8 + 27,
        _ => 0,
    }
}
