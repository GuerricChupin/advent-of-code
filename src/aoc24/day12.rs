use std::collections::{HashMap, HashSet};

use crate::{
    position::{read_map, Position},
    puzzle::Puzzle,
};

fn make_patch(
    position: Position,
    patch_type: char,
    map: &mut HashMap<Position, char>,
    this_patch: &mut HashSet<Position>,
) {
    let _ = map.remove(&position);
    this_patch.insert(position);

    let neighbors = position.neighbors();

    for other in neighbors {
        if Some(&patch_type) == map.get(&other) {
            make_patch(other, patch_type, map, this_patch);
        }
    }
}

fn make_all_patches(map: &mut HashMap<Position, char>) -> Vec<HashSet<Position>> {
    let mut patches = Vec::new();

    while !map.is_empty() {
        let (start_position, patch_type) = map.iter().next().unwrap();
        let mut this_patch = HashSet::new();

        make_patch(*start_position, *patch_type, map, &mut this_patch);

        patches.push(this_patch);
    }

    patches
}

fn patch_perimeter(patch: &HashSet<Position>) -> i64 {
    let mut perimeter = 0;

    for &patch_element in patch.iter() {
        // We are counting how many fences the patch element contributes to.
        // This depends on the number of neighbours that are also in the patch.
        // By default, if this element has no neighbour, it requires 4 fences.
        // Each neighbour in the patch reduces that number by 1

        let neighbours_in_the_patch = patch_element
            .neighbors()
            .into_iter()
            .filter(|other| patch.contains(other))
            .count();

        perimeter += 4 - neighbours_in_the_patch as i64
    }

    perimeter
}

fn patch_area(patch: &HashSet<Position>) -> i64 {
    patch.len() as i64
}

pub struct Day12 {
    map: HashMap<Position, char>,
}

impl Puzzle for Day12 {
    fn parse(input: &str) -> Option<Self> {
        Some(Day12 {
            map: read_map(input, |c| c),
        })
    }

    fn part1(mut self) -> Option<i64> {
        let patches = make_all_patches(&mut self.map);
        Some(
            patches
                .into_iter()
                .map(|patch| patch_area(&patch) * patch_perimeter(&patch))
                .sum(),
        )
    }

    fn part2(self) -> Option<i64> {
        None
    }
}
