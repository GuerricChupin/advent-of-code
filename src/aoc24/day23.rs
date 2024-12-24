use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use regex::Regex;

use crate::puzzle::Puzzle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Computer {
    name: [char; 2],
}

impl Computer {
    fn may_belong_to_the_chief_historian(self) -> bool {
        self.name[0] == 't'
    }
}

#[derive(Clone, Copy)]
enum Parent<'a> {
    None { size: usize },
    Some { parent: &'a EquivalenceClass<'a> },
}

struct EquivalenceClass<'a> {
    computer: Computer,
    parent: RefCell<Parent<'a>>,
}

impl<'a> PartialEq for EquivalenceClass<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.computer == other.computer
    }
}

impl<'a> Eq for EquivalenceClass<'a> {}

impl<'a> EquivalenceClass<'a> {
    fn new(computer: Computer) -> Self {
        EquivalenceClass {
            computer,
            parent: RefCell::new(Parent::None { size: 1 }),
        }
    }

    fn find(&'a self) -> (&'a EquivalenceClass<'a>, usize) {
        let parent = self.parent.borrow().clone();

        match parent {
            Parent::None { size } => (self, size),
            Parent::Some { parent } => {
                let (new_parent, size) = parent.find();
                *self.parent.borrow_mut() = Parent::Some { parent: new_parent };
                (new_parent, size)
            }
        }
    }

    fn union(&'a self, other: &'a EquivalenceClass<'a>) {
        let (self_repr, self_size) = self.find();
        let (other_repr, other_size) = other.find();

        if self_repr != other_repr {
            let (smallest, biggest) = if self_size < other_size {
                (self_repr, other_repr)
            } else {
                (other_repr, self_repr)
            };

            *smallest.parent.borrow_mut() = Parent::Some { parent: biggest };
            *biggest.parent.borrow_mut() = Parent::None {
                size: self_size + other_size,
            };
        }
    }
}

fn make_equivalences(links: &[[Computer; 2]]) -> Vec<HashSet<Computer>> {
    let classes = {
        let mut classes = HashMap::new();

        for [left, right] in links {
            classes
                .entry(*left)
                .or_insert_with(|| EquivalenceClass::new(*left));
            classes
                .entry(*right)
                .or_insert_with(|| EquivalenceClass::new(*right));
        }

        classes
    };

    for [left, right] in links {
        let left_repr = classes.get(left).unwrap();
        let right_repr = classes.get(right).unwrap();

        left_repr.union(&right_repr);
    }

    let mut bag_map: HashMap<Computer, HashSet<Computer>> = HashMap::new();

    for (computer, class) in classes.iter() {
        let (parent, _) = class.find();

        bag_map
            .entry(parent.computer)
            .and_modify(|set| {
                let _ = set.insert(*computer);
            })
            .or_insert_with(|| HashSet::from([*computer]));
    }

    bag_map
        .into_values()
        .map(|set| {
            println!("{}", set.len());
            set
        })
        .collect()
}

pub struct Day23 {
    links: Vec<[Computer; 2]>,
}

impl Puzzle for Day23 {
    type Output = i64;

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
        let classes = make_equivalences(&self.links);
        println!("{:?}", classes);

        Some(
            classes
                .into_iter()
                .filter(|computer_set| computer_set.len() == 3)
                .filter(|computer_set| {
                    computer_set
                        .iter()
                        .any(|computer| computer.may_belong_to_the_chief_historian())
                })
                .count() as i64,
        )
    }

    fn part2(self) -> Option<Self::Output> {
        None
    }
}
