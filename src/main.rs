pub mod position;
pub mod puzzle;
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
}

use std::{
    fs::read_to_string,
    path::PathBuf,
};

use anyhow::anyhow;
use aoc_client::AocClient;

use clap::Parser as _;
use puzzle::Puzzle;

struct Client<'a> {
    aoc_client: Option<AocClient>,
    args: &'a Args,
}

impl<'a> Client<'a> {
    fn new(args: &Args) -> Client {
        Client {
            aoc_client: None,
            args,
        }
    }

    fn get_aoc_client(&mut self) -> anyhow::Result<&AocClient> {
        match self.aoc_client {
            None => {
                let mut client_builder = &mut AocClient::builder();
                client_builder = match &self.args.cookie_file {
                    Some(cookie) => client_builder.session_cookie_from_file(cookie)?,
                    None => client_builder.session_cookie_from_default_locations()?,
                };

                let client = client_builder
                    .year(self.args.year)?
                    .day(self.args.day)?
                    .build()?;

                self.aoc_client = Some(client);

                Ok(self.aoc_client.as_ref().unwrap())
            }
            Some(ref client) => Ok(&client),
        }
    }

    fn submit_answer(&mut self, puzzle_part: i64, answer: i64) -> anyhow::Result<()> {
        if !self.args.no_submit {
            let client = self.get_aoc_client()?;
            client.submit_answer_and_show_outcome(puzzle_part, answer)?;
        }

        Ok(())
    }

    fn get_input(&mut self) -> anyhow::Result<String> {
        match &self.args.input_file {
            None => {
                let client = self.get_aoc_client()?;
                Ok(client.get_input()?)
            }
            Some(path) => Ok(read_to_string(path)?),
        }
    }
}

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
        (2024, 07) => aoc24::day07::Day07::parse(input)?.part(part),
        (2024, 08) => aoc24::day08::Day08::parse(input)?.part(part),
        (2024, 09) => aoc24::day09::Day09::parse(input)?.part(part),
        (2024, 10) => aoc24::day10::Day10::parse(input)?.part(part),
        (2024, 11) => aoc24::day11::Day11::parse(input)?.part(part),
        _ => None,
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut client = Client::new(&args);

    let input = client.get_input()?;

    match puzzle(args.year, args.day, args.part, &input) {
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
