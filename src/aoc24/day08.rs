use std::collections::{HashMap, HashSet};

use gcd::Gcd as _;

use crate::{position::Position, puzzle::Puzzle};

impl Position {
    fn in_bounds(self, south_east_corner: Position) -> bool {
        0 <= self.x && self.x <= south_east_corner.x && 0 <= self.y && self.y <= south_east_corner.y
    }
}

fn per_station_antinodes(
    south_east_corner: Position,
    antinodes: &mut HashSet<Position>,
    antennas: &[Position],
) {
    for first_station in antennas.iter() {
        for second_station in antennas
            .iter()
            .filter(|other_station| &first_station != other_station)
        {
            let antinode_position = *second_station + (*second_station - *first_station);

            if antinode_position.in_bounds(south_east_corner) {
                let _already_seen = antinodes.insert(antinode_position);
            }
        }
    }
}

fn pairwise_line_antinodes(
    south_east_corner: Position,
    antinodes: &mut HashSet<Position>,
    first_station: Position,
    second_station: Position,
) {
    let unit_diff_vector = {
        let diff_vector = second_station - first_station;

        let gcd = (diff_vector.x.unsigned_abs()).gcd(diff_vector.y.unsigned_abs());

        Position {
            x: diff_vector.x / gcd as i64,
            y: diff_vector.y / gcd as i64,
        }
    };

    let mut next_point = first_station + unit_diff_vector;

    while next_point.in_bounds(south_east_corner) {
        let _already_visited = antinodes.insert(next_point);

        next_point = next_point + unit_diff_vector;
    }
}

fn in_line_antinodes(
    south_east_corner: Position,
    antinodes: &mut HashSet<Position>,
    antennas: &[Position],
) {
    for first_station in antennas.iter() {
        for second_station in antennas
            .iter()
            .filter(|other_station| &first_station != other_station)
        {
            pairwise_line_antinodes(
                south_east_corner,
                antinodes,
                *first_station,
                *second_station,
            );
        }
    }
}

pub struct Day08 {
    antennas: HashMap<char, Vec<Position>>,
    south_east_corner: Position,
}

impl Puzzle for Day08 {
    type Output = i64;
    
    fn parse(input: &str) -> Option<Self> {
        let mut south_east_corner = Position { x: 0, y: 0 };
        let mut antennas = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            let y = y as i64;
            south_east_corner.y = south_east_corner.y.max(y);

            for (x, c) in line.chars().enumerate() {
                let x = x as i64;
                south_east_corner.x = south_east_corner.x.max(x);

                match c {
                    '.' => (),
                    c => antennas
                        .entry(c)
                        .or_insert_with(Vec::new)
                        .push(Position { x, y }),
                }
            }
        }

        Some(Day08 {
            antennas,
            south_east_corner,
        })
    }

    fn part1(self) -> Option<i64> {
        let mut antinodes = HashSet::new();

        for (_, antennas) in self.antennas.iter() {
            per_station_antinodes(self.south_east_corner, &mut antinodes, antennas);
        }

        Some(antinodes.len() as i64)
    }

    fn part2(self) -> Option<i64> {
        let mut antinodes = HashSet::new();

        for (_, antennas) in self.antennas.iter() {
            in_line_antinodes(self.south_east_corner, &mut antinodes, antennas);
        }

        Some(antinodes.len() as i64)
    }
}
