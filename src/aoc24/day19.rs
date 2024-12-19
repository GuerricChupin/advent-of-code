use std::collections::HashMap;

use crate::puzzle::Puzzle;

struct Trie {
    is_word: bool,
    next_letters: HashMap<Color, Trie>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            is_word: false,
            next_letters: HashMap::new(),
        }
    }

    fn add_pattern(&mut self, pattern: &[Color]) {
        match pattern {
            [] => self.is_word = true,
            [c, tl @ ..] => {
                self.next_letters
                    .entry(*c)
                    .or_insert_with(Trie::new)
                    .add_pattern(tl);
            }
        }
    }

    fn check_pattern(&self, pattern: &[Color]) -> i64 {
        let mut cache = HashMap::new();

        self.check_pattern_with_cache(pattern, &mut cache)
    }

    fn check_pattern_with_cache<'a>(
        &self,
        pattern: &'a [Color],
        cache: &mut HashMap<&'a [Color], i64>,
    ) -> i64 {
        match cache.get(pattern) {
            None => {
                let answer = self.check_sub_pattern(self, pattern, cache);
                cache.insert(pattern, answer);
                answer
            }
            Some(answer) => *answer,
        }
    }

    fn check_sub_pattern<'a>(
        &self,
        original: &Trie,
        pattern: &'a [Color],
        cache: &mut HashMap<&'a [Color], i64>,
    ) -> i64 {
        match pattern {
            [] => {
                if self.is_word {
                    1
                } else {
                    0
                }
            }
            [c, tl @ ..] => {
                let check_tail = self
                    .next_letters
                    .get(c)
                    .map(|trie| trie.check_sub_pattern(original, tl, cache))
                    .unwrap_or(0);

                let as_other_word_match = if self.is_word {
                    original.check_pattern_with_cache(pattern, cache)
                } else {
                    0
                };

                check_tail + as_other_word_match
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Color {
    fn from_char(c: char) -> Option<Color> {
        match c {
            'w' => Some(Color::White),
            'u' => Some(Color::Blue),
            'b' => Some(Color::Black),
            'r' => Some(Color::Red),
            'g' => Some(Color::Green),
            _ => None,
        }
    }
}

pub struct Day19 {
    towels: Vec<Vec<Color>>,

    patterns: Vec<Vec<Color>>,
}

impl Puzzle for Day19 {
    type Output = i64;

    fn parse(input: &str) -> Option<Self> {
        let (towels, patterns) = input.split_once("\n\n")?;

        let towels = towels
            .split(", ")
            .map(|towel| {
                towel
                    .chars()
                    .map(Color::from_char)
                    .collect::<Option<Vec<_>>>()
            })
            .collect::<Option<Vec<_>>>()?;

        let patterns = patterns
            .lines()
            .map(|pattern| {
                pattern
                    .chars()
                    .map(Color::from_char)
                    .collect::<Option<Vec<_>>>()
            })
            .collect::<Option<Vec<_>>>()?;

        Some(Day19 { towels, patterns })
    }

    fn part1(self) -> Option<Self::Output> {
        let mut trie = Trie::new();

        for towel in self.towels.into_iter() {
            trie.add_pattern(&towel);
        }

        Some(
            self.patterns
                .into_iter()
                .filter(|pattern| trie.check_pattern(&pattern) > 0)
                .count() as i64,
        )
    }

    fn part2(self) -> Option<Self::Output> {
        let mut trie = Trie::new();

        for towel in self.towels.into_iter() {
            trie.add_pattern(&towel);
        }

        let mut patterns = self.patterns;
        patterns.sort_by(|p1, p2| p1.len().cmp(&p2.len()));

        Some(
            patterns
                .into_iter()
                .map(|pattern| trie.check_pattern(&pattern))
                .sum(),
        )
    }
}
