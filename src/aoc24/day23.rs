use std::collections::{BTreeSet, HashMap, HashSet};

use regex::Regex;

use crate::puzzle::Puzzle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Computer {
    name: [char; 2],
}

impl Computer {
    fn name(&self) -> String {
        format!("{}{}", self.name[0], self.name[1])
    }

    fn may_belong_to_the_chief_historian(self) -> bool {
        self.name[0] == 't'
    }
}

fn make_link_set(links: &[[Computer; 2]]) -> HashMap<Computer, BTreeSet<Computer>> {
    let mut set: HashMap<Computer, BTreeSet<Computer>> = HashMap::new();

    for &[left, right] in links {
        set.entry(left)
            .and_modify(|direct| {
                direct.insert(right);
            })
            .or_insert_with(|| BTreeSet::from([right]));
        set.entry(right)
            .and_modify(|direct| {
                direct.insert(left);
            })
            .or_insert_with(|| BTreeSet::from([left]));
    }

    set
}

fn can_insert_in_set(direct_links: &HashMap<Computer, BTreeSet<Computer>>, set: &BTreeSet<Computer>, computer: Computer) -> bool {
    !set.contains(&computer) /* The computer must no already be in the set */
    && set.iter().all(|other| direct_links.get(other).map(|direct| direct.contains(&computer)).unwrap_or(false)) /* All members of the set must be directly connected to that computer */
}

fn expand_sets(direct_links: &HashMap<Computer, BTreeSet<Computer>>, sets: &BTreeSet<BTreeSet<Computer>>) -> BTreeSet<BTreeSet<Computer>> {
    let mut new_sets = BTreeSet::new();

    for n_set in sets.iter() {
        let first_element = n_set.first().unwrap();

        let links = direct_links.get(first_element).unwrap();

        for &direct in links.into_iter() {
            if can_insert_in_set(direct_links, &n_set, direct) {
                let mut np1_set = n_set.clone();
                np1_set.insert(direct);
                new_sets.insert(np1_set);
            }
        }
    }

    new_sets
}

fn makes_sets_of_size(direct_links: &HashMap<Computer, BTreeSet<Computer>>, size: usize) -> BTreeSet<BTreeSet<Computer>> {
    match size {
        0 => BTreeSet::new(),
        1 => direct_links.keys().map(|computer| BTreeSet::from([*computer])).collect(),
        n => {
            let sets_of_nm1 = makes_sets_of_size(direct_links, n - 1);
            expand_sets(direct_links, &sets_of_nm1)
        }
    }
}

fn find_largest_connected_set(links: &HashMap<Computer, BTreeSet<Computer>>) -> BTreeSet<Computer> {
    let mut current_set = makes_sets_of_size(links, 1);

    loop {
        let next_set = expand_sets(links, &current_set);

        if next_set.is_empty() {
            return current_set.pop_first().unwrap()
        }

        current_set = next_set;
    }
}

pub struct Day23 {
    links: Vec<[Computer; 2]>,
}

impl Puzzle for Day23 {
    type Output = String;

    fn parse(input: &str) -> Option<Self> {
        let regex = Regex::new(r"(?<left>[a-z]{2})-(?<right>[a-z]{2})").unwrap();

        let links = input
            .lines()
            .map(|line| {
                regex.captures(line).and_then(|capture| {
                    let left = Computer {
                        name: [
                            capture["left"].chars().nth(0)?,
                            capture["left"].chars().nth(1)?,
                        ],
                    };
                    let right = Computer {
                        name: [
                            capture["right"].chars().nth(0)?,
                            capture["right"].chars().nth(1)?,
                        ],
                    };

                    Some([left, right])
                })
            })
            .collect::<Option<Vec<_>>>()?;

        Some(Day23 { links })
    }

    fn part1(self) -> Option<Self::Output> {
        let link_set = make_link_set(&self.links);

        let count = makes_sets_of_size(&link_set, 3)
            .into_iter()
            .filter(|set| {
                set.iter()
                    .any(|computer| computer.may_belong_to_the_chief_historian())
            })
            .count();

        Some(format!("{count}"))
    }

    fn part2(self) -> Option<Self::Output> {
        let link_set = make_link_set(&self.links);

        let largest_set = find_largest_connected_set(&link_set);

        let password = largest_set.into_iter().map(|computer| computer.name()).collect::<Vec<_>>().join(",");

        Some(password)
    }
}
