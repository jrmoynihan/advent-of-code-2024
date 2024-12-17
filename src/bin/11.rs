use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    shared_logic(input.to_string(), 25)
}

#[cached]
fn split(stone: usize) -> Vec<usize> {
    if stone == 0 {
        return vec![1];
    }
    let len = stone.ilog10() + 1;
    if len % 2 == 0 {
        let split = 10usize.pow(len / 2);
        vec![stone / split, stone % split]
    } else {
        vec![stone * 2024]
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    shared_logic(input.to_string(), 75)
}

fn shared_logic(input: String, blinks: usize) -> Option<usize> {
    let split_numbers = input.split_ascii_whitespace().collect_vec();
    let stones = split_numbers
        .iter()
        .filter_map(|&s| s.parse::<usize>().ok())
        .collect_vec();
    let mut current = stones
        .iter()
        .map(|&s| (s, 1))
        .collect::<HashMap<usize, usize>>();
    for _ in 0..blinks {
        let mut next: HashMap<usize, usize> = HashMap::new();
        for (stone, count) in current {
            for new_stone in split(stone) {
                next.entry(new_stone)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            }
        }
        current = next;
    }
    Some(current.values().sum::<usize>())
}

#[cfg(test)]
mod tests {
    // use std::error::Error;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(228651922369703));
    }
}
