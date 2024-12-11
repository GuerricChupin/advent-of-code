use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::puzzle::Puzzle;

#[derive(Debug)]
struct Rule {
    before: i64,
    after: i64,
}

struct RulesGraph {
    graph: HashMap<i64, HashSet<i64>>,
}

impl RulesGraph {
    fn new(rules: &[Rule]) -> RulesGraph {
        let mut graph = HashMap::new();

        for Rule { before, after } in rules {
            let direct_children = graph.entry(*before).or_insert_with(HashSet::new);
            direct_children.insert(*after);
        }

        RulesGraph { graph }
    }

    fn is_in_correct_order(&self, before: i64, after: i64) -> bool {
        // "before" and "after" are in a correct order if after is not a direct
        // child of before (transitive "childness" is ok, apparently)
        self.graph
            .get(&after)
            .map(|children| !children.contains(&before))
            .unwrap_or(true)
    }

    fn check_page_tail(&self, head: i64, tail: &[i64]) -> bool {
        tail.iter()
            .all(|after| self.is_in_correct_order(head, *after))
    }

    fn correct_page(&self, page: &[i64]) -> bool {
        match page.split_first() {
            Some((head, tail)) => self.check_page_tail(*head, tail) && self.correct_page(tail),
            None => true,
        }
    }

    fn reorder(&self, page: &[i64]) -> Vec<i64> {
        let mut scored_page = page.iter().map(|&value| {
            let score = page
                .iter()
                .filter(|&&other| other != value)
                .map(|other| {
                    if self.is_in_correct_order(value, *other) {
                        0
                    } else {
                        1
                    }
                })
                .sum::<i32>();

            (value, score)
        }).collect::<Vec<_>>();

        scored_page.sort_by(|(_, s1), (_, s2)| s1.cmp(s2));

        scored_page.into_iter().map(|(value, _)| value).collect::<Vec<_>>()
    }
}

#[derive(Debug)]
pub struct Day05 {
    rules: Vec<Rule>,
    pages: Vec<Vec<i64>>,
}

impl Puzzle for Day05 {
    fn parse(input: &str) -> Option<Self> {
        let (rules_input, pages_input) = input.split_once("\n\n")?;
        let rule_regex = Regex::new(r"(\d+)\|(\d+)").unwrap();

        let rules = rules_input
            .lines()
            .map(|line| {
                rule_regex.captures(line).and_then(|capture| {
                    Some(Rule {
                        before: capture[1].parse::<i64>().ok()?,
                        after: capture[2].parse::<i64>().ok()?,
                    })
                })
            })
            .collect::<Option<Vec<_>>>()?;

        let pages = pages_input
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|page| page.parse::<i64>().ok())
                    .collect::<Option<Vec<_>>>()
            })
            .collect::<Option<Vec<_>>>()?;

        Some(Day05 { rules, pages })
    }

    fn part1(self) -> Option<i64> {
        let graph = RulesGraph::new(&self.rules);

        Some(
            self.pages
                .into_iter()
                .filter(|page| graph.correct_page(page))
                .map(|page| page[page.len() / 2])
                .sum(),
        )
    }

    fn part2(self) -> Option<i64> {
        let graph = RulesGraph::new(&self.rules);

        Some(
            self.pages
                .into_iter()
                .filter(|page| !graph.correct_page(page))
                .map(|page| graph.reorder(&page))
                .map(|page| page[page.len() / 2])
                .sum(),
        )
    }
}
