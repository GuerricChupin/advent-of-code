use crate::puzzle::Puzzle;

#[derive(Debug, Clone, Copy)]
struct Occupied {
    size: usize,
    id: i64,
}

#[derive(Debug, Clone, Copy)]
enum Block {
    Free(usize),
    Occupied(Occupied, bool),
}

fn fit_block_into(blocks: &[Block], block: Occupied) -> Vec<Block> {
    let mut new_blocks = Vec::with_capacity(blocks.len() + 1);
    let mut found_slot = false;

    for slot in blocks.iter() {
        match *slot {
            Block::Free(free_space) if !found_slot && free_space >= block.size => {
                let new_free_space = free_space - block.size;
                new_blocks.push(Block::Occupied(block, true));
                if new_free_space > 0 {
                    new_blocks.push(Block::Free(new_free_space))
                }

                found_slot = true;
            }
            slot @ (Block::Occupied(_, _) | Block::Free(_)) => {
                new_blocks.push(slot);
            }
        }
    }

    if !found_slot {
        new_blocks.push(Block::Occupied(block, true))
    }

    new_blocks
}

fn merge_free_space(blocks: &[Block]) -> Vec<Block> {
    let mut prefix_free_space: Option<usize> = None;

    let mut new_block = Vec::new();

    for slot in blocks {
        match (prefix_free_space, slot) {
            (Some(prefix_space), Block::Free(free_space)) => {
                prefix_free_space = Some(prefix_space + *free_space);
            }
            (None, Block::Free(free_space)) => {
                prefix_free_space = Some(*free_space);
            }
            (Some(prefix_space), Block::Occupied(occupied, pinned)) => {
                new_block.push(Block::Free(prefix_space));
                new_block.push(Block::Occupied(*occupied, *pinned));
                prefix_free_space = None;
            }
            (None, Block::Occupied(occupied, pinned)) => {
                new_block.push(Block::Occupied(*occupied, *pinned));
            }
        }
    }

    new_block
}

fn defragment_by_block(blocks: &[Block]) -> Vec<Block> {
    let mut blocks = Vec::from(blocks);

    let mut there_are_unpinned_blocks = true;

    while there_are_unpinned_blocks {
        there_are_unpinned_blocks = false;

        'search_unpinned: for block in blocks.iter_mut().rev() {
            match block {
                Block::Occupied(_, true) | Block::Free(_) => (),
                Block::Occupied(occupied, false) => {
                    let to_be_inserted = *occupied;
                    *block = Block::Free(occupied.size);
                    blocks = fit_block_into(&blocks, to_be_inserted);
                    there_are_unpinned_blocks = true;
                    break 'search_unpinned;
                }
            }
        }
    }

    merge_free_space(&blocks)
}

fn checksum(blocks: &[Block]) -> i64 {
    let mut current_index = 0;
    let mut checksum = 0;

    for block in blocks {
        match block {
            Block::Free(free_space) => {
                current_index += *free_space as i64;
            }
            Block::Occupied(Occupied { size, id }, _) => {
                for _sub_addr in 0..*size {
                    checksum += current_index * *id;
                    current_index += 1;
                }
            }
        }
    }

    checksum
}

fn to_block(disk_map: &[usize]) -> Vec<Block> {
    disk_map
        .iter()
        .enumerate()
        .filter_map(|(i, block_size)| {
            if i % 2 == 0 {
                Some(Block::Occupied(
                    Occupied {
                        size: *block_size,
                        id: (i / 2) as i64,
                    },
                    false,
                ))
            } else {
                Some(Block::Free(*block_size))
            }
        })
        .collect::<Vec<_>>()
}

fn flatten_blocks(blocks: Vec<Block>) -> Vec<Block> {
    blocks
        .into_iter()
        .map(|block| match block {
            Block::Free(free) => vec![Block::Free(free)],
            Block::Occupied(Occupied { size, id }, pinned) => {
                vec![Block::Occupied(Occupied { size: 1, id }, pinned); size]
            }
        })
        .flatten()
        .collect()
}

pub struct Day09 {
    disk_map: Vec<usize>,
}

impl Puzzle for Day09 {
    type Output = i64;
    
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
        let blocks = flatten_blocks(blocks);
        let defragmented_blocks = defragment_by_block(&blocks);
        Some(checksum(&defragmented_blocks))
    }

    fn part2(self) -> Option<i64> {
        let blocks = to_block(&self.disk_map);
        let defragmented_blocks = defragment_by_block(&blocks);
        Some(checksum(&defragmented_blocks))
    }
}
