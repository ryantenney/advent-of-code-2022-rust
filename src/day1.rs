use crate::AocDay;

pub struct Day1 {
    input: Vec<i32>,
}

impl Day1 {

    pub fn new() -> Day1 {
        Day1 { input: vec![] }
    }

}

impl AocDay for Day1 {

    fn day(&self) -> i32 {
        1
    }

    fn init(&mut self, input: &Vec<String>) {
        let mut cals_vec: Vec<i32> = Vec::new();
        let mut cals_current: i32 = 0;
        for val in input.clone() {
            if val == "" {
                cals_vec.push(cals_current);
                cals_current = 0;
            } else {
                cals_current += val.parse::<i32>().unwrap();
            }
        }
        cals_vec.push(cals_current);
        cals_vec.sort_by(|a, b| b.cmp(a));
        self.input = cals_vec;
    }

    fn part1(&self) -> String {
        self.input[0].to_string()
    }

    fn part2(&self) -> String {
        self.input[..3].iter().sum::<i32>().to_string()
    }

}
