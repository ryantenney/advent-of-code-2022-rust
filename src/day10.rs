use std::fmt::{Debug, Display, Formatter};
use std::ops::{Range, RangeInclusive};
use std::str::FromStr;

use crate::AocDay;

use crate::day10::Insn::{ADDX, NOOP};

#[derive(Default)]
pub struct Day10 {
    moves: Option<Vec<Insn>>,
    cycles: Option<Vec<Cycle>>,
}

impl Day10 {

    pub fn new() -> Day10 {
        Day10 { ..Default::default() }
    }

}

impl AocDay for Day10 {

    fn day(&self) -> i32 {
        10
    }

    fn init(&mut self, input: &Vec<String>) {
        let moves = input
            .into_iter()
            .map(|line| line.parse::<Insn>().expect("invalid insn"))
            .collect::<Vec<_>>();

        self.moves = Some(moves.clone());

        let mut x = 1;
        let mut cycle: usize = 0;
        let mut cycles: Vec<Cycle> = Vec::new();
        for insn in moves.into_iter() {
            cycle += 1;
            match insn {
                NOOP => {
                    cycles.push(Cycle::BEFORE(cycle, x));
                    cycles.push(Cycle::DURING(cycle, x));
                    cycles.push(Cycle::AFTER(cycle, x));
                }
                ADDX(v) => {
                    cycles.push(Cycle::BEFORE(cycle, x));
                    cycles.push(Cycle::DURING(cycle, x));
                    cycles.push(Cycle::AFTER(cycle, x));

                    cycle += 1;
                    cycles.push(Cycle::BEFORE(cycle, x));
                    cycles.push(Cycle::DURING(cycle, x));

                    x += v;
                    cycles.push(Cycle::AFTER(cycle, x));
                }
            }
        }

        self.cycles = Some(cycles);
    }

    fn part1(&self) -> String {
        self.cycles.as_ref().unwrap().iter().map(|cycle| {
            match cycle {
                Cycle::DURING(i, x) => {
                    if i % 40 == 20 {
                        *i as i32 * x
                    } else {
                        0
                    }
                },
                _ => 0,
            }
        })
            .sum::<i32>()
            .to_string()
    }

    fn part2(&self) -> String {
        let mut crt = [['.'; 40]; 6];

        let mut sprite: RangeInclusive<i32> = 0..=2;
        self.cycles.as_ref().unwrap().iter().for_each(|cycle| {
            if let Cycle::DURING(i, _) = cycle {
                let pos = get_pos(i);
                if sprite.contains(&(pos.1 as i32)) {
                    crt[pos.0][pos.1] = '#';
                }
            } else if let Cycle::AFTER(_, x) = cycle {
                sprite = x - 1 ..= x + 1;
            }
        });

        crt.map(|x| x.iter().collect::<String>()).join("\n")
    }

}

fn get_pos(i: &usize) -> (usize, usize) {
    let i = (i - 1) as usize;
    (i / 40, i % 40)
}

#[derive(Copy, Clone, Debug)]
enum Insn {
    ADDX(i32),
    NOOP
}

impl FromStr for Insn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..4] {
            "addx" => Ok(ADDX(s[5..].parse::<i32>().expect("addx requires an int argument"))),
            "noop" => Ok(NOOP),
            _ => Err(String::from(s)),
        }
    }
}

impl Display for Insn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ADDX(v) => write!(f, "addx {}", v),
            NOOP => write!(f, "noop"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Cycle {
    BEFORE(usize, i32),
    DURING(usize, i32),
    AFTER(usize, i32),
}

#[cfg(test)]
mod tests {

    use crate::AocDay;
    use crate::day10::Day10;

    const EX2: &str = "addx 15,addx -11,addx 6,addx -3,addx 5,addx -1,addx -8,addx 13,addx 4,noop,\
                       addx -1,addx 5,addx -1,addx 5,addx -1,addx 5,addx -1,addx 5,addx -1,addx -35,\
                       addx 1,addx 24,addx -19,addx 1,addx 16,addx -11,noop,noop,addx 21,addx -15,\
                       noop,noop,addx -3,addx 9,addx 1,addx -3,addx 8,addx 1,addx 5,noop,noop,noop,\
                       noop,noop,addx -36,noop,addx 1,addx 7,noop,noop,noop,addx 2,addx 6,noop,noop,\
                       noop,noop,noop,addx 1,noop,noop,addx 7,addx 1,noop,addx -13,addx 13,addx 7,\
                       noop,addx 1,addx -33,noop,noop,noop,addx 2,noop,noop,noop,addx 8,noop,\
                       addx -1,addx 2,addx 1,noop,addx 17,addx -9,addx 1,addx 1,addx -3,addx 11,\
                       noop,noop,addx 1,noop,addx 1,noop,noop,addx -13,addx -19,addx 1,addx 3,\
                       addx 26,addx -30,addx 12,addx -1,addx 3,addx 1,noop,noop,noop,addx -9,\
                       addx 18,addx 1,addx 2,noop,noop,addx 9,noop,noop,noop,addx -1,addx 2,\
                       addx -37,addx 1,addx 3,noop,addx 15,addx -21,addx 22,addx -6,addx 1,noop,\
                       addx 2,addx 1,noop,addx -10,noop,noop,addx 20,addx 1,addx 2,addx 2,addx -6,\
                       addx -11,noop,noop,noop";

    #[test]
    fn example() {
        let day10 = init(EX2);
        assert_eq!(day10.part1(), "13140");
        assert_eq!(day10.part2(), "\
            ##..##..##..##..##..##..##..##..##..##..\n\
            ###...###...###...###...###...###...###.\n\
            ####....####....####....####....####....\n\
            #####.....#####.....#####.....#####.....\n\
            ######......######......######......####\n\
            #######.......#######.......#######.....");
    }

    fn init(input: &str) -> Day10 {
        let mut day = Day10::new();
        day.init(&input.split(",").map(|s| s.to_string()).collect());
        day
    }

}
