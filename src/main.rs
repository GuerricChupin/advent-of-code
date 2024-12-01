use anyhow::anyhow;
use aoc_client::{AocClient, SubmissionOutcome};

use clap::Parser as _;

mod aoc24 {
    pub mod day01;
}

#[derive(clap::Parser)]
struct Args {
    #[arg(short, long)]
    year: i32,

    #[arg(short, long)]
    day: u32,

    #[arg(short, long)]
    part: i64,
}

fn answer(year: i32, day: u32, part: i64, input: &str) -> Option<i64> {
    match (year, day, part) {
        (2024, 01, 01) => aoc24::day01::part_1(input),
        (2024, 01, 02) => aoc24::day01::part_2(input),
        _ => None,
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(args.year)?
        .day(args.day)?
        .build()?;

    let input = client.get_input()?;

    match answer(args.year, args.day, args.part, &input) {
        None => Err(anyhow!(
            "Not able to compute an answer for part {} of day {} of year {}",
            args.part,
            args.day,
            args.year
        )),
        Some(value) => {
            let outcome = client.submit_answer(args.part, value)?;
            match outcome {
                SubmissionOutcome::Correct => Ok(()),
                SubmissionOutcome::Incorrect => Err(anyhow!("Incorrect answer")),
                SubmissionOutcome::Wait => Err(anyhow!("Timeout")),
                SubmissionOutcome::WrongLevel => Err(anyhow!("Wrong level")),
            }
        }
    }
}
