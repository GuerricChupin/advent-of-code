use crate::puzzle::Puzzle;

fn concatenate(hd: i64, tl: i64) -> i64 {
    hd * 10_i64.pow(1 + tl.ilog10()) + tl
}

fn handle_equation(
    with_concatenation: bool,
    target: i64,
    current_total: i64,
    operands: &[i64],
) -> bool {
    if target < current_total {
        false
    } else {
        match operands.split_first() {
            None => target == current_total,
            Some((hd, tl)) => {
                handle_equation(with_concatenation, target, current_total + hd, tl)
                    || handle_equation(with_concatenation, target, current_total * hd, tl)
                    || (with_concatenation
                        && handle_equation(
                            with_concatenation,
                            target,
                            concatenate(current_total, *hd),
                            tl,
                        ))
            }
        }
    }
}

struct Equation {
    total: i64,
    operands: Vec<i64>,
}

impl Equation {
    fn possible(&self, with_concatenation: bool) -> bool {
        self.operands
            .split_first()
            .map(|(hd, tl)| handle_equation(with_concatenation, self.total, *hd, tl))
            .unwrap_or(false)
    }
}

pub struct Day07 {
    equations: Vec<Equation>,
}

impl Puzzle for Day07 {
    fn parse(input: &str) -> Option<Self> {
        let equations = input
            .lines()
            .map(|line| {
                let (total, operands) = line.split_once(':')?;
                let total = total.parse::<i64>().ok()?;
                let operands = operands
                    .split_whitespace()
                    .map(|word| word.parse::<i64>().ok())
                    .collect::<Option<Vec<_>>>()?;
                Some(Equation { total, operands })
            })
            .collect::<Option<Vec<_>>>()?;

        Some(Day07 { equations })
    }

    fn part1(self) -> Option<i64> {
        Some(
            self.equations
                .into_iter()
                .filter(|equation| equation.possible(false))
                .map(|equation| equation.total)
                .sum(),
        )
    }

    fn part2(self) -> Option<i64> {
        Some(
            self.equations
                .into_iter()
                .filter(|equation| equation.possible(true))
                .map(|equation| equation.total)
                .sum(),
        )
    }
}
