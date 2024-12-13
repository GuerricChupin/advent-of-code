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

fn patch_side_count(patch: &HashSet<Position>) -> i64 {
    // The number of sides is equal to the number of angles of the polygon (even
    // if it has holes). So we will just count the number of angles contributed
    // by each patch element and return that
    let mut number_of_angles = 0;

    for &patch_element in patch.iter() {
        let neighbours_in_the_patch = patch_element
            .neighbors()
            .into_iter()
            .filter(|other| patch.contains(other))
            .collect::<Vec<_>>();

        number_of_angles += match neighbours_in_the_patch.as_slice() {
            [] => 4, // If no neighbours are in the patch, the patch is a single square with four sides

            [_] => 2, // If there is a single neighbour in the patch, we are at the end of a line like so:
            //      -----+
            //      X  X |
            //      -----+
            //
            // so this element contributes 2 angles
            [p1, p2] => {
                // When there are 2 neighbours, either the neighbours are
                // parallel, in which case the patch counts for 0 or they make
                // an angle, in which case it counts for 1 or 2 (see below to
                // distinguish between these two cases)
                let dx = (p1.x - p2.x).abs();
                let dy = (p1.y - p2.y).abs();

                // If the two neighbours are parallel, either dx or dy is 2.
                // Otherwise they should be both one
                assert!((dx == 2 && dy == 0) || (dx == 0 && dy == 2) || (dx == dy && dx == 1));

                if dx == 2 || dy == 2 {
                    0
                } else {
                    let dp1 = *p1 - patch_element;
                    let dp2 = *p2 - patch_element;
                    let inside_patch_element = patch_element + dp1 + dp2;
                    if patch.contains(&inside_patch_element) {
                        1
                    } else {
                        2
                    }
                }
            }
            [p1, p2, p3] => {
                // When there are 3 neighbours in the patch, it may contribute
                // 0, 1 or 2 angles. Each case is illustrated in the figures
                // below, looking at the bottom center X:
                //
                //     X X X
                //     X X X
                //     +---+
                //
                //     A | X   X
                //     --+
                //     X   X   X
                //     ---------
                //
                //     A | X | A
                //     --+   +--
                //     X   X   X
                //     ---------
                //
                // We will only consider adding the angles that are between the
                // three neighbours (the ones marked on the diagram with a plus
                // to avoid any overcounting). Indeed, consider a situation like
                // this:
                //
                //     X   X   X
                //     X   X
                //     X   X   X
                //
                // If we aren't careful, we might count 2 angles for the center
                // X and 1 angle for the top center X and the bottom center X,
                // leading to 4 angles when there are only two

                // We calculate the vectors from the patch element to each neighbour
                let dp1 = *p1 - patch_element;
                let dp2 = *p2 - patch_element;
                let dp3 = *p3 - patch_element;

                // We then construct all the "diagonal" points by adding each
                // vector to the patch element. Actually we only get two
                // diagonal point and the original point back, but that doesn't
                // really matter

                let d1 = patch_element + dp1 + dp2;
                let d2 = patch_element + dp1 + dp3;
                let d3 = patch_element + dp2 + dp3;

                // The total number of angles in the number of diagonal points
                // that are *not* in the patch. This eliminate the original
                // point (which must be in the patch) so the answer is
                // necessarily contained between 0 and 2
                [d1, d2, d3]
                    .into_iter()
                    .filter(|d| !patch.contains(d))
                    .count() as i64
            }
            [_, _, _, _] => {
                // If all the neigbours are in the patch, this element
                // contributes anywhere between 0 and 4 angles. This can be
                // easily calculated by just counting how many diagonal elements
                // are in the patch

                patch_element
                    .diagonal_neighbors()
                    .into_iter()
                    .filter(|other| !patch.contains(other))
                    .count() as i64
            }
            _ => unreachable!("There cannot be more than 4 neighbours to a patch element"),
        };
    }

    number_of_angles
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

    fn part2(mut self) -> Option<i64> {
        let patches = make_all_patches(&mut self.map);

        Some(
            patches
                .into_iter()
                .map(|patch| patch_area(&patch) * patch_side_count(&patch))
                .sum(),
        )
    }
}
