use crate::puzzle::Puzzle;

pub struct Day01 {
    elves: Vec<Vec<u64>>,
}

impl Puzzle for Day01 {
    type Output = u64;

    fn parse(input: &str) -> Option<Self> {
        Some(Day01 {
            elves: input
                .split("\n\n")
                .map(|block| {
                    block
                        .lines()
                        .map(|line| line.parse::<u64>().ok())
                        .collect::<Option<Vec<_>>>()
                })
                .collect::<Option<Vec<_>>>()?,
        })
    }

    fn part1(self) -> Option<Self::Output> {
        self.elves.into_iter().map(|elf| elf.into_iter().sum()).max()
    }

    fn part2(self) -> Option<Self::Output> {
        let mut calories_per_elf = self.elves.into_iter().map(|elf| elf.into_iter().sum()).collect::<Vec<u64>>();
        calories_per_elf.sort();

        Some(calories_per_elf.into_iter().rev().take(3).sum())
    }
}
