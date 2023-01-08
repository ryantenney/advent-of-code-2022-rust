use std::collections::HashSet;
use std::fmt::Display;
use std::ptr::null;

use crate::AocDay;

pub struct Day4 {
    input: Option<Vec<(Range, Range)>>,
}

impl Day4 {

    pub fn new() -> Day4 {
        Day4 { input: None }
    }

}

impl AocDay for Day4 {

    fn day(&self) -> i32 {
        4
    }

    fn init(&mut self, input: &Vec<String>) {
        let mut ranges_vec = Vec::new();
        for val in input.clone() {
            let parts: Vec<&str> = val.split(",").collect();
            let range_left = Range::parse(parts[0]);
            let range_right = Range::parse(parts[1]);
            ranges_vec.push((range_left, range_right));
        }
        self.input = Some(ranges_vec);
    }

    fn part1(&self) -> String {
        let mut count: u16 = 0;
        for val in self.input.as_ref().unwrap() {
            if val.0.contains(&val.1) || val.1.contains(&val.0) {
                count += 1;
            }
        }
        count.to_string()
    }

    fn part2(&self) -> String {
        let mut count: u16 = 0;
        for val in self.input.as_ref().unwrap() {
            if val.0.overlaps(&val.1) || val.1.overlaps(&val.0) {
                count += 1;
            }
        }
        count.to_string()
    }

}

#[derive(Clone,Copy)]
struct Range {
    start: u8,
    end: u8,
}

impl Range {

    fn new(start: u8, end: u8) -> Range {
        Range { start, end }
    }

    fn parse(input: &str) -> Range {
        let parts: Vec<&str> = input.split("-").collect();
        Range { start: parts[0].parse::<u8>().unwrap(), end: parts[1].parse::<u8>().unwrap() }
    }

    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        (self.start >= other.start && self.start <= other.end)
                || (self.end >= other.start && self.end <= other.end)
    }

}
