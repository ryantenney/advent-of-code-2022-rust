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
        for i in 0 .. msg.len() - len {
            let set: HashSet<&char> = HashSet::from_iter(msg[i .. i + len].iter());
            if set.len() == len {
                return (i + len).to_string();
            }
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
