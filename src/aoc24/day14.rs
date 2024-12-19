use std::cmp::Ordering;

use regex::Regex;

use crate::{position::Position, puzzle::Puzzle};

const FLOOR_WIDTH: i64 = 101;
const FLOOR_HEIGHT: i64 = 103;

enum Quadrant {
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

fn quadrant(position: Position) -> Option<Quadrant> {
    match (
        position.x.cmp(&(FLOOR_WIDTH / 2)),
        position.y.cmp(&(FLOOR_HEIGHT / 2)),
    ) {
        (Ordering::Equal, _) | (_, Ordering::Equal) => None,
        (Ordering::Less, Ordering::Less) => Some(Quadrant::NorthWest),
        (Ordering::Less, Ordering::Greater) => Some(Quadrant::SouthWest),
        (Ordering::Greater, Ordering::Less) => Some(Quadrant::NorthEast),
        (Ordering::Greater, Ordering::Greater) => Some(Quadrant::SouthEast),
    }
}

struct QuadrantCount {
    north_west: i64,
    north_east: i64,
    south_west: i64,
    south_east: i64,
}

impl QuadrantCount {
    fn safety_score(self) -> i64 {
        self.north_east * self.north_west * self.south_east * self.south_west
    }
}

fn quadrant_count(positions: &[Position]) -> QuadrantCount {
    let mut count = QuadrantCount {
        north_west: 0,
        north_east: 0,
        south_east: 0,
        south_west: 0,
    };

    for position in positions {
        match quadrant(*position) {
            None => (),
            Some(Quadrant::NorthEast) => count.north_east += 1,
            Some(Quadrant::NorthWest) => count.north_west += 1,
            Some(Quadrant::SouthEast) => count.south_east += 1,
            Some(Quadrant::SouthWest) => count.south_west += 1,
        }
    }

    count
}

// This formula is taken from
// https://en.wikipedia.org/wiki/Variance#Discrete_random_variable but without
// division, because we expect all sets of numbers to be of the same size, so
// the n parameter doesn't matter. We are not interested in the actual value of
// the variance but in having something we can compare
fn variance_estimate(values: &[i64]) -> i64 {
    values
        .iter()
        .map(|&xi| values.iter().map(|&xj| (xi - xj).pow(2)).sum::<i64>())
        .sum()
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Position,
    velocity: Position,
}

impl Robot {
    fn position_at(self, time: i64) -> Position {
        Position {
            x: (self.position.x + time * self.velocity.x).rem_euclid(FLOOR_WIDTH),
            y: (self.position.y + time * self.velocity.y).rem_euclid(FLOOR_HEIGHT),
        }
    }
}

pub struct Day14 {
    robots: Vec<Robot>,
}

impl Puzzle for Day14 {
    type Output = i64;
    
    fn parse(input: &str) -> Option<Self> {
        let re = Regex::new(r"p=(?<x>\-?\d+),(?<y>\-?\d+) v=(?<vx>\-?\d+),(?<vy>\-?\d+)").unwrap();

        Some(Day14 {
            robots: input
                .lines()
                .map(|line| {
                    let captures = re.captures(line)?;
                    Some(Robot {
                        position: Position {
                            x: captures["x"].parse().unwrap(),
                            y: captures["y"].parse().unwrap(),
                        },
                        velocity: Position {
                            x: captures["vx"].parse().unwrap(),
                            y: captures["vy"].parse().unwrap(),
                        },
                    })
                })
                .collect::<Option<Vec<_>>>()?,
        })
    }

    fn part1(self) -> Option<i64> {
        let final_robot_positions = self
            .robots
            .into_iter()
            .map(|robot| robot.position_at(100))
            .collect::<Vec<_>>();

        Some(quadrant_count(&final_robot_positions).safety_score())
    }

    fn part2(self) -> Option<i64> {
        // This solution is based on this reddit thread:
        // https://www.reddit.com/r/adventofcode/comments/1he0asr/2024_day_14_part_2_why_have_fun_with_image/
        //
        // The idea is that the easter egg corresponds to a time step of
        // particularly low variance for the x and y coordinates (because a lot
        // of the robots will be in the same place when the Christmas tree
        // appears)
        //
        // For all robots, the x and y coordinates repeat respectively 101 times
        // and 103 times (it doesn't mean that the positions do though). So we
        // compute all the positions they visit in a 103 steps and compute the
        // variance for each of the steps.
        //
        // For x and y, this will give us an index for which the variance is
        // very small

        // For all time, produce a vector with 1) the current time, 2) the
        // variance of the xs, 3) the variance of the ys
        let all_variances = (0..FLOOR_HEIGHT.max(FLOOR_WIDTH))
            .map(|time| {
                let robot_positions = self
                    .robots
                    .iter()
                    .map(|robot| robot.position_at(time))
                    .collect::<Vec<_>>();

                let xs = robot_positions.iter().map(|pos| pos.x).collect::<Vec<_>>();
                let ys = robot_positions.iter().map(|pos| pos.y).collect::<Vec<_>>();

                let x_variance = variance_estimate(&xs);
                let y_variance = variance_estimate(&ys);

                (time, x_variance, y_variance)
            })
            .collect::<Vec<_>>();

        let (n_x, _, _) = all_variances
            .iter()
            .min_by(|(_, x_a, _), (_, x_b, _)| x_a.cmp(x_b))
            .cloned()
            .unwrap();

        let (n_y, _, _) = all_variances
            .iter()
            .min_by(|(_, _, y_a), (_, _, y_b)| y_a.cmp(y_b))
            .cloned()
            .unwrap();

        // We know that the offset we are looking for, t, is of the form:
        //
        //     t % FLOOR_WIDTH  = n_x
        //     t % FLOOR_HEIGHT = n_y
        //
        // We'll use a brute-force approach by starting at t = n_x and just
        // adding the width until we get the result we want
        let mut t = n_x;
        while t % FLOOR_HEIGHT != n_y {
            t += FLOOR_WIDTH;
        }

        Some(t)
    }
}
