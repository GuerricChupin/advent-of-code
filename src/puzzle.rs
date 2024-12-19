use std::fmt::Display;

pub trait Puzzle: Sized {
    type Output: Display; 
    
    fn parse(input: &str) -> Option<Self>;

    fn part1(self) -> Option<Self::Output>;

    fn part2(self) -> Option<Self::Output>;
}
