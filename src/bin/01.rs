use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse_lists(input);

    left.sort();
    right.sort();

    let sum = std::iter::zip(left, right)
        .map(|(l, r)| l.abs_diff(r))
        .sum::<u32>();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_lists(input);

    // Add left values as keys to a hashmap
    let mut map = HashMap::new();
    for l in left {
        map.insert(l, 0);
    }
    // If the right value is in the map, increment the value
    for r in right {
        if let Some(v) = map.get_mut(&r) {
            *v += 1;
        }
    }
    // Multiply the keys by their values and sum them
    let sum = map.iter().map(|(k, v)| k * v).sum::<u32>();
    Some(sum)
}

fn parse_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut items = line.split_ascii_whitespace();
        left.push(items.next().unwrap().parse::<u32>().unwrap());
        right.push(items.next().unwrap().parse::<u32>().unwrap());
    }

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert!(result.is_some());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        //assert the result is an integer
        assert!(result.is_some());
    }
}
