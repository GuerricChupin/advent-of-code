pub mod position;
pub mod puzzle;

mod aoc22 {
    pub mod day01;
}

mod aoc24 {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
    pub mod day07;
    pub mod day08;
    pub mod day09;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
    pub mod day17;
    pub mod day19;
    pub mod day18;
    pub mod day20;
    pub mod day21;
    pub mod day22;
    pub mod day23;
    pub mod day24;
    pub mod day25;
}

use std::{fs::read_to_string, path::PathBuf};

use anyhow::anyhow;
use aoc_client::AocClient;

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

    #[arg(short, long, default_value = ".cookie")]
    cookie_file: PathBuf,

    #[arg(short, long)]
    input_file: Option<PathBuf>,
}

struct Client<'a> {
    aoc_client: Option<AocClient>,
    args: &'a Args,
}

impl Client<'_> {
    fn new(args: &Args) -> Client {
        Client {
            aoc_client: None,
            args,
        }
    }

    fn with_aoc_client<T>(
        &mut self,
        run: impl FnOnce(&AocClient) -> anyhow::Result<T>,
    ) -> anyhow::Result<T> {
        match &self.aoc_client {
            None => {
                let client = AocClient::builder()
                    .session_cookie_from_file(&self.args.cookie_file)?
                    .year(self.args.year)?
                    .day(self.args.day)?
                    .build()?;

                let result = run(&client);

                self.aoc_client = Some(client);

                result
            }
            Some(client) => run(client),
        }
    }

    fn submit_answer(&mut self, puzzle_part: i64, answer: impl std::fmt::Display) -> anyhow::Result<()> {
        println!("Answer is {}", answer);

        if !self.args.no_submit {
            self.with_aoc_client(|client| {
                Ok(client.submit_answer_and_show_outcome(puzzle_part, answer)?)
            })?
        }

        Ok(())
    }

    fn get_input(&mut self) -> anyhow::Result<String> {
        match &self.args.input_file {
            None => self.with_aoc_client(|client| Ok(client.get_input()?)),
            Some(path) => Ok(read_to_string(path)?),
        }
    }
}

macro_rules! make_puzzle_runner {
    [ $( ($year:literal, $day:literal, $day_type:ty) ),* ] => {
        #[allow(clippy::zero_prefixed_literal)]
        fn puzzle_runner(year: i32, day: u32, part: i64, input: &str) -> Option<Box<dyn std::fmt::Display>> {
        $(
            if year == $year && day == $day {
                match part {
                    1 => return Some(Box::new(<$day_type>::parse(input)?.part1()?)),
                    2 => return Some(Box::new(<$day_type>::parse(input)?.part2()?)),
                    _ => return None
                }
            }
        )*

        None
    }

    };
}

// Defines the puzzle_runner function
make_puzzle_runner![
    (2022, 01, aoc22::day01::Day01),

    (2024, 01, aoc24::day01::Day01),
    (2024, 02, aoc24::day02::Day02),
    (2024, 03, aoc24::day03::Day03),
    (2024, 04, aoc24::day04::Day04),
    (2024, 05, aoc24::day05::Day05),
    (2024, 06, aoc24::day06::Day06),
    (2024, 07, aoc24::day07::Day07),
    (2024, 08, aoc24::day08::Day08),
    (2024, 09, aoc24::day09::Day09),
    (2024, 10, aoc24::day10::Day10),
    (2024, 11, aoc24::day11::Day11),
    (2024, 12, aoc24::day12::Day12),
    (2024, 13, aoc24::day13::Day13),
    (2024, 14, aoc24::day14::Day14),
    (2024, 15, aoc24::day15::Day15),
    (2024, 16, aoc24::day16::Day16),
    (2024, 17, aoc24::day17::Day17),
    (2024, 18, aoc24::day18::Day18),
    (2024, 19, aoc24::day19::Day19),
    (2024, 20, aoc24::day20::Day20),
    (2024, 21, aoc24::day21::Day21),
    (2024, 22, aoc24::day22::Day22),
    (2024, 23, aoc24::day23::Day23),
    (2024, 24, aoc24::day24::Day24),
    (2024, 25, aoc24::day25::Day25)
];

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut client = Client::new(&args);

    let input = client.get_input()?;

    match puzzle_runner(args.year, args.day, args.part, &input) {
        None => Err(anyhow!(
            "Not able to compute an answer for part {} of day {} of year {}",
            args.part,
            args.day,
            args.year
        )),
        Some(value) => {
            client.submit_answer(args.part, value)?;
            Ok(())
        }
    }
}
