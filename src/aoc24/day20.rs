use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::{
    position::{read_map, Position},
    puzzle::Puzzle,
};

#[derive(Copy, Clone, Eq, PartialEq)]
struct PathPoint {
    pos: Position,
    score: i64,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl PartialOrd for PathPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PathPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .score
            .cmp(&self.score)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

pub struct Day20 {
    start: Position,
    end: Position,
    obstacles: HashSet<Position>,
}

fn neighbours_at_a_distance(position: Position, max_distance: usize) -> HashMap<Position, i64> {
    let mut neighbours = HashMap::new();

    let mut to_visit = Vec::from([position]);

    for distance in 1..=max_distance {
        let mut to_visit_next = Vec::new();

        while let Some(pos) = to_visit.pop() {
            for other in pos.neighbors() {
                if !neighbours.contains_key(&other) {
                    neighbours.insert(other, distance as i64);
                    to_visit_next.push(other);
                }
            }
        }

        to_visit = to_visit_next;
    }

    neighbours
}

fn find_shortcuts_from_current_position(
    max_cheat_duration: usize,
    current_position: Position,
    path: &[Position],
    shortcuts: &mut Vec<i64>,
) {
    let neighbours = neighbours_at_a_distance(current_position, max_cheat_duration);

    for (delta, other) in path.iter().enumerate() {
        let delta = delta as i64;

        if let Some(cost) = neighbours.get(other) {
            let saving = delta - cost;

            if saving > 0 {
                shortcuts.push(saving)
            }
        }
    }
}

fn find_all_shortcuts_mut(max_cheat_duration: usize, path: &[Position], shortcuts: &mut Vec<i64>) {
    match path.split_first() {
        None => (),
        Some((hd, tl)) => {
            find_shortcuts_from_current_position(max_cheat_duration, *hd, path, shortcuts);
            find_all_shortcuts_mut(max_cheat_duration, tl, shortcuts);
        }
    }
}

fn find_all_shortcuts(max_cheat_duration: usize, path: &[Position]) -> Vec<i64> {
    let mut shortcuts = Vec::new();

    find_all_shortcuts_mut(max_cheat_duration, path, &mut shortcuts);

    shortcuts
}

fn find_path(start: Position, end: Position, obstacles: &HashSet<Position>) -> Vec<Position> {
    let mut unvisited_nodes = BinaryHeap::from([PathPoint {
        pos: start,
        score: 0,
    }]);
    let mut visited_nodes = HashMap::new();

    while let Some(PathPoint { pos, score }) = unvisited_nodes.pop() {
        let neighbours = pos
            .neighbors()
            .into_iter()
            .filter(|other| !obstacles.contains(&other))
            .filter(|other| !visited_nodes.contains_key(other))
            .map(|other| PathPoint {
                pos: other,
                score: score + 1,
            });

        unvisited_nodes.extend(neighbours);

        visited_nodes.insert(pos, score);
    }

    let mut path_heap = BinaryHeap::from_iter(
        visited_nodes
            .into_iter()
            .map(|(pos, score)| PathPoint { pos, score }),
    );

    let mut path = Vec::new();

    while let Some(PathPoint { pos, .. }) = path_heap.pop() {
        path.push(pos);

        if pos == end {
            break;
        }
    }

    path
}

impl Puzzle for Day20 {
    type Output = i64;

    fn parse(input: &str) -> Option<Self> {
        let mut start = None;
        let mut end = None;
        let mut obstacles = HashSet::new();

        read_map(input, |pos, c| match c {
            '#' => {
                let _ = obstacles.insert(pos);
            }
            'S' => start = Some(pos),
            'E' => end = Some(pos),
            _ => (),
        });

        Some(Day20 {
            start: start?,
            end: end?,
            obstacles,
        })
    }

    fn part1(self) -> Option<Self::Output> {
        let path = find_path(self.start, self.end, &self.obstacles);

        let shortcuts = find_all_shortcuts(2, &path);

        Some(shortcuts.into_iter().filter(|value| *value >= 100).count() as i64)
    }

    fn part2(self) -> Option<Self::Output> {
        let path = find_path(self.start, self.end, &self.obstacles);

        let shortcuts = find_all_shortcuts(20, &path);

        Some(shortcuts.into_iter().filter(|value| *value >= 100).count() as i64)
    }
}
