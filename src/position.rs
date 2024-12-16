use std::{
    collections::HashMap,
    ops::{Add, Sub},
};

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
    pub fn neighbors(self) -> [Self; 4] {
        let vectors = [
            Position { x: 1, y: 0 },
            Position { x: 0, y: 1 },
            Position { x: -1, y: 0 },
            Position { x: 0, y: -1 },
        ];

        vectors.map(|diff| self + diff)
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
