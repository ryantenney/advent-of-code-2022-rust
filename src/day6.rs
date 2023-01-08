use std::collections::HashSet;

use crate::AocDay;

pub struct Day6 {
    message: Option<String>,
}

impl Day6 {

    pub fn new() -> Day6 {
        Day6 { message: None }
    }

    fn find(&self, len: usize) -> String {
        let msg: Vec<char> = self.message.as_ref().unwrap().chars().collect();
        let mut tmp: Vec<char> = Vec::with_capacity(len);
        'next: for i in len .. msg.len() {
            msg[i - len .. i].clone_into(&mut tmp);
            tmp.sort();
            for j in 1 .. len {
                if tmp[j - 1] == tmp[j] {
                    continue 'next;
                }
            }
            return i.to_string();
        }
        "NOTFOUND".to_string()
    }

}

impl AocDay for Day6 {

    fn day(&self) -> i32 {
        6
    }

    fn init(&mut self, input: &Vec<String>) {
        self.message = input.get(0).map(String::clone);
    }

    fn part1(&self) -> String {
        self.find(4)
    }

    fn part2(&self) -> String {
        self.find(14)
    }

}
