use std::collections::{HashMap, HashSet, VecDeque};

use crate::{
    position::{Direction, Position},
    puzzle::Puzzle,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Deer {
    position: Position,
    direction: Direction,
}

impl Deer {
    fn next(self) -> [(Deer, i64); 3] {
        [
            (
                Deer {
                    position: self.position + self.direction.delta(),
                    direction: self.direction,
                },
                1,
            ),
            (
                Deer {
                    position: self.position,
                    direction: self.direction.next_clockwise(),
                },
                1_000,
            ),
            (
                Deer {
                    position: self.position,
                    direction: self.direction.next_anticlockwise(),
                },
                1_000,
            ),
        ]
    }

    fn previous(self) -> [(Deer, i64); 3] {
        [
            (
                Deer {
                    position: self.position - self.direction.delta(),
                    direction: self.direction,
                },
                -1,
            ),
            (
                Deer {
                    position: self.position,
                    direction: self.direction.next_clockwise(),
                },
                -1_000,
            ),
            (
                Deer {
                    position: self.position,
                    direction: self.direction.next_anticlockwise(),
                },
                -1_000,
            ),
        ]
    }
}

fn explore(walls: &HashSet<Position>, deer: Deer) -> HashMap<Deer, i64> {
    let mut visited = HashMap::from([(deer, 0)]);
    let mut positions_left_to_visit = VecDeque::from(deer.next());

    while let Some((deer, score)) = positions_left_to_visit.pop_front() {
        if !walls.contains(&deer.position) {
            match visited.get(&deer).cloned() {
                Some(old_score) if old_score <= score => {
                    // Nothing to do
                }
                None | Some(_) => {
                    // This tile was never visited going that direction or
                    // its score needs to be updated
                    visited.insert(deer, score);
                    // Instruct to revisit the neighbours
                    let next = deer.next().map(|(deer, cost)| (deer, score + cost));
                    positions_left_to_visit.extend(next);
                }
            }
        }
    }

    visited
}

pub struct Day16 {
    walls: HashSet<Position>,
    deer: Deer,
    end: Position,
}

impl Puzzle for Day16 {
    fn parse(input: &str) -> Option<Self> {
        let mut walls = HashSet::new();
        let mut deer = None;
        let mut end = None;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Position {
                    x: x as i64,
                    y: y as i64,
                };

                match c {
                    '#' => {
                        let _already_inserted = walls.insert(pos);
                    }
                    'E' => {
                        end = Some(pos);
                    }
                    'S' => {
                        deer = Some(Deer {
                            position: pos,
                            direction: Direction::Right,
                        });
                    }
                    _ => (),
                }
            }
        }

        let deer = deer?;
        let end = end?;

        Some(Day16 { walls, deer, end })
    }

    fn part1(self) -> Option<i64> {
        let walls = self.walls;

        let visited = explore(&walls, self.deer);

        let best_score = Direction::ALL
            .into_iter()
            .filter_map(|dir| {
                visited.get(&Deer {
                    position: self.end,
                    direction: dir,
                })
            })
            .min()
            .cloned()?;

        Some(best_score)
    }

    fn part2(self) -> Option<i64> {
        let visited = explore(&self.walls, self.deer);
        let (final_deer_direction, best_score) = Direction::ALL
            .into_iter()
            .filter_map(|dir| {
                visited
                    .get(&Deer {
                        position: self.end,
                        direction: dir,
                    })
                    .map(|score| (dir, *score))
            })
            .min_by(|(_, s1), (_, s2)| s1.cmp(s2))?;

        let deer = Deer {
            direction: final_deer_direction,
            position: self.end,
        };

        let mut best_path = HashSet::from([deer.position]);

        // The tiles on the best path are all the tiles that are reachable
        // (backward) from the end tile with a decreasing score
        let mut left_to_visit = VecDeque::from([(deer, best_score)]);

        while let Some((deer, score)) = left_to_visit.pop_front() {
            best_path.insert(deer.position);
            let previous_neighbours = deer.previous();
            let previous_neighbours_on_the_best_path =
                previous_neighbours
                    .into_iter()
                    .map(|(other, delta_score)| (other, score + delta_score))
                    .filter(|(other, expected_score)| {
                        visited.get(&other).cloned() == Some(*expected_score)
                    });
            left_to_visit.extend(previous_neighbours_on_the_best_path);
        }

        Some(best_path.len() as i64)
    }
}
