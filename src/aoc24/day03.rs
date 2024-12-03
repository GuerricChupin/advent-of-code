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
        None
    }

    fn part2(self) -> Option<i64> {
        None
    }
}
