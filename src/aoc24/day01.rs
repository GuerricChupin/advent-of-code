use std::collections::HashMap;

use crate::puzzle::Puzzle;

pub struct Day01 {
    left: Vec<i64>,
    right: Vec<i64>,
}

impl Puzzle for Day01 {
    fn parse(input: &str) -> Option<Self> {
        let mut left = Vec::new();
        let mut right = Vec::new();

        for line in input.lines() {
            let mut whitespace = line.split_whitespace();
            left.push(whitespace.next()?.parse::<i64>().ok()?);
            right.push(whitespace.next()?.parse::<i64>().ok()?);
        }

        Some(Day01 { left, right })
    }

    fn part1(mut self) -> Option<i64> {
        self.left.sort();
        self.right.sort();

        Some(
            self.left
                .into_iter()
                .zip(self.right.into_iter())
                .map(|(x, y)| (x - y).abs())
                .sum(),
        )
    }

    fn part2(self) -> Option<i64> {
        let mut right_frequency = HashMap::new();
        for number in self.right.into_iter() {
            let entry = right_frequency.entry(number);
            *entry.or_insert(0) += 1;
        }

        Some(
            self.left
                .into_iter()
                .map(|number| number * right_frequency.get(&number).cloned().unwrap_or(0))
                .sum(),
        )
    }
}
