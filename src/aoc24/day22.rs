use std::collections::HashMap;

use crate::puzzle::Puzzle;

pub struct Day22 {
    secrets: Vec<u64>,
}

fn price(secret: u64) -> u64 {
    secret % 10
}

fn mix(secret: u64, mixer: u64) -> u64 {
    secret ^ mixer
}

fn prune(secret: u64) -> u64 {
    secret % 16_777_216
}

fn step_secret(secret: u64) -> u64 {
    let mixer = secret * 64;
    let secret = mix(secret, mixer);
    let secret = prune(secret);

    let mixer = secret / 32;
    let secret = mix(secret, mixer);
    let secret = prune(secret);

    let mixer = secret * 2048;
    let secret = mix(secret, mixer);
    let secret = prune(secret);

    secret
}

fn secrets_iter(secret: u64) -> impl Iterator<Item = u64> {
    let mut secret = secret;

    std::iter::from_fn(move || {
        let next_secret = step_secret(secret);
        let previous_secret = secret;

        secret = next_secret;

        Some(previous_secret)
    })
}

fn prices(secret: u64) -> impl Iterator<Item = u64> {
    secrets_iter(secret).map(|s| price(s))
}

fn price_variations(prices: [u64; 5]) -> [i64; 4] {
    let prices = prices.map(|p| p as i64);

    [
        prices[1] - prices[0],
        prices[2] - prices[1],
        prices[3] - prices[2],
        prices[4] - prices[3],
    ]
}

fn rank_all_windows(mut prices: &[u64]) -> HashMap<[i64; 4], u64> {
    let mut windows = HashMap::new();

    while let [p1, p2, p3, p4, p5, ..] = prices {
        let variations = price_variations([*p1, *p2, *p3, *p4, *p5]);

        // Input the window into the list of prices, we only insert if there
        // is no value already in the map because the monkey is only
        // interested in the first time this window appears
        let _ = windows.entry(variations).or_insert(*p5);

        prices = &prices[1..];
    }

    windows
}

fn merge_windows(
    current_windows_gains: &mut HashMap<[i64; 4], u64>,
    monkey_window: HashMap<[i64; 4], u64>,
) {
    for (window, price) in monkey_window.into_iter() {
        let _ = current_windows_gains
            .entry(window)
            .and_modify(|current| *current += price)
            .or_insert(price);
    }
}

impl Puzzle for Day22 {
    type Output = u64;

    fn parse(input: &str) -> Option<Self> {
        Some(Day22 {
            secrets: input
                .lines()
                .map(|line| line.parse::<u64>().ok())
                .collect::<Option<Vec<_>>>()?,
        })
    }

    fn part1(self) -> Option<Self::Output> {
        Some(
            self.secrets
                .into_iter()
                .map(|secret| secrets_iter(secret).nth(2_000).unwrap())
                .sum(),
        )
    }

    fn part2(self) -> Option<Self::Output> {
        let mut bananas_per_window = HashMap::new();

        for secret in self.secrets.into_iter() {
            let prices = prices(secret)
                .take(
                    2_001, /* We generate 2001 prices because we need 2000 prices changes */
                )
                .collect::<Vec<_>>();

            let window = rank_all_windows(&prices);

            merge_windows(&mut bananas_per_window, window);
        }

        bananas_per_window.into_values().max()
    }
}
