use std::collections::HashMap;

pub fn part_1(input: &str) -> Option<i64> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let mut whitespace = line.split_whitespace();
        left.push(whitespace.next()?.parse::<i64>().ok()?);
        right.push(whitespace.next()?.parse::<i64>().ok()?);
    }

    left.sort();
    right.sort();

    Some(
        left.into_iter()
            .zip(right.into_iter())
            .map(|(x, y)| (x - y).abs())
            .sum(),
    )
}

pub fn part_2(input: &str) -> Option<i64> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let mut whitespace = line.split_whitespace();
        left.push(whitespace.next()?.parse::<i64>().ok()?);
        right.push(whitespace.next()?.parse::<i64>().ok()?);
    }

    let mut right_frequency = HashMap::new();
    for number in right.into_iter() {
        let entry = right_frequency.entry(number);
        *entry.or_insert(0) += 1;
    }

    Some(
        left.into_iter()
            .map(|number| number * right_frequency.get(&number).cloned().unwrap_or(0))
            .sum(),
    )
}
