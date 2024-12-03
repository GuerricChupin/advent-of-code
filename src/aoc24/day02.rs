use crate::puzzle::Puzzle;

pub struct Day02 {
    reports: Vec<Vec<i64>>,
}

impl Puzzle for Day02 {
    fn parse(input: &str) -> Option<Self> {
        Some(Day02 {
            reports: input
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|number| number.parse::<i64>().ok())
                        .collect::<Option<Vec<_>>>()
                })
                .collect::<Option<Vec<_>>>()?,
        })
    }

    fn part1(self) -> Option<i64> {
        Some(
            self.reports
                .into_iter()
                .filter(|report| is_report_safe(report))
                .count() as i64,
        )
    }

    fn part2(self) -> Option<i64> {
        Some(
            self.reports
                .into_iter()
                .filter(|report| is_report_loosely_safe(&report))
                .count() as i64,
        )
    }
}

fn compute_differences(report: &[i64]) -> Vec<i64> {
    let mut diff = Vec::with_capacity(report.len());

    let mut previous = None;

    for &value in report {
        if let Some(prev) = previous {
            diff.push(value - prev);
        }

        previous = Some(value);
    }

    diff
}

fn safe_differences(differences: &[i64]) -> bool {
    let all_ascending = differences.iter().all(|&diff| diff >= 0);
    let all_descending = differences.iter().all(|&diff| diff <= 0);
    let valid_gaps = differences
        .iter()
        .all(|diff| 1 <= diff.abs() && diff.abs() <= 3);

    (all_ascending || all_descending) && valid_gaps
}

fn is_report_safe(report: &[i64]) -> bool {
    let differences = compute_differences(report);

    safe_differences(&differences)
}

fn truncated_reports(report: &[i64]) -> Vec<Vec<i64>> {
    let mut truncs = Vec::with_capacity(report.len());

    for i in 0..report.len() {
        let mut truncated_report = Vec::with_capacity(report.len());
        for (_, value) in report.iter().enumerate().filter(|(j, _)| i != *j) {
            truncated_report.push(*value)
        }

        truncs.push(truncated_report)
    }

    truncs
}

// There is probably a more efficient solution that considers the list of
// differences and tries to remove an outlier, but I can't quite figure it out
fn is_report_loosely_safe(report: &[i64]) -> bool {
    is_report_safe(report)
        || truncated_reports(report)
            .into_iter()
            .any(|truncated_report| {
                 is_report_safe(&truncated_report)
            })
}
