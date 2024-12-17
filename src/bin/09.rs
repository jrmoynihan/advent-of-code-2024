use std::{collections::VecDeque, fmt::Display, ops::Deref};

use itertools::Itertools;
use nom::{bytes::complete::take, combinator::map_res as map_result, multi::many1, IResult};

advent_of_code::solution!(9);

fn parse_digit(input: &str) -> IResult<&str, u32> {
    map_result(take(1usize), |digit: &str| digit.parse::<u32>())(input)
}

fn parse_digits(input: &str) -> IResult<&str, Vec<u32>> {
    many1(parse_digit)(input)
}

#[derive(Debug, Clone, Copy)]
struct DriveFileLayout {
    blocks_occupied: usize,
    id: Option<usize>,
    moved: bool,
}

pub fn part_one(input: &str) -> Option<usize> {
    let disk_map = input.trim();
    let files: VecDeque<DriveFileLayout> = disk_map
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let blocks_occupied = c.to_digit(10).unwrap() as usize;
            if i % 2 != 0 {
                DriveFileLayout {
                    blocks_occupied,
                    id: None,
                    moved: false,
                }
            } else {
                DriveFileLayout {
                    blocks_occupied,
                    id: Some(i / 2),
                    moved: false,
                }
            }
        })
        .collect();
    // dbg!(&numbers);
    let mut queue: VecDeque<usize> = files
        .iter()
        .flat_map(|file| std::iter::repeat(file.id).take(file.blocks_occupied))
        .flatten()
        .collect();
    // dbg!(&queue);
    let mut drive = vec![];
    for file in files.iter() {
        for _ in 0..file.blocks_occupied {
            let possible_id = if file.id.is_some() {
                queue.pop_front()
            } else {
                queue.pop_back()
            };
            if let Some(id) = possible_id {
                drive.push(id)
            }
        }
    }
    // dbg!(&drive);

    Some(drive.iter().enumerate().map(|(i, &id)| (i * id)).sum())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FileBlock {
    id: usize,
    length: usize,
}
impl FileBlock {
    fn to_string(&self) -> String {
        format!("{}", self.id)
    }
}

#[derive(Debug, Clone)]
struct DiskLayout {
    blocks: VecDeque<Option<FileBlock>>,
    max_id: usize,
}

impl DiskLayout {
    fn new(disk_map: &str) -> Self {
        let mut blocks = VecDeque::new();
        let mut max_id = 0;
        // Parse the disk map
        disk_map.chars().enumerate().peekable().for_each(|(i, c)| {
            let length = c.to_digit(10).unwrap() as usize;
            // If it's an even index, it's a file...
            if i % 2 == 0 {
                if c.is_digit(10) {
                    blocks.extend(
                        std::iter::repeat(Some(FileBlock { id: i / 2, length })).take(length),
                    );
                }
            // If it's an odd index, it's a free space...
            } else {
                blocks.extend(std::iter::repeat(None).take(length));
            }
            max_id = i / 2;
        });

        DiskLayout { blocks, max_id }
    }

    fn compact_files(&mut self) {
        for current_id in (0..=self.max_id).rev() {
            // println!("compacting file with id: {}", current_id);
            // Starting from the end of the disk, find the first file with the current id
            if let Some((file_index, file)) = self
                .blocks
                .iter()
                .enumerate()
                .rev()
                .find(|(_, block)| block.is_some() && block.as_ref().unwrap().id == current_id)
            {
                let space_required = file.as_ref().unwrap().length;

                // Return the positions of the first consecutive free spaces in the disk, iterating over chunks of size space_required
                let mut positions = Vec::<usize>::with_capacity(space_required);
                for (i, block) in self.blocks.iter().enumerate() {
                    if block.is_none() && i < file_index {
                        positions.push(i);
                        if positions.len() == space_required {
                            break;
                        }
                    } else {
                        positions.clear();
                    }
                }
                if positions.len() == space_required {
                    for position in positions {
                        let (file_block_index, _) = self
                            .blocks
                            .iter()
                            .enumerate()
                            .rev()
                            .find(|(_, block)| {
                                block.is_some() && block.as_ref().unwrap().id == current_id
                            })
                            .unwrap();
                        self.blocks.swap(position, file_block_index);
                    }
                }
            }
            // self.print_disk();
        }
    }

    fn calculate_checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .filter_map(|(pos, block)| block.as_ref().map(|file_block| pos * file_block.id))
            .sum()
    }
    fn print_disk(&self) {
        // Print the file block id or a "." if it's a free space
        println!(
            "{}",
            self.blocks
                .iter()
                .map(|block| block
                    .as_ref()
                    .map(|file_block| file_block.id.to_string())
                    .unwrap_or(".".to_string()))
                .join("")
        );
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let disk_map = input.trim();
    let mut disk = DiskLayout::new(disk_map);
    // println!("fragmented:");
    // disk.print_disk();
    disk.compact_files();
    // println!("compacted:");
    // disk.print_disk();
    let checksum = disk.calculate_checksum();
    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
