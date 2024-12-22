use std::collections::HashMap;

use crate::{
    position::{Direction, Position},
    puzzle::Puzzle,
};

const NUMERIC_KEYPAD: [Position; 11] = [
    Position { x: -1, y: 0 },  // 0
    Position { x: -2, y: -1 }, // 1
    Position { x: -1, y: -1 }, // 2
    Position { x: 0, y: -1 },  // 3
    Position { x: -2, y: -2 }, // 4
    Position { x: -1, y: -2 }, // 5
    Position { x: 0, y: -2 },  // 6
    Position { x: -2, y: -3 }, // 7
    Position { x: -1, y: -3 }, // 8
    Position { x: 0, y: -3 },  // 9
    Position { x: 0, y: 0 },   // A
];

const ARROW_KEYPAD: [Position; 5] = [
    Position { x: 0, y: 0 },  // A
    Position { x: -1, y: 0 }, // ^
    Position { x: 0, y: 1 },  // >
    Position { x: -1, y: 1 }, // v
    Position { x: -2, y: 1 }, // <
];

#[derive(Clone)]
struct Board {
    board: &'static [Position],
    cache: HashMap<(Position, Position), Vec<Direction>>,
}

impl Board {
    fn inside(&self, position: Position) -> bool {
        self.board.contains(&position)
    }

    fn move_to(&mut self, start: Position, end: Position) -> Vec<Direction> {
        match self.cache.get(&(start, end)) {
            None => {
                let Position { x: dx, y: dy } = end - start;

                // Yes it looks in reverse because in my lib, y goes down
                let vertical_direction = if dy < 0 {
                    Direction::Up
                } else {
                    Direction::Down
                };
                let horizontal_direction = if dx < 0 {
                    Direction::Left
                } else {
                    Direction::Right
                };

                // We forbid any zigzaging, so we'll either go in the vertical direction
                // all the way first, and then in the horizontal direction or vice-versa

                let vertical_then_horizontal = Vec::from_iter(
                    std::iter::repeat(vertical_direction)
                        .take(dy.unsigned_abs() as usize)
                        .chain(
                            std::iter::repeat(horizontal_direction)
                                .take(dx.unsigned_abs() as usize),
                        ),
                );
                let horizontal_then_vertical = Vec::from_iter(
                    std::iter::repeat(horizontal_direction)
                        .take(dx.unsigned_abs() as usize)
                        .chain(
                            std::iter::repeat(vertical_direction).take(dy.unsigned_abs() as usize),
                        ),
                );

                // We will select the optimal sequence of moves. Empirically (see some
                // Reddit threads), it is alsways better to go left first, then use of
                // the middle keys (up/down), then right, if possible. That means that,
                // if possible, we should go horizontal then vertical first if we are
                // going left, vertical then horizontal in all other cases.
                let possibilities = if dx < 0 {
                    [horizontal_then_vertical, vertical_then_horizontal]
                } else {
                    [vertical_then_horizontal, horizontal_then_vertical]
                };

                let result = possibilities
                    .into_iter()
                    .filter(|moves| {
                        let mut position = start;
                        for dir in moves.iter() {
                            position = position + dir.delta();

                            if !self.inside(position) {
                                return false;
                            }
                        }

                        true
                    })
                    .next()
                    .unwrap();

                self.cache.insert((start, end), result.clone());

                result
            }
            Some(moves) => moves.clone(),
        }
    }

    fn make_path(&mut self, start: &mut Position, positions: &[Position]) -> Vec<ArrowKey> {
        let mut moves = Vec::new();

        for position in positions {
            let path_chunk = self.move_to(*start, *position);
            moves.extend(
                path_chunk
                    .into_iter()
                    .map(|arrow| ArrowKey::Direction(arrow))
                    .chain(std::iter::once(ArrowKey::A)),
            );

            *start = *position;
        }

        moves
    }
}

#[derive(Clone, Copy)]
enum NumericKey {
    A,
    Num(u8),
}

impl NumericKey {
    fn position(self) -> Position {
        match self {
            NumericKey::Num(n) => NUMERIC_KEYPAD[usize::from(n)],
            NumericKey::A => NUMERIC_KEYPAD[10],
        }
    }
}

fn numeric_code(keys: &[NumericKey]) -> i64 {
    keys.into_iter()
        .rev()
        .filter_map(|key| match key {
            NumericKey::A => None,
            NumericKey::Num(n) => Some(i64::from(*n)),
        })
        .enumerate()
        .map(|(exp, n)| n * 10_i64.pow(exp as u32))
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ArrowKey {
    A,
    Direction(Direction),
}

impl ArrowKey {
    fn position(self) -> Position {
        match self {
            ArrowKey::A => ARROW_KEYPAD[0],
            ArrowKey::Direction(Direction::Up) => ARROW_KEYPAD[1],
            ArrowKey::Direction(Direction::Right) => ARROW_KEYPAD[2],
            ArrowKey::Direction(Direction::Down) => ARROW_KEYPAD[3],
            ArrowKey::Direction(Direction::Left) => ARROW_KEYPAD[4],
        }
    }

    fn _display(self) -> char {
        match self {
            ArrowKey::A => 'A',
            ArrowKey::Direction(Direction::Up) => '^',
            ArrowKey::Direction(Direction::Right) => '>',
            ArrowKey::Direction(Direction::Down) => 'v',
            ArrowKey::Direction(Direction::Left) => '<',
        }
    }
}

fn robot(
    mut arrow_board: Board,
    mut sequence: impl Iterator<Item = ArrowKey>,
) -> impl Iterator<Item = Vec<ArrowKey>> {
    let mut current_key = ArrowKey::A;

    std::iter::from_fn(move || {
        let next_key = sequence.next()?;

        let moves = arrow_board.move_to(current_key.position(), next_key.position());
        current_key = next_key;

        let key_sequence = moves
            .into_iter()
            .map(|dir| ArrowKey::Direction(dir))
            .chain(std::iter::once(ArrowKey::A))
            .collect::<Vec<_>>();

        Some(key_sequence)
    })
}

fn robot_stack(
    arrow_board: Board,
    sequence: Box<dyn Iterator<Item = ArrowKey>>,
    robots_involved: usize,
) -> Box<dyn Iterator<Item = ArrowKey>> {
    if robots_involved <= 1 {
        Box::new(sequence)
    } else {
        let robot = Box::new(robot(arrow_board.clone(), sequence).flatten());
        robot_stack(arrow_board, robot, robots_involved - 1)
    }
}

fn compute_door_solution(
    mut numeric_board: Board,
    arrow_board: Board,
    code: &[NumericKey],
    robots_involved: usize,
) -> i64 {
    let pad_positions = code.iter().map(|key| key.position()).collect::<Vec<_>>();

    let initial_sequence = numeric_board.make_path(&mut NumericKey::A.position(), &pad_positions);

    let final_sequence_length = robot_stack(
        arrow_board,
        Box::new(initial_sequence.into_iter()),
        robots_involved,
    )
    .count() as i64;

    numeric_code(&code) * final_sequence_length
}

pub struct Day21 {
    door_codes: Vec<Vec<NumericKey>>,
}

impl Puzzle for Day21 {
    type Output = i64;

    fn parse(input: &str) -> Option<Self> {
        let door_codes = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'A' => Some(NumericKey::A),
                        _ => Some(NumericKey::Num(c.to_digit(10)?.try_into().ok()?)),
                    })
                    .collect::<Option<Vec<_>>>()
            })
            .collect::<Option<Vec<_>>>()?;

        Some(Day21 { door_codes })
    }

    fn part1(self) -> Option<Self::Output> {
        let numeric_board = Board {
            board: &NUMERIC_KEYPAD,
            cache: HashMap::new(),
        };

        let arrow_board = Board {
            board: &ARROW_KEYPAD,
            cache: HashMap::new(),
        };

        let mut result = 0;

        for code in self.door_codes.into_iter() {
            result += compute_door_solution(numeric_board.clone(), arrow_board.clone(), &code, 3);
        }

        Some(result)
    }

    fn part2(self) -> Option<Self::Output> {
        let numeric_board = Board {
            board: &NUMERIC_KEYPAD,
            cache: HashMap::new(),
        };

        let arrow_board = Board {
            board: &ARROW_KEYPAD,
            cache: HashMap::new(),
        };

        let mut result = 0;

        for code in self.door_codes.into_iter() {
            result += compute_door_solution(numeric_board.clone(), arrow_board.clone(), &code, 25);
        }

        Some(result)
    }
}
