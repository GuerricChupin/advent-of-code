use regex::Regex;

use crate::puzzle::Puzzle;

pub struct Day03 {
    input: String,
}

impl Puzzle for Day03 {
    fn parse(input: &str) -> Option<Self> {
        Some(Day03 {
            input: String::from(input),
        })
    }

    fn part1(self) -> Option<i64> {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        Some(
            re.captures_iter(&self.input)
                .map(|capture| {
                    capture[1].parse::<i64>().unwrap() * capture[2].parse::<i64>().unwrap()
                })
                .sum(),
        )
    }

    fn part2(self) -> Option<i64> {
        let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

        let mut enabled = true;
        let mut total = 0;

        for capture in re.captures_iter(&self.input) {
            match &capture[0] {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    if enabled {
                        total +=
                            capture[1].parse::<i64>().unwrap() * capture[2].parse::<i64>().unwrap();
                    }
                }
            }
        }

        Some(total)
    }
}
