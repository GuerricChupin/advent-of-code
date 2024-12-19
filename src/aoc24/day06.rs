use std::collections::{HashMap, HashSet};

use crate::{position::{Direction, Position}, puzzle::Puzzle};


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
            direction: self.direction.next_clockwise(),
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

enum Status {
    Leaving,
    InALoop,
}

fn run_guard(
    mut guard: Guard,
    grid_south_east_corner: Position,
    obstacles: &HashSet<Position>,
) -> (Status, HashMap<Position, Direction>) {
    let mut visited_positions = HashMap::new();

    loop {
        if !guard.inside(grid_south_east_corner) {
            break (Status::Leaving, visited_positions);
        }

        let already_visited = visited_positions.insert(guard.position, guard.direction);

        if already_visited == Some(guard.direction) {
            break (Status::InALoop, visited_positions);
        }

        guard = guard.step(obstacles);
    }
}

pub struct Day06 {
    initial_guard: Guard,
    obstacles: HashSet<Position>,
    grid_south_east_corner: Position,
}

impl Puzzle for Day06 {
    type Output = i64;
    
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
                                position,
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
        let (_final_status, visited_positions) = run_guard(
            self.initial_guard,
            self.grid_south_east_corner,
            &self.obstacles,
        );

        Some(visited_positions.len() as i64)
    }

    fn part2(self) -> Option<i64> {
        let mut added_obstacles = 0;

        for x in 0..self.grid_south_east_corner.x {
            for y in 0..self.grid_south_east_corner.y {
                let new_obstacle_position = Position { x, y };

                if !self.obstacles.contains(&new_obstacle_position)
                    && new_obstacle_position != self.initial_guard.position
                {
                    let mut new_obstacles = self.obstacles.clone();
                    new_obstacles.insert(new_obstacle_position);
                    let (status, _) = run_guard(
                        self.initial_guard,
                        self.grid_south_east_corner,
                        &new_obstacles,
                    );

                    match status {
                        Status::InALoop => {
                            added_obstacles += 1;
                        }
                        Status::Leaving => (),
                    };
                }
            }
        }

        Some(added_obstacles)
    }
}
