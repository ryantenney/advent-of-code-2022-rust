#![allow(dead_code)]
#![allow(unused_imports)]

mod day1;
mod day2;
mod day3;

use std::env;
use std::fs;
use std::str;
use std::time::Instant;

use crate::day1::Day1;
use crate::day2::Day2;
use crate::day3::Day3;

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
    let run_day = -1;

    let mut days: Vec<Box<dyn AocDay>> = Vec::new();
    days.push(Box::new(Day1::new()));
    days.push(Box::new(Day2::new()));
    days.push(Box::new(Day3::new()));

    for day in days.iter_mut() {
        let day_num = day.day();
        if run_day == -1 || run_day == day_num {
            println!("Day {}", day_num);
            let input = read_lines(format!("advent-of-code-2022-rust/src/day{}.txt", day_num));
            day.init(&input);

            let part1start = Instant::now();
            let part1 = day.part1();
            let part1duration = part1start.elapsed();
            println!("  Part 1: {} ({:?})", part1, part1duration);

            let part2start = Instant::now();
            let part2 = day.part2();
            let part2duration = part2start.elapsed();
            println!("  Part 2: {} ({:?})", part2, part2duration);
        }
    }
}
