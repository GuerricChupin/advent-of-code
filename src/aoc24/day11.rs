use std::collections::HashMap;

use crate::puzzle::Puzzle;

fn step_stone(stone: i64) -> (i64, Option<i64>) {
    if stone == 0 {
        (1, None)
    } else {
        let digits = 1 + stone.ilog10();

        if digits % 2 == 0 {
            let divisor = 10_i64.pow(digits / 2);
            let first_stone = stone / divisor;
            let second_stone = stone % divisor;
            (first_stone, Some(second_stone))
        } else {
            (stone * 2_024, None)
        }
    }
}

fn step_all_stones(stones: HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut new_stones = HashMap::with_capacity(stones.len());

    for (stone, stone_count) in stones.into_iter() {
        let (first_stone, optional_second_stone) = step_stone(stone);
        *new_stones.entry(first_stone).or_insert(0) += stone_count;

        if let Some(second_stone) = optional_second_stone {
            *new_stones.entry(second_stone).or_insert(0) += stone_count;
        }
    }

    new_stones
}

fn make_original_stone_map(stones: Vec<i64>) -> HashMap<i64, i64> {
    let mut stone_map = HashMap::with_capacity(stones.len());

    for stone in stones.into_iter() {
        *stone_map.entry(stone).or_insert(0) += 1;
    }

    stone_map
}

fn count_all_stones(stones: HashMap<i64, i64>) -> i64 {
    stones.into_values().sum()
}

fn count_stones_after_steps(stones: Vec<i64>, step_count: u32) -> i64 {
    let mut stones = make_original_stone_map(stones);

    for _step in 0..step_count {
        stones = step_all_stones(stones);
    }

    count_all_stones(stones)
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

    fn part1(self) -> Option<i64> {
        Some(count_stones_after_steps(self.stones, 25))
    }

    fn part2(self) -> Option<i64> {
        Some(count_stones_after_steps(self.stones, 75))
    }
}
