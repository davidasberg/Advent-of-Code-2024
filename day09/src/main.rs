use core::fmt;
use std::ops::Div;

#[derive(Debug, Eq, PartialEq, Clone)]
enum BlockType {
    File,
    FreeSpace,
}

#[derive(Debug, Clone)]
struct Block {
    block_type: BlockType,
    id: Option<u32>,
}

struct File {
    blocks: Vec<Block>,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in &self.blocks {
            match block.block_type {
                BlockType::File => {
                    write!(f, "{}", block.id.expect("file has no id"))?;
                }
                BlockType::FreeSpace => {
                    write!(f, ".")?;
                }
            }
        }
        Ok(())
    }
}
fn get_input(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap()
}

fn parse_input(input: &str) -> File {
    let blocks = input
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let size = c.to_digit(10).unwrap() as usize;

            let block_type = if i % 2 == 0 {
                BlockType::File
            } else {
                BlockType::FreeSpace
            };

            let id = if block_type == BlockType::File {
                Some(i.div(2) as u32)
            } else {
                None
            };

            vec![Block { block_type, id }; size]
        })
        .collect();

    File { blocks }
}

fn move_blocks(blocks: &mut [Block]) {
    let mut first_empty_block = 0;
    let mut last_file_block = blocks.len() - 1;
    loop {
        for i in first_empty_block..blocks.len() {
            if blocks[i].block_type == BlockType::FreeSpace {
                first_empty_block = i;
                break;
            }
        }

        for i in (0..=last_file_block).rev() {
            if blocks[i].block_type == BlockType::File {
                last_file_block = i;
                break;
            }
        }

        if first_empty_block >= last_file_block {
            break;
        }

        blocks.swap(last_file_block, first_empty_block);
    }
}

fn find_last_block(blocks: &[Block]) -> Option<(usize, usize)> {
    for i in (0..blocks.len()).rev() {
        if blocks[i].block_type == BlockType::FreeSpace {
            continue;
        }

        let mut j = i;
        let id = blocks[i].id.expect("file has no id");
        loop {
            if blocks[j].block_type != BlockType::File || blocks[j].id != Some(id) || j == 0 {
                return Some((j + 1, i - j));
            }
            j -= 1
        }
    }

    None
}

fn find_first_empty_block_of_size(size: usize, blocks: &[Block]) -> Option<usize> {
    for i in 0..blocks.len() - size + 1 {
        if blocks[i..i + size]
            .iter()
            .all(|b| b.block_type == BlockType::FreeSpace)
        {
            return Some(i);
        }
    }

    None
}

fn swap_full_blocks(i: usize, j: usize, size: usize, blocks: &mut [Block]) {
    for k in 0..size {
        blocks.swap(i + k, j + k);
    }
}

fn move_full_blocks(blocks: &mut [Block]) {
    let len = blocks.len();
    let mut slice = &mut blocks[0..len];
    loop {
        if slice.len() == 1 {
            break;
        }

        let last_full_block_index = find_last_block(slice);

        let Some((last_full_block_index, last_full_block_size)) = last_full_block_index else {
            break;
        };

        let first_empty_block_index = find_first_empty_block_of_size(last_full_block_size, slice);
        let Some(first_empty_block_index) = first_empty_block_index else {
            slice = &mut slice[0..last_full_block_index];
            continue;
        };

        swap_full_blocks(
            first_empty_block_index,
            last_full_block_index,
            last_full_block_size,
            slice,
        );

        slice = &mut slice[0..last_full_block_index];
    }
}

fn calc_checksum(blocks: &[Block]) -> u64 {
    blocks
        .iter()
        .enumerate()
        .filter(|(_, b)| b.block_type == BlockType::File)
        .map(|(i, b)| b.id.expect("file has no id") as u64 * i as u64)
        .sum::<u64>()
}

fn part01() {
    let input = get_input("input.txt");
    let mut blocks = parse_input(&input);

    move_blocks(&mut blocks.blocks);

    let checksum = calc_checksum(&blocks.blocks);
    println!("{}", checksum);
}

fn part02() {
    let input = get_input("input.txt");
    let mut blocks = parse_input(&input);

    move_full_blocks(&mut blocks.blocks);

    let checksum = calc_checksum(&blocks.blocks);

    println!("{}", checksum);
}

fn main() {
    part01();
    part02();
}
