use std::{
    collections::HashMap,
    ops::{Add, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    pub fn delta(self) -> Position {
        match self {
            Direction::Up => Position { x: 0, y: -1 },
            Direction::Down => Position { x: 0, y: 1 },
            Direction::Left => Position { x: -1, y: 0 },
            Direction::Right => Position { x: 1, y: 0 },
        }
    }

    pub fn next_clockwise(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn next_anticlockwise(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    pub fn reverse(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Position {
    pub fn neighbors_with_directions(self) -> [(Self, Direction); 4] {
        let vectors = [
            (Position { x: 1, y: 0 }, Direction::Right),
            (Position { x: 0, y: 1 }, Direction::Down),
            (Position { x: -1, y: 0 }, Direction::Left),
            (Position { x: 0, y: -1 }, Direction::Up),
        ];

        vectors.map(|(diff, dir)| (self + diff, dir))
    }

    pub fn neighbors(self) -> [Self; 4] {
        self.neighbors_with_directions().map(|(pos, _)| pos)
    }

    pub fn diagonal_neighbors(self) -> [Self; 4] {
        let vectors = [
            Position { x: 1, y: 1 },
            Position { x: 1, y: -1 },
            Position { x: -1, y: -1 },
            Position { x: -1, y: 1 },
        ];

        vectors.map(|diff| self + diff)
    }
}

pub fn read_map<T>(input: &str, map_element: impl Fn(char) -> T) -> HashMap<Position, T> {
    let mut map = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pt = Position {
                x: x as i64,
                y: y as i64,
            };

            map.insert(pt, map_element(c));
        }
    }

    map
}
