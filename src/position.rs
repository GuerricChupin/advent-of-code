use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
}
