advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut items = line.split_ascii_whitespace();
        left.push(items.next().unwrap().parse::<u32>().unwrap());
        right.push(items.next().unwrap().parse::<u32>().unwrap());
    }

    left.sort();
    right.sort();

    let sum = std::iter::zip(left, right)
        .map(|(l, r)| l.abs_diff(r))
        .sum::<u32>();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_ne!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
