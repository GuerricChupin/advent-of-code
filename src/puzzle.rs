pub trait Puzzle: Sized {
    fn parse(input: &str) -> Option<Self>;

    fn part1(self) -> Option<i64>;

    fn part2(self) -> Option<i64>;

    fn part(self, part: i64) -> Option<i64> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => None,
        }
    }
}
