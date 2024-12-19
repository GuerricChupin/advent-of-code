use regex::Regex;

use crate::{position::Position, puzzle::Puzzle};

#[derive(Debug)]
struct Solution {
    a_presses: i64,
    b_presses: i64,
}

impl Solution {
    fn price(self) -> i64 {
        3 * self.a_presses + 1 * self.b_presses
    }
}

#[derive(Debug)]
struct Machine {
    button_a: Position,
    button_b: Position,
    prize: Position,
}

impl Machine {
    fn solve(&self) -> Option<Solution> {
        let a = self.button_a.x;
        let b = self.button_b.x;
        let c = self.button_a.y;
        let d = self.button_b.y;

        let denominator = a * d - b * c;

        let a_numerator = d * self.prize.x - b * self.prize.y;
        let b_numerator = -c * self.prize.x + a * self.prize.y;

        if denominator == 0 || a_numerator % denominator != 0 || b_numerator % denominator != 0 {
            None
        } else {
            let a_presses = a_numerator / denominator;
            let b_presses = b_numerator / denominator;

            Some(Solution {
                a_presses,
                b_presses,
            })
        }
    }
}

pub struct Day13 {
    machine_list: Vec<Machine>,
}

impl Puzzle for Day13 {
    type Output = i64;
    
    fn parse(input: &str) -> Option<Self> {
        let button_regex = Regex::new(r"Button (A|B): X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
        let prize_regex = Regex::new(r"Prize: X=(?<x>\d+), Y=(?<y>\d+)").unwrap();

        let mut machines = Vec::new();

        for block in input.split("\n\n") {
            let mut lines = block.lines();

            let button_a_spec = button_regex.captures(lines.next()?)?;
            let button_b_spec = button_regex.captures(lines.next()?)?;
            let prize_spec = prize_regex.captures(lines.next()?)?;

            let machine = Machine {
                button_a: Position {
                    x: button_a_spec["x"].parse::<i64>().unwrap(),
                    y: button_a_spec["y"].parse::<i64>().unwrap(),
                },
                button_b: Position {
                    x: button_b_spec["x"].parse::<i64>().unwrap(),
                    y: button_b_spec["y"].parse::<i64>().unwrap(),
                },
                prize: Position {
                    x: prize_spec["x"].parse::<i64>().unwrap(),
                    y: prize_spec["y"].parse::<i64>().unwrap(),
                },
            };

            machines.push(machine);
        }

        Some(Day13 {
            machine_list: machines,
        })
    }

    fn part1(self) -> Option<i64> {
        Some(
            self.machine_list
                .into_iter()
                .filter_map(|machine| machine.solve())
                .map(|solution| solution.price())
                .sum(),
        )
    }

    fn part2(self) -> Option<i64> {
        let offset = Position {
            x: 10000000000000,
            y: 10000000000000,
        };

        Some(
            self.machine_list
                .into_iter()
                .map(|machine| Machine {
                    prize: machine.prize + offset,
                    ..machine
                })
                .filter_map(|machine| machine.solve())
                .map(|solution| solution.price())
                .sum(),
        )
    }
}
