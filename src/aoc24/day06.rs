use std::collections::{HashMap, HashSet};

use crate::puzzle::Puzzle;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Clone, Copy)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    fn inside(self, south_east_corner: Position) -> bool {
        let position = self.position;

        0 <= position.x
            && position.x <= south_east_corner.x
            && 0 <= position.y
            && position.y <= south_east_corner.y
    }

    fn turn(self) -> Self {
        Guard {
            direction: self.direction.turn(),
            ..self
        }
    }

    fn next_unobstructed_position(self) -> Position {
        let current_position = self.position;

        match self.direction {
            Direction::Up => Position {
                y: current_position.y - 1,
                ..current_position
            },
            Direction::Down => Position {
                y: current_position.y + 1,
                ..current_position
            },
            Direction::Left => Position {
                x: current_position.x - 1,
                ..current_position
            },
            Direction::Right => Position {
                x: current_position.x + 1,
                ..current_position
            },
        }
    }

    fn step(self, obstacles: &HashSet<Position>) -> Self {
        let next_unobstructed_position = self.next_unobstructed_position();

        if obstacles.contains(&next_unobstructed_position) {
            self.turn().step(obstacles)
        } else {
            Guard {
                position: next_unobstructed_position,
                ..self
            }
        }
    }
}

pub struct Day06 {
    initial_guard: Guard,
    obstacles: HashSet<Position>,
    grid_south_east_corner: Position,
}

impl Puzzle for Day06 {
    fn parse(input: &str) -> Option<Self> {
        let mut guard = None;
        let mut obstacles = HashSet::new();
        let mut grid_south_east_corner = Position { x: 0, y: 0 };

        for (y, line) in input.lines().enumerate() {
            grid_south_east_corner.y = grid_south_east_corner.y.max(y as i64);

            for (x, c) in line.chars().enumerate() {
                grid_south_east_corner.x = grid_south_east_corner.x.max(x as i64);

                let position = Position {
                    x: x as i64,
                    y: y as i64,
                };

                match c {
                    '#' => {
                        obstacles.insert(position);
                    }
                    '^' => match guard {
                        None => {
                            guard = Some(Guard {
                                position: position,
                                direction: Direction::Up,
                            })
                        }
                        Some(_) => return None,
                    },
                    '.' => (),
                    _ => return None,
                };
            }
        }

        Some(Day06 {
            initial_guard: guard?,
            obstacles,
            grid_south_east_corner,
        })
    }

    fn part1(self) -> Option<i64> {
        let mut guard = self.initial_guard;
        let mut visited_positions = HashSet::new();

        while guard.inside(self.grid_south_east_corner) {
            let _already_visited = visited_positions.insert(guard.position);

            guard = guard.step(&self.obstacles);
        }

        Some(visited_positions.len() as i64)
    }

    fn part2(self) -> Option<i64> {
        let mut guard = self.initial_guard;
        let mut visited_positions = HashMap::new();
        let mut added_obstacles = 0;

        while guard.inside(self.grid_south_east_corner) {
            if let Some(previously_visited_direction) = visited_positions.get(&guard.position) {
                // If the guard was previously on this direction, we have a
                // chance to place an obstacle, if that obstacle would make the
                // guard turn in the direction they were facing when they first
                // visited that tile
                if &guard.direction.turn() == previously_visited_direction {
                    let next_position = guard.next_unobstructed_position();
                    if !visited_positions.contains_key(&next_position) {
                        // If the next position was not previously visited
                        // by the guard, we can add an obstacle here to send
                        // them in a loop. We can't add an obstacle if the
                        // guard visited that place previously, because that
                        // would mean that they would have encountered the
                        // obstacle and so would not have followed the same
                        // path

                        added_obstacles += 1;
                    }
                }
            }

            let _ = visited_positions.insert(guard.position, guard.direction);
            guard = guard.step(&self.obstacles);
        }

        Some(added_obstacles)
    }
}
