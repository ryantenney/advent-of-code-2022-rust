#![allow(dead_code)]
#![allow(unused_imports)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day9;
mod day10;
mod parser;

use std::env;
use std::fs;
use std::str;
use std::time::{Duration, Instant};

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
    let run_day: Option<i32> = None;

    let mut days: Vec<Box<dyn AocDay>> = Vec::new();
    days.push(Box::new(day1::Day1::new()));
    days.push(Box::new(day2::Day2::new()));
    days.push(Box::new(day3::Day3::new()));
    days.push(Box::new(day4::Day4::new()));
    days.push(Box::new(day5::Day5::new()));
    days.push(Box::new(day6::Day6::new()));
    days.push(Box::new(day9::Day9::new()));
    days.push(Box::new(day10::Day10::new()));

    let mut total_duration = Duration::default();
    for day in days.iter_mut() {
        if let Some(day_num) = run_day {
            if day_num != day.day() {
                continue;
            }
        }

        println!("Day {}", day.day());
        let input = read_lines(format!("advent-of-code-2022-rust/src/day{}.txt", day.day()));
        let initstart = Instant::now();
        day.init(&input);
        let initduration = initstart.elapsed();
        println!("  Init  : ({:?})", initduration);

        let part1start = Instant::now();
        let part1 = day.part1();
        let part1duration = part1start.elapsed();
        if part1.contains('\n') {
            println!("  Part 1: ({:?})\n    {}", part1duration, part1.replace('\n', "\n    "));
        } else {
            println!("  Part 1: {} ({:?})", part1, part1duration);
        }

        let part2start = Instant::now();
        let part2 = day.part2();
        let part2duration = part2start.elapsed();
        if part2.contains('\n') {
            println!("  Part 2: ({:?})\n    {}", part2duration, part2.replace('\n', "\n    "));
        } else {
            println!("  Part 2: {} ({:?})", part2, part2duration);
        }

        total_duration += initduration + part1duration + part2duration;
    }

    println!("Total: {:?}", total_duration);
}
