use std::collections::{HashMap, HashSet, VecDeque};

use crate::{position::Position, puzzle::Puzzle};

const GRID_SIZE: i64 = 71;

fn explore(obstacles: &HashSet<Position>) -> HashMap<Position, i64> {
    let initial_position = Position { x: 0, y: 0 };
    let mut visited = HashMap::from([(initial_position, 0)]);

    let mut positions_left_to_visit =
        VecDeque::from(initial_position.neighbors().map(|pos| (pos, 1)));

    while let Some((pos, score)) = positions_left_to_visit.pop_front() {
        if !obstacles.contains(&pos)
            && pos.x >= 0
            && pos.y >= 0
            && pos.x < GRID_SIZE
            && pos.y < GRID_SIZE
        {
            match visited.get(&pos).cloned() {
                Some(old_score) if old_score <= score => {
                    // Nothing to do
                }
                None | Some(_) => {
                    // This tile was never visited going that direction or
                    // its score needs to be updated
                    visited.insert(pos, score);
                    // Instruct to revisit the neighbours
                    let next = pos.neighbors().map(|next| (next, score + 1));
                    positions_left_to_visit.extend(next);
                }
            }
        }
    }

    visited
}

pub struct Day18 {
    coordinates: Vec<Position>,
}

impl Puzzle for Day18 {
    type Output = String;

    fn parse(input: &str) -> Option<Self> {
        Some(Day18 {
            coordinates: input
                .lines()
                .map(|word| {
                    let (x, y) = word.split_once(',')?;
                    let x = x.parse::<i64>().ok()?;
                    let y = y.parse::<i64>().ok()?;

                    Some(Position { x, y })
                })
                .collect::<Option<Vec<_>>>()?,
        })
    }

    fn part1(self) -> Option<Self::Output> {
        let obstacles = self
            .coordinates
            .into_iter()
            .take(1_024)
            .collect::<HashSet<_>>();

        let results = explore(&obstacles);

        let answer = results
            .get(&Position {
                x: GRID_SIZE - 1,
                y: GRID_SIZE - 1,
            })
            .cloned()?;

        Some(format!("{answer}"))
    }

    fn part2(self) -> Option<Self::Output> {
        let mut start = 0; 
        let mut end = self.coordinates.len() - 1; 

        // We do a dichotomic search over the list of obstacles, finding which
        // obstacle causes the path to get blocked
        while end != start {
            let mid_point = (end + start) / 2;
            // Not that this set of obstacles doesn't contain the mid_point-th
            // obstacle. This explain the choices to always remove the mid point
            // from start and end, otherwise we return the obstacle just after
            // the one we are looking for
            let obstacles = HashSet::from_iter(self.coordinates.iter().cloned().take(mid_point));
            let results = explore(&obstacles);
            let can_cross = results.get(&Position {
                    x: GRID_SIZE - 1,
                    y: GRID_SIZE - 1,
                }).is_some(); 

            if can_cross {
                start = mid_point + 1;
            } else {
                end = mid_point - 1;
            }
        }

        let obstacle = self.coordinates[start]; 

        Some(format!("{},{}", obstacle.x, obstacle.y))
    }
}
