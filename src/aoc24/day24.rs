use std::collections::{BTreeSet, HashMap, HashSet};

use regex::Regex;

use crate::puzzle::Puzzle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Register {
    X,
    Y,
    Z,
}

impl Register {
    fn is_input(self) -> bool {
        match self {
            Register::X | Register::Y => true,
            Register::Z => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Wire {
    Numbered(Register, u32),
    Named([char; 3]),
}

impl Wire {
    fn new(wire: &str) -> Option<Self> {
        let mut chars = wire.chars();
        let a = chars.next()?;
        let b = chars.next()?;
        let c = chars.next()?;

        let reg = match a {
            'x' => Some(Register::X),
            'y' => Some(Register::Y),
            'z' => Some(Register::Z),
            _ => None,
        };

        match (reg, b.to_digit(10), c.to_digit(10)) {
            (Some(reg), Some(b), Some(c)) => Some(Wire::Numbered(reg, 10 * b + c)),
            _ => Some(Wire::Named([a, b, c])),
        }
    }

    fn to_prefix(&self) -> String {
        match self {
            Wire::Numbered(register, bit) => format!("{:?}{:02}", register, bit),
            Wire::Named(name) => name.iter().collect(),
        }
    }
}

enum Gate {
    And,
    Xor,
    Or,
}

struct Connection {
    fst: Wire,
    snd: Wire,
    gate: Gate,
}

fn value_of(
    rules: &HashMap<Wire, Connection>,
    known_values: &mut HashMap<Wire, bool>,
    wire: Wire,
) -> bool {
    match known_values.get(&wire) {
        Some(value) => *value,
        None => {
            let rule = rules.get(&wire).expect("Missing rule :(");
            let fst = value_of(rules, known_values, rule.fst);
            let snd = value_of(rules, known_values, rule.snd);

            let result = match rule.gate {
                Gate::And => fst && snd,
                Gate::Xor => fst ^ snd,
                Gate::Or => fst || snd,
            };

            known_values.insert(wire, result);

            result
        }
    }
}

fn get_zs(rules: &HashMap<Wire, Connection>, known_values: &mut HashMap<Wire, bool>) -> u64 {
    let mut result = 0;

    for &wire in rules.keys() {
        match wire {
            Wire::Numbered(Register::Z, n) => {
                let on = value_of(rules, known_values, wire);

                if on {
                    result |= 1_u64 << n;
                }
            }
            Wire::Named(_) | Wire::Numbered(_, _) => (),
        }
    }

    result
}

// fn foo(input_bit_count: u32) -> () {
//     let mut current_carry = None;

//     for bit in (0..input_bit_count).rev() {
//         let output_wire = Wire::Numbered(Register::Z, bit);

//         let first_sum = Connection {
//             fst: Wire::Numbered(Register::X, bit),
//             snd: Wire::Numbered(Register::Y, bit),
//             gate: Gate::Xor,
//         };
//         let this_carry = Connection {
//             fst: Wire::Numbered(Register::X, bit),
//             snd: Wire::Numbered(Register::Y, bit),
//             gate: Gate::Xor,
//         };

//         let final_sum = if let Some(carry) = current_carry {
//             Some(Connection {
//                 fst: _first_sum_wire,
//                 snd: carry,
//                 gate: Gate::Xor,
//             })
//         } else {
//             None
//         };

//         let carry_out_part_1 = Connection {
//             fst: Wire::Numbered(Register::X, bit),

//         }
//     }
// }

// fn half_adder<'c>(
//     ctx: &'c z3::Context,
//     a: &z3::ast::Bool<'c>,
//     b: &z3::ast::Bool<'c>,
// ) -> (z3::ast::Bool<'c>, z3::ast::Bool<'c>) {
//     let sum = a.xor(b);
//     let carry = z3::ast::Bool::and(ctx, &[a, b]);

//     (sum, carry)
// }

// fn full_adder<'c>(
//     ctx: &'c z3::Context,
//     a: &z3::ast::Bool<'c>,
//     b: &z3::ast::Bool<'c>,
//     carry: &z3::ast::Bool<'c>,
// ) -> (z3::ast::Bool<'c>, z3::ast::Bool<'c>) {
//     let (s_1, c_1) = half_adder(ctx, a, b);
//     let (sum, c_2) = half_adder(ctx, carry, &s_1);
//     let carry = z3::ast::Bool::or(ctx, &[&c_1, &c_2]);

//     (sum, carry)
// }

pub struct Day24 {
    inputs: HashMap<Wire, bool>,
    connections: HashMap<Wire, Connection>,
}

impl Puzzle for Day24 {
    type Output = String;

    fn parse(input: &str) -> Option<Self> {
        let input_regex = Regex::new(r"(?<wire>[a-z0-9]+): (?<value>\d+)").unwrap();
        let connection_regex = Regex::new(
            r"(?<fst>[a-z0-9]+) (?<gate>OR|XOR|AND) (?<snd>[a-z0-9]+) -> (?<output>[a-z0-9]+)",
        )
        .unwrap();

        let (inputs, connections) = input.split_once("\n\n")?;

        let inputs = inputs
            .lines()
            .map(|line| {
                let capture = input_regex.captures(line)?;
                let value = capture["value"].parse::<i64>().unwrap();

                Some((Wire::new(&capture["wire"])?, value != 0))
            })
            .collect::<Option<HashMap<_, _>>>()?;

        let connections = connections
            .lines()
            .map(|line| {
                let capture = connection_regex.captures(line)?;
                let fst = Wire::new(&capture["fst"])?;
                let snd = Wire::new(&capture["snd"])?;
                let output = Wire::new(&capture["output"])?;
                let gate = match &capture["gate"] {
                    "OR" => Gate::Or,
                    "XOR" => Gate::Xor,
                    "AND" => Gate::And,
                    unknown => panic!("Not a gate {unknown}"),
                };

                Some((output, Connection { fst, snd, gate }))
            })
            .collect::<Option<HashMap<_, _>>>()?;

        Some(Day24 {
            inputs,
            connections,
        })
    }

    fn part1(self) -> Option<Self::Output> {
        let mut known_values = self.inputs;

        Some(format!("{}", get_zs(&self.connections, &mut known_values)))
    }

    fn part2(self) -> Option<Self::Output> {
        None
    }
}
