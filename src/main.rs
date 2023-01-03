#![allow(dead_code)]
#![allow(unused_imports)]

mod day1;

use std::env;
use std::fs;
use std::str;

use crate::day1::Day1;

trait AocDay {
    fn day(&self) -> i32;
    fn init(&mut self, input: &Vec<String>);
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

fn read_lines(name: String) -> Vec<String> {
    let mut path = env::current_dir().unwrap();
    path.set_file_name(name);

    let contents = fs::read_to_string(path).unwrap();
    contents.lines().map(str::to_string).collect()
}

fn main() {
    let mut days: Vec<Box<dyn AocDay>> = Vec::new();
    days.push(Box::new(Day1::new()));
    //days.push(Box::new(Day2::new()));

    for day in days.iter_mut() {
        let day_num = day.day();
        println!("Day {}", day_num);
        let input = read_lines(format!("advent-of-code-2022-rust/src/day{}.txt", day_num));
        day.init(&input);
        println!("  Part 1: {}", day.part1());
        println!("  Part 2: {}", day.part2());
    }
}
