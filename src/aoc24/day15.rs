use std::collections::HashSet;

use crate::{position::Position, puzzle::Puzzle};

#[derive(Clone, Copy)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl Instruction {
    fn move_direction(self) -> Position { 
        match self {
            Instruction::Up => Position { x: 0, y: -1 },
            Instruction::Down => Position { x: 0, y: 1 },
            Instruction::Left => Position { x: -1, y: 0 },
            Instruction::Right => Position { x: 1, y: 0 },
        }
    }

    // fn has_neighbour(self, position: Position, obstacles: &HashSet<Position>) -> Option<Position> {
    //     let possible_neighbour = self.move_direction() + position; 

    //     if obstacles.contains(&possible_neighbour) {
    //         Some(possible_neighbour)
    //     } else {
    //         None
    //     }
    // }

    fn execute(
        self,
        walls: &HashSet<Position>,
        objects: &mut HashSet<Position>,
        robot: &mut Position,
    ) {
        let dv = self.move_direction();

        let mut line_of_object = Vec::new();

        let mut neighbour = *robot + dv;
        loop {
            if walls.contains(&neighbour) {
                return;
            }

            if objects.contains(&neighbour) {
                line_of_object.push(neighbour);
            } else {
                break;
            }

            neighbour = neighbour + dv;
        }

        *robot = *robot + dv;
        for &neighbour in line_of_object.iter() {
            let _ = objects.remove(&neighbour);
        }
        for neighbour in line_of_object.into_iter() {
            let _ = objects.insert(neighbour + dv);
        }
    }
}

fn gps_coord(pos: Position) -> i64 {
    100 * pos.y + pos.x
}

pub struct Day15 {
    walls: HashSet<Position>,
    objects: HashSet<Position>,

    initial_robot_position: Position,

    instruction_tape: Vec<Instruction>,
}

impl Puzzle for Day15 {
    fn parse(input: &str) -> Option<Self> {
        let mut split_input = input.split("\n\n");
        let grid = split_input.next()?;
        let instructions = split_input.next()?;

        let mut walls = HashSet::new();
        let mut objects = HashSet::new();
        let mut robot = None;

        for (y, line) in grid.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Position {
                    x: x as i64,
                    y: y as i64,
                };

                match c {
                    '@' => {
                        robot = Some(pos);
                    }
                    '#' => {
                        walls.insert(pos);
                    }
                    'O' => {
                        objects.insert(pos);
                    }
                    _ => (),
                }
            }
        }

        let initial_robot_position = robot?;

        let instruction_tape = instructions
            .chars()
            .filter_map(|c| match c {
                '^' => Some(Instruction::Up),
                'v' => Some(Instruction::Down),
                '<' => Some(Instruction::Left),
                '>' => Some(Instruction::Right),
                _ => None,
            })
            .collect::<Vec<_>>();

        Some(Day15 {
            walls,
            objects,
            initial_robot_position,
            instruction_tape,
        })
    }

    fn part1(self) -> Option<i64> {
        let mut robot = self.initial_robot_position;
        let mut objects = self.objects;
        let walls = self.walls;

        for instruction in self.instruction_tape.into_iter() {
            instruction.execute(&walls, &mut objects, &mut robot);
        }

        Some(objects.into_iter().map(|pos| gps_coord(pos)).sum())
    }

    fn part2(self) -> Option<i64> {
        // let mut robot = Position {
        //     x: self.initial_robot_position.x * 2,
        //     ..self.initial_robot_position
        // };

        // let walls = self
        //     .walls
        //     .into_iter()
        //     .map(|pos| {
        //         [
        //             Position {
        //                 x: pos.x * 2,
        //                 y: pos.y,
        //             },
        //             Position {
        //                 x: pos.x * 2 + 1,
        //                 y: pos.y,
        //             },
        //         ]
        //     })
        //     .flatten()
        //     .collect::<HashSet<_>>();

        // let mut objects = self
        //     .objects
        //     .into_iter()
        //     .map(|pos| Position {
        //         x: pos.x * 2,
        //         y: pos.y,
        //     })
        //     .collect::<HashSet<_>>();

        None
    }
}
