use crate::puzzle::Puzzle;

const LOCK_SIZE: usize = 5;

type Shape = [u8; LOCK_SIZE];

enum ShapeKind {
    Lock,
    Key,
}

fn parse_shape(input: &str) -> Option<(ShapeKind, Shape)> {
    let mut lines = input.lines();

    let first_line = lines.next()?;
    let mut shape = [0; LOCK_SIZE];

    for line in lines.take(LOCK_SIZE) {
        for (i, c) in line.chars().enumerate() {
            match c {
                '#' => shape[i] += 1,
                _ => (),
            }
        }
    }

    let kind = if first_line.chars().all(|c| c == '#') {
        ShapeKind::Lock
    } else {
        ShapeKind::Key
    };

    Some((kind, shape))
}

fn fits(lock: Shape, key: Shape) -> bool {
    lock.into_iter()
        .zip(key.into_iter())
        .all(|(x, y)| usize::from(x + y) <= LOCK_SIZE)
}

pub struct Day25 {
    locks: Vec<Shape>,
    keys: Vec<Shape>,
}

impl Puzzle for Day25 {
    type Output = i64;

    fn parse(input: &str) -> Option<Self> {
        let mut locks = Vec::new();
        let mut keys = Vec::new();

        for shape in input.split("\n\n") {
            let (kind, shape) = parse_shape(shape)?;

            match kind {
                ShapeKind::Lock => locks.push(shape),
                ShapeKind::Key => keys.push(shape),
            }
        }

        Some(Day25 { locks, keys })
    }

    fn part1(self) -> Option<Self::Output> {
        let mut fitting = 0;

        for &lock in self.locks.iter() {
            for &key in self.keys.iter() {
                if fits(lock, key) {
                    fitting += 1;
                }
            }
        }

        Some(fitting)
    }

    fn part2(self) -> Option<Self::Output> {
        None
    }
}
