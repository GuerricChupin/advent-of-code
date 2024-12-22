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

fn inside_board(board: &[Position], position: Position) -> bool {
    board.contains(&position)
}

fn move_to(board: &[Position], start: Position, end: Position) -> Vec<Direction> {
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
            .chain(std::iter::repeat(horizontal_direction).take(dx.unsigned_abs() as usize)),
    );
    let horizontal_then_vertical = Vec::from_iter(
        std::iter::repeat(horizontal_direction)
            .take(dx.unsigned_abs() as usize)
            .chain(std::iter::repeat(vertical_direction).take(dy.unsigned_abs() as usize)),
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

    possibilities
        .into_iter()
        .filter(|moves| {
            let mut position = start;
            for dir in moves.iter() {
                position = position + dir.delta();

                if !inside_board(board, position) {
                    return false;
                }
            }

            true
        })
        .next()
        .unwrap()
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
}

// This function returns the list of keys to press, so that the robot controlled
// by the keypad, moves from the current_key to the next_key
fn compute_keys_to_press(
    board: &[Position],
    current_position: Position,
    next_position: Position,
) -> Vec<ArrowKey> {
    move_to(board, current_position, next_position)
        .into_iter()
        .map(|dir| ArrowKey::Direction(dir))
        .chain(std::iter::once(ArrowKey::A))
        .collect::<Vec<_>>()
}

fn recursively_compute_length(
    board: &[Position],
    keys_to_press: &[ArrowKey],
    stack_id: usize,
    cache: &mut Cache,
) -> i64 {
    if stack_id == 0 {
        keys_to_press.len() as i64
    } else {
        let mut current_key = ArrowKey::A;
        let mut result = 0;

        for &next_key in keys_to_press {
            let current_position = current_key.position();
            let next_position = next_key.position();
            match cache.get(&(stack_id, current_position, next_position)) {
                Some(value) => {
                    result += *value;
                }
                None => {
                    let value = recursively_compute_length(
                        board,
                        &compute_keys_to_press(board, current_position, next_position),
                        stack_id - 1,
                        cache,
                    );

                    cache.insert((stack_id, current_position, next_position), value);

                    result += value;
                }
            }

            current_key = next_key;
        }

        result
    }
}

type Cache = HashMap<(usize, Position, Position), i64>;

fn compute_door_solution(code: &[NumericKey], robots_involved: usize) -> i64 {
    let initial_sequence = {
        let mut sequence = Vec::new();
        let mut current_key = NumericKey::A;

        for &key in code.iter() {
            let mut moves =
                compute_keys_to_press(&NUMERIC_KEYPAD, current_key.position(), key.position());
            sequence.append(&mut moves);
            current_key = key;
        }

        sequence
    };

    let mut cache = HashMap::new();

    let final_sequence_length = recursively_compute_length(
        &ARROW_KEYPAD,
        &initial_sequence,
        robots_involved - 1,
        &mut cache,
    );

    numeric_code(&code) * final_sequence_length
}

fn sum_door_solution(codes: Vec<Vec<NumericKey>>, robots_involved: usize) -> i64 {
    codes
        .into_iter()
        .map(|code| compute_door_solution(&code, robots_involved))
        .sum()
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
        Some(sum_door_solution(self.door_codes, 3))
    }

    fn part2(self) -> Option<Self::Output> {
        Some(sum_door_solution(self.door_codes, 26))
    }
}
