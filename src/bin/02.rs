advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    // Parse the input into a vector of lines
    let reports = input.lines().collect::<Vec<&str>>();
    let mut safe_reports = vec![];
    // Over each line, split by whitespace, and parse the elements as u32
    for line in reports {
        let values = line.split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        // If all values are decreasing or increasing, and each adjacent pair has an absolute difference less than or equal to 3, add to safe_reports
        if (values.windows(2).all(|w| w[0] < w[1]) || values.windows(2).all(|w| w[0] > w[1])) && values.windows(2).all(|w| w[0].abs_diff(w[1]) <= 3) {
            safe_reports.push(values);
        }
    }
    Some(safe_reports.len() as u32)
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
        assert_eq!(result, result.is_some());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
