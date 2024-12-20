use std::collections::{HashMap, HashSet};

use crate::{
    position::{Direction, Position},
    puzzle::Puzzle,
};

enum Object {
    Wall,
    Crate,
    BigCrateLeft,
    BigCrateRight,
}

fn get_block_of_neighbours(
    direction: Direction,
    current_position: Position,
    objects: &HashMap<Position, Object>,
    block: &mut HashMap<Position, Object>,
) -> bool {
    let dv = direction.delta();

    let mut to_visit = Vec::from([current_position]);
    let mut visited = HashSet::new();

    while let Some(pos) = to_visit.pop() {
        if !visited.contains(&pos) {
            visited.insert(pos);
            
            let neighbour = pos + dv;

            match objects.get(&neighbour) {
                Some(Object::Wall) => return false,
                Some(Object::Crate) => {
                    block.insert(neighbour, Object::Crate);
                    to_visit.push(neighbour);
                }
                Some(Object::BigCrateLeft) => {
                    block.insert(neighbour, Object::BigCrateLeft);
                    let righty_neighbour = neighbour + Position { x: 1, y: 0 };
                    block.insert(righty_neighbour, Object::BigCrateRight);

                    to_visit.push(neighbour);
                    to_visit.push(righty_neighbour);
                }
                Some(Object::BigCrateRight) => {
                    block.insert(neighbour, Object::BigCrateRight);
                    let lefty_neighbour = neighbour + Position { x: -1, y: 0 };
                    block.insert(lefty_neighbour, Object::BigCrateLeft);

                    to_visit.push(neighbour);
                    to_visit.push(lefty_neighbour);
                }
                None => (),
            }
        }
    }

    true
}

fn execute(direction: Direction, objects: &mut HashMap<Position, Object>, robot: &mut Position) {
    let mut block = HashMap::new();
    let can_move = get_block_of_neighbours(direction, *robot, objects, &mut block);

    if can_move {
        let dv = direction.delta();

        *robot = *robot + dv;
        for (neighbour, _) in block.iter() {
            let _ = objects.remove(&neighbour);
        }
        for (neighbour, kind) in block.into_iter() {
            let _ = objects.insert(neighbour + dv, kind);
        }
    }
}

fn gps_coord(pos: Position) -> i64 {
    100 * pos.y + pos.x
}

fn compute_score(objects: HashMap<Position, Object>) -> i64 {
    objects
        .into_iter()
        .map(|(pos, object)| match object {
            Object::Wall | Object::BigCrateRight => 0,
            Object::Crate | Object::BigCrateLeft => gps_coord(pos),
        })
        .sum()
}

pub struct Day15 {
    objects: HashMap<Position, Object>,

    initial_robot_position: Position,

    instruction_tape: Vec<Direction>,
}

impl Day15 {
    fn expand(self) -> Self {
        let initial_robot_position = Position {
            x: self.initial_robot_position.x * 2,
            y: self.initial_robot_position.y,
        };

        let objects = self
            .objects
            .into_iter()
            .flat_map(|(pos, object)| {
                let p1 = Position {
                    x: pos.x * 2,
                    y: pos.y,
                };
                let p2 = Position {
                    x: pos.x * 2 + 1,
                    y: pos.y,
                };

                match object {
                    Object::Wall => [(p1, Object::Wall), (p2, Object::Wall)],
                    Object::Crate => [(p1, Object::BigCrateLeft), (p2, Object::BigCrateRight)],
                    Object::BigCrateLeft | Object::BigCrateRight => {
                        panic!("Cannot expand maps with big crates")
                    }
                }
            })
            .collect::<HashMap<_, _>>();

        Day15 {
            objects,
            initial_robot_position,
            instruction_tape: self.instruction_tape,
        }
    }
}

impl Puzzle for Day15 {
    type Output = i64;

    fn parse(input: &str) -> Option<Self> {
        let mut split_input = input.split("\n\n");
        let grid = split_input.next()?;
        let instructions = split_input.next()?;

        let mut objects = HashMap::new();
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
                        objects.insert(pos, Object::Wall);
                    }
                    'O' => {
                        objects.insert(pos, Object::Crate);
                    }
                    _ => (),
                }
            }
        }

        let initial_robot_position = robot?;

        let instruction_tape = instructions
            .chars()
            .filter_map(|c| match c {
                '^' => Some(Direction::Up),
                'v' => Some(Direction::Down),
                '<' => Some(Direction::Left),
                '>' => Some(Direction::Right),
                _ => None,
            })
            .collect::<Vec<_>>();

        Some(Day15 {
            objects,
            initial_robot_position,
            instruction_tape,
        })
    }

    fn part1(self) -> Option<i64> {
        let mut robot = self.initial_robot_position;
        let mut objects = self.objects;

        for instruction in self.instruction_tape.into_iter() {
            execute(instruction, &mut objects, &mut robot);
        }

        Some(compute_score(objects))
    }

    fn part2(mut self) -> Option<i64> {
        self = self.expand();

        let mut robot = self.initial_robot_position;
        let mut objects = self.objects;

        for instruction in self.instruction_tape.into_iter() {
            execute(instruction, &mut objects, &mut robot);
        }

        Some(compute_score(objects))
    }
}
