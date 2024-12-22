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

const NUMERIC_BOARD: Board = Board {
    board: &NUMERIC_KEYPAD,
};

const ARROW_KEYPAD: [Position; 5] = [
    Position { x: 0, y: 0 },  // A
    Position { x: -1, y: 0 }, // ^
    Position { x: 0, y: 1 },  // >
    Position { x: -1, y: 1 }, // v
    Position { x: -2, y: 1 }, // <
];

const ARROW_BOARD: Board = Board {
    board: &ARROW_KEYPAD,
};

struct Board {
    board: &'static [Position],
}

impl Board {
    fn inside(&self, position: Position) -> bool {
        self.board.contains(&position)
    }

    fn move_to(&self, start: Position, end: Position) -> Vec<Direction> {
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

                    if !self.inside(position) {
                        return false;
                    }
                }

                true
            })
            .next()
            .unwrap()
    }

    fn make_path(&self, start: &mut Position, positions: &[Position]) -> Vec<ArrowKey> {
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

#[derive(Clone, Copy)]
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

fn compute_door_solution(code: &[NumericKey], robots_involved: usize) -> i64 {
    let pad_positions = code.iter().map(|key| key.position()).collect::<Vec<_>>();

    let mut path = NUMERIC_BOARD.make_path(&mut NumericKey::A.position(), &pad_positions);

    for _robot_id in 1..robots_involved {
        let positions = path
            .into_iter()
            .map(|key| key.position())
            .collect::<Vec<_>>();
        path = ARROW_BOARD.make_path(&mut ArrowKey::A.position(), &positions);
    }

    numeric_code(&code) * path.len() as i64
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
        Some(
            self.door_codes
                .into_iter()
                .map(|code| compute_door_solution(&code, 3))
                .sum(),
        )
    }

    fn part2(self) -> Option<Self::Output> {
        Some(
            self.door_codes
                .into_iter()
                .map(|code| compute_door_solution(&code, 25))
                .sum(),
        )
    }
}
