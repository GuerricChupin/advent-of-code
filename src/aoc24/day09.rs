use std::{collections::VecDeque, iter::repeat, num::NonZeroUsize};

use crate::puzzle::Puzzle;

#[derive(Debug, Clone, Copy)]
enum Block {
    Free(NonZeroUsize),
    Occupied(NonZeroUsize, i64),
}

fn defragment(disk: &mut Vec<i64>, mut blocks: VecDeque<Block>) {
    loop {
        let first_block = blocks.pop_front();
        let last_block = blocks.pop_back();

        match (first_block, last_block) {
            (None, Some(_)) => unreachable!(),
            (None, None) => return,
            (Some(Block::Occupied(block_size, block_id)), _) => {
                disk.extend(repeat(block_id).take(block_size.into()));

                if let Some(block) = last_block {
                    blocks.push_back(block);
                }
            }
            (_, Some(Block::Free(_))) | (Some(Block::Free(_)), None) => {
                if let Some(block) = first_block {
                    blocks.push_front(block);
                }
            }
            (Some(Block::Free(free_space)), Some(Block::Occupied(block_size, block_id))) => {
                let bytes_to_copy = free_space.min(block_size).into();

                disk.extend(repeat(block_id).take(bytes_to_copy));

                let new_free_space = NonZeroUsize::new(usize::from(free_space) - bytes_to_copy);
                let new_block_size = NonZeroUsize::new(usize::from(block_size) - bytes_to_copy);

                if let Some(new_free_space) = new_free_space {
                    blocks.push_front(Block::Free(new_free_space));
                }

                if let Some(new_block_size) = new_block_size {
                    blocks.push_back(Block::Occupied(new_block_size, block_id));
                }
            }
        }
    }
}

fn to_block(disk_map: &[usize]) -> VecDeque<Block> {
    disk_map
        .iter()
        .enumerate()
        .filter_map(|(i, block_size)| {
            let block_size = NonZeroUsize::new(*block_size)?;
            if i % 2 == 0 {
                Some(Block::Occupied(block_size, (i / 2) as i64))
            } else {
                Some(Block::Free(block_size))
            }
        })
        .collect::<VecDeque<_>>()
}

fn checksum(disk: &[i64]) -> i64 {
    disk.iter()
        .enumerate()
        .map(|(i, value)| i as i64 * value)
        .sum()
}

pub struct Day09 {
    disk_map: Vec<usize>,
}

impl Puzzle for Day09 {
    fn parse(input: &str) -> Option<Self> {
        Some(Day09 {
            disk_map: input
                .chars()
                .filter_map(|c| Some(c.to_digit(10)? as usize))
                .collect::<Vec<_>>(),
        })
    }

    fn part1(self) -> Option<i64> {
        let blocks = to_block(&self.disk_map);
        let mut disk = Vec::new();
        defragment(&mut disk, blocks);

        Some(checksum(&disk))
    }

    fn part2(self) -> Option<i64> {
        None
    }
}
