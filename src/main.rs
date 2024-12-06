pub mod puzzle;
mod aoc24 {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
}

use std::{fs::read_to_string, path::PathBuf};

use anyhow::anyhow;
use aoc_client::{AocClient, SubmissionOutcome};

use clap::Parser as _;
use puzzle::Puzzle;

#[derive(clap::Parser)]
struct Args {
    #[arg(short, long)]
    year: i32,

    #[arg(short, long)]
    day: u32,

    #[arg(short, long)]
    part: i64,

    #[arg(short, long, default_value = "false")]
    no_submit: bool,

    #[arg(short, long)]
    cookie_file: Option<PathBuf>,

    #[arg(short, long)]
    input_file: Option<PathBuf>,
}

fn puzzle(year: i32, day: u32, part: i64, input: &str) -> Option<i64> {
    match (year, day) {
        (2024, 01) => aoc24::day01::Day01::parse(input)?.part(part),
        (2024, 02) => aoc24::day02::Day02::parse(input)?.part(part),
        (2024, 03) => aoc24::day03::Day03::parse(input)?.part(part),
        (2024, 04) => aoc24::day04::Day04::parse(input)?.part(part),
        (2024, 05) => aoc24::day05::Day05::parse(input)?.part(part),
        (2024, 06) => aoc24::day06::Day06::parse(input)?.part(part),
        _ => None,
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let client = {
        let mut client_builder = &mut AocClient::builder();
        client_builder = match args.cookie_file {
            Some(cookie) => client_builder.session_cookie_from_file(cookie)?,
            None => client_builder.session_cookie_from_default_locations()?,
        };
        client_builder.year(args.year)?.day(args.day)?.build()?
    };

    let input = match args.input_file {
        None => client.get_input()?,
        Some(path) => read_to_string(path)?,
    };

    match puzzle(args.year, args.day, args.part, &input) {
        None => Err(anyhow!(
            "Not able to compute an answer for part {} of day {} of year {}",
            args.part,
            args.day,
            args.year
        )),
        Some(value) => {
            println!("Answer is {value}");
            if !args.no_submit {
                let outcome = client.submit_answer(args.part, value)?;
                match outcome {
                    SubmissionOutcome::Correct => Ok(()),
                    SubmissionOutcome::Incorrect => Err(anyhow!("Incorrect answer")),
                    SubmissionOutcome::Wait => Err(anyhow!("Timeout")),
                    SubmissionOutcome::WrongLevel => Err(anyhow!("Wrong level")),
                }
            } else {
                println!("Not submitting");
                Ok(())
            }
        }
    }
}
