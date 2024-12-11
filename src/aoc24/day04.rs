use crate::puzzle::Puzzle;

fn check_word_against_option<const N: usize>(
    input: &[Vec<u8>],
    option: [(usize, usize); N],
    possible_words: &[[u8; N]],
) -> bool {
    possible_words.iter().any(|word| {
        word.iter().zip(option).all(|(c, (i, j))| {
            input
                .get(i)
                .and_then(|line| line.get(j))
                .map(|k| k == c)
                .unwrap_or(false)
        })
    })
}

fn check_xmas(input: &[Vec<u8>], i: usize, j: usize) -> i64 {
    let horizontal = [(i, j), (i + 1, j), (i + 2, j), (i + 3, j)];
    let vertical = [(i, j), (i, j + 1), (i, j + 2), (i, j + 3)];
    let south_east_diagonal = [(i, j), (i + 1, j + 1), (i + 2, j + 2), (i + 3, j + 3)];
    let north_east_diagonal = [(i, j + 3), (i + 1, j + 2), (i + 2, j + 1), (i + 3, j)];

    let options = [
        horizontal,
        vertical,
        south_east_diagonal,
        north_east_diagonal,
    ];

    let words = [*b"XMAS", *b"SAMX"];

    options
        .into_iter()
        .filter(|&option| check_word_against_option(input, option, &words))
        .count() as i64
}

fn check_mas(input: &[Vec<u8>], i: usize, j: usize) -> bool {
    let south_east_diagonal = [(i, j), (i + 1, j + 1), (i + 2, j + 2)];
    let north_east_diagonal = [(i, j + 2), (i + 1, j + 1), (i + 2, j)];

    let options = [south_east_diagonal, north_east_diagonal];

    let words = [*b"MAS", *b"SAM"];

    options
        .into_iter()
        .all(|option| check_word_against_option(input, option, &words))
}

pub struct Day04 {
    input: Vec<Vec<u8>>,
}

impl Puzzle for Day04 {
    fn parse(input: &str) -> Option<Self> {
        Some(Day04 {
            input: input
                .lines()
                .map(|line| Vec::from(line.as_bytes()))
                .collect::<Vec<_>>(),
        })
    }

    fn part1(self) -> Option<i64> {
        let mut count = 0;

        for (j, line) in self.input.iter().enumerate() {
            for (i, _) in line.iter().enumerate() {
                count += check_xmas(&self.input, i, j)
            }
        }

        Some(count)
    }

    fn part2(self) -> Option<i64> {
        let mut count = 0;

        for (j, line) in self.input.iter().enumerate() {
            for (i, _) in line.iter().enumerate() {
                if check_mas(&self.input, i, j) {
                    count += 1
                }
            }
        }

        Some(count)
    }
}
