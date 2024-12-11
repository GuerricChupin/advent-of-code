use std::collections::{HashMap, HashSet};

use crate::{position::Position, puzzle::Puzzle};

pub struct Day10 {
    map: HashMap<Position, i64>,
}

fn reachable_summits(
    map: &HashMap<Position, i64>,
    current_position: Position,
    current_height: i64,
) -> HashSet<Position> {
    if current_height >= 9 {
        HashSet::from([current_position])
    } else {
        let neighbors = current_position.neighbors();
        let next_height_target = current_height + 1;

        neighbors
            .into_iter()
            .filter_map(|next_position| {
                map.get(&next_position)
                    .cloned()
                    .filter(|&next_height| next_height == next_height_target)
                    .map(|next_height| (next_position, next_height))
            })
            .map(|(next_position, next_height)| {
                reachable_summits(map, next_position, next_height).into_iter()
            })
            .flatten()
            .collect()
    }
}

fn count_all_paths(
    map: &HashMap<Position, i64>,
    current_position: Position,
    current_height: i64,
) -> i64 {
    if current_height >= 9 {
        1
    } else {
        let neighbors = current_position.neighbors();
        let next_height_target = current_height + 1;

        neighbors
            .into_iter()
            .filter_map(|next_position| {
                map.get(&next_position)
                    .cloned()
                    .filter(|&next_height| next_height == next_height_target)
                    .map(|next_height| (next_position, next_height))
            })
            .map(|(next_position, next_height)| count_all_paths(map, next_position, next_height))
            .sum()
    }
}

impl Puzzle for Day10 {
    fn parse(input: &str) -> Option<Self> {
        let mut map = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            for (x, height) in line.chars().enumerate() {
                let height = height.to_digit(10)?.into();

                let _ = map.insert(
                    Position {
                        x: x as i64,
                        y: y as i64,
                    },
                    height,
                );
            }
        }

        Some(Day10 { map })
    }

    fn part1(self) -> Option<i64> {
        let trailheads =
            self.map.iter().filter_map(
                |(&position, &height)| {
                    if height == 0 {
                        Some(position)
                    } else {
                        None
                    }
                },
            );

        Some(
            trailheads
                .map(|starting_point| reachable_summits(&self.map, starting_point, 0).len() as i64)
                .sum(),
        )
    }

    fn part2(self) -> Option<i64> {
        let trailheads =
            self.map.iter().filter_map(
                |(&position, &height)| {
                    if height == 0 {
                        Some(position)
                    } else {
                        None
                    }
                },
            );

        Some(
            trailheads
                .map(|starting_point| count_all_paths(&self.map, starting_point, 0))
                .sum(),
        )
    }
}
