use crate::{position::{Direction, Position}, puzzle::Puzzle};

const NUMERIC_KEYPAD: [Position; 11] = [
    Position { x: -1, y: 0 }, // 0
    Position { x: 0, y:  0}, // A 

    Position { x: -2, y:  1}, // 1 
    Position { x: -1, y:  1}, // 2 
    Position { x: 0, y:  1}, // 3 

    Position { x: -2, y:  2}, // 4 
    Position { x: -1, y:  2}, // 5 
    Position { x: 0, y:  2}, // 6

    Position { x: -2, y:  3}, // 7 
    Position { x: -1, y:  3}, // 8 
    Position { x: 0, y:  3}, // 9
];

const ARROW_KEYPAD: [Position; 5] = [
    Position { x: 0, y: 0 }, // A 
    Position { x: -1, y: 0}, // ^ 

    Position {x: 0, y: -1 }, // > 
    Position {x: -1, y:-1}, // v 
    Position {x: -2, y: -1}, // <
];

struct Board {
    board: &'static [Position], 
}


impl Board {
    fn inside(&self, position: Position) -> bool {
        self.board.contains(&position)
    }

    fn move_on_by(&self, current_position: &mut Position, move_by: Position) -> Vec<Direction> {
        let mut moves = Vec::new();


    }
}

#[derive(Clone, Copy)]
enum NumericKey {
    A,
    Num(u8),
}

impl NumericKey {
    fn optimal(self) -> Vec<ArrowKey> {
        match self {
            NumericKey::A => vec![],
            NumericKey::Num(0) => vec![ArrowKey::Left],
            NumericKey::Num(1) => vec![ArrowKey::Up, ArrowKey::Left, ArrowKey::Left],
            NumericKey::Num(2) => vec![ArrowKey::Up, ArrowKey::Left],
        }
    }
    fn position(self) -> Position {
        match self {
            // We arbitrarily decide that key 1 is at 0,0, with other keys
            // having increasing x,y coordinates
            NumericKey::Num(0) => Position { x: 1, y: -1 },
            NumericKey::Num(n) => Position {
                x: (i64::from(n) - 1) % 3,
                y: (i64::from(n) - 1) / 3,
            },
            NumericKey::A => Position { x: 2, y: -1 },
        }
    }

    fn safe(position: Position) -> bool {
        position != Position {x: 0, y: - 1} && position.x >= 0 && position.x < 3 && position.y >= - 1 && position.y < 3
    }
}

fn position_to_moves(current_position: &mut Position, next_positions: &[Position]) -> Vec<ArrowKey> {
    let mut moves = Vec::new();
    
    while let Some(next_position) = next_positions.iter().next() {
        let delta = *next_position - *current_position;

        let x_move = if delta.x < 0 {
            ArrowKey::Left
        } else {
            ArrowKey::Right
        }; 

        if delta.x < 0 {
            moves.extend(std::iter::repeat(ArrowKey::Left).take(delta.x.abs()))
        }

        moves.

    }

    todo!()
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

fn numeric_key_motion(current_key: &mut NumericKey, keys: &[NumericKey]) -> Position {
    let mut total_motion = Position { x: 0, y: 0 };

    while let Some(key) = keys.into_iter().next() {
        let key_position = key.position();

        total_motion = total_motion + key_position - current_key.position();
        *current_key = *key;
    }

    total_motion
}

fn position_length_sequence(motion: Position) -> i64 {
    1 + motion.x.abs() + motion.y.abs() 
}

fn motion_length_to_next_motion(motion: Position) -> Position {
    // motion is the number of time the robot 
}

// We can describe the point a robot needs to be at with a list of points. The
// sum of the Manhattan distance between these points give us the answer we
// want, as far as the length of the sequence to press is concerned.
//
// For instance, if the original robot (A) needs to type 02A, the sequence of
// moves that robot needs to do is <A^A>vA, which is of length 7. Note that this
// is the same as 1 + 1 + 2 (distances between A and 0, 0 and 2 and 2 and A). To
// that, we need to add 3 for the three times we had to press A.
//
// If we now need a robot to make that robot type these keys. It will have to go 
// <vvA>>^A
//
// When another robot (B) needs to make the original robot (A) perform that
// sequence of moves, it needs to 

#[derive(Clone, Copy)]
enum ArrowKey {
    A,
    Up,
    Down,
    Left,
    Right,
}

impl ArrowKey {
    fn position(self) -> Position {
        match self {
            ArrowKey::A => Position { x: 1, y: 1 },
            ArrowKey::Up => Position { x: 0, y: 1 },
            ArrowKey::Down => Position { x: 0, y: 0 },
            ArrowKey::Left => Position { x: -1, y: 0 },
            ArrowKey::Right => Position { x: 1, y: 0 },
        }
    }
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
        None
    }

    fn part2(self) -> Option<Self::Output> {
        None
    }
}
