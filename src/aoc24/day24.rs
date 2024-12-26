use std::collections::HashMap;

use regex::Regex;

use crate::puzzle::Puzzle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Register {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    fn show(&self) -> String {
        match self {
            Wire::Numbered(register, bit) => {
                let register = match register {
                    Register::X => 'x',
                    Register::Y => 'y',
                    Register::Z => 'z',
                };

                format!("{}{:02}", register, bit)
            }
            Wire::Named(name) => name.iter().collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Gate {
    And,
    Xor,
    Or,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Connection {
    fst: Wire,
    snd: Wire,
    gate: Gate,
}

impl Connection {
    fn new(fst: Wire, snd: Wire, gate: Gate) -> Self {
        let new_fst = fst.min(snd);
        let new_snd = fst.max(snd);

        Connection {
            fst: new_fst,
            snd: new_snd,
            gate,
        }
    }
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

fn is_input_of_gate(
    connections: &HashMap<Wire, Connection>,
    output: Wire,
    gate_type: Gate,
) -> bool {
    connections
        .iter()
        .filter(|(_, Connection { fst, snd, gate })| {
            gate == &gate_type && (fst == &output || snd == &output)
        })
        .next()
        .is_some()
}

fn valid_xor_gate(
    connections: &HashMap<Wire, Connection>,
    fst: Wire,
    snd: Wire,
    output: Wire,
) -> bool {
    match (fst, snd, output) {
        (Wire::Numbered(_, _), Wire::Numbered(_, _), Wire::Named(_)) => {
            is_input_of_gate(connections, output, Gate::Xor)
        }
        (
            Wire::Numbered(Register::X, 0),
            Wire::Numbered(Register::Y, 0),
            Wire::Numbered(Register::Z, 0),
        )
        | (Wire::Named(_), Wire::Named(_), Wire::Numbered(_, _)) => true,
        _ => false,
    }
}

fn valid_and_gate(
    connections: &HashMap<Wire, Connection>,
    fst: Wire,
    snd: Wire,
    output: Wire,
) -> bool {
    match (fst, snd, output) {
        (Wire::Numbered(Register::X, 0), Wire::Numbered(Register::Y, 0), Wire::Named(_)) => true,
        (Wire::Numbered(_, _), Wire::Numbered(_, _), Wire::Named(_))
        | (Wire::Named(_), Wire::Named(_), Wire::Named(_)) => {
            is_input_of_gate(connections, output, Gate::Or)
        }
        _ => false,
    }
}

fn valid_or_gate(
    input_bit_count: u32,
    fst: Wire,
    snd: Wire,
    output: Wire,
) -> bool {
    match (fst, snd, output) {
        (Wire::Named(_), Wire::Named(_), Wire::Named(_)) => true,
        (Wire::Named(_), Wire::Named(_), Wire::Numbered(Register::Z, bit)) => {
            input_bit_count + 1 == bit
        }
        _ => false,
    }
}

fn is_valid_gate(
    connections: &HashMap<Wire, Connection>,
    input_bit_count: u32,
    Connection { fst, snd, gate }: &Connection,
    output: Wire,
) -> bool {
    match gate {
        Gate::And => valid_and_gate(connections, *fst, *snd, output),
        Gate::Xor => valid_xor_gate(connections, *fst, *snd, output),
        Gate::Or => valid_or_gate(input_bit_count, *fst, *snd, output),
    }
}

fn extract_odd_gates(input_bit_count: u32, connections: &HashMap<Wire, Connection>) -> Vec<Wire> {
    connections
        .into_iter()
        .filter(|(wire, connection)| {
            !is_valid_gate(connections, input_bit_count, connection, **wire)
        })
        .map(|(wire, _)| *wire)
        .collect()
}

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

                Some((output, Connection::new(fst, snd, gate)))
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
        // The solution for this part is taken from
        // https://www.reddit.com/r/adventofcode/comments/1hla5ql/comment/m3kws15/.
        // I find it very unsatisfactory but it works :/
        let input_bit_count = self
            .inputs
            .into_iter()
            .filter_map(|(register, _)| match register {
                Wire::Numbered(_, bit) => Some(bit),
                Wire::Named(_) => None,
            })
            .max()
            .unwrap();

        let odd_gates = extract_odd_gates(input_bit_count, &self.connections);
        let mut odd_gates = odd_gates
            .into_iter()
            .map(|wire| wire.show())
            .collect::<Vec<_>>();
        odd_gates.sort();
        Some(odd_gates.join(","))
    }
}
