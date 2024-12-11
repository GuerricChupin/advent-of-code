pub trait Puzzle: Sized {
    fn parse(input: &str) -> Option<Self>;

    fn part1(self) -> Option<i64>;

    fn part2(self) -> Option<i64>;
}
