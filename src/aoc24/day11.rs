use crate::puzzle::Puzzle;

fn step_stone(stone: i64) -> (i64, Option<i64>) {
    if stone == 0 {
        (1, None)
    } else {
        let digits = 1 + stone.ilog10();

        if digits % 2 == 0 {
            let first_stone = stone / (10_i64.pow(digits / 2));
            let second_stone = stone % 10_i64.pow(digits / 2);
            (first_stone, Some(second_stone))
        } else {
            (stone * 2_024, None)
        }
    }
}

fn step_all_stones(stones: &[i64]) -> Vec<i64> {
    let mut new_stones = Vec::with_capacity(stones.len());

    for &stone in stones { 
        let (first_stone, optional_second_stone) = step_stone(stone);
        new_stones.push(first_stone);

        if let Some(second_stone) = optional_second_stone { 
            new_stones.push(second_stone);
        }
    }

    new_stones
}

pub struct Day11 {
    stones: Vec<i64>,
}

impl Puzzle for Day11 {
    fn parse(input: &str) -> Option<Self> {
        Some(Day11 {
            stones: input
                .split_whitespace()
                .map(|stone| stone.parse::<i64>().ok())
                .collect::<Option<Vec<_>>>()?,
        })
    }

    fn part1(mut self) -> Option<i64> {
        for _step in 0..25 {
            self.stones = step_all_stones(&self.stones);
        }

        Some(self.stones.len() as i64)
    }

    fn part2(self) -> Option<i64> {
        None
    }
}
