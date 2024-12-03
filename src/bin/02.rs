advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let (safe_reports, _) = split_reports_by_safety(input);
    Some(safe_reports.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (safe_reports, unsafe_reports) = split_reports_by_safety(input);
    // For each unsafe report, find how many can be made safe by removing one value
    let mut count = safe_reports.len();
    for report in unsafe_reports {
        for i in 0..report.len() {
            let mut new_report = report.clone();
            new_report.remove(i);
            if is_safe(&new_report) {
                count += 1;
                break;
            }
        }
    }
    Some(count as u32)
}

fn is_safe(report: &Vec<u32>) -> bool {
    (report.windows(2).all(|w| w[0] < w[1]) || report.windows(2).all(|w| w[0] > w[1]))
        && report.windows(2).all(|w| w[0].abs_diff(w[1]) <= 3)
}

fn split_reports_by_safety(input: &str) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    // Parse the input into a vector of lines
    let reports = input.lines().collect::<Vec<&str>>();
    let mut safe_reports = vec![];
    let mut unsafe_reports = vec![];
    // Over each line, split by whitespace, and parse the elements as u32
    for line in reports {
        let report = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        // If all values are decreasing or increasing, and each adjacent pair has an absolute difference less than or equal to 3, add to safe_reports
        if is_safe(&report) {
            safe_reports.push(report);
        } else {
            unsafe_reports.push(report);
        }
    }
    (safe_reports, unsafe_reports)
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
        let result_one = part_one(&advent_of_code::template::read_file("examples", DAY));
        let result_two = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert!(result_one.is_some());
        assert!(result_two.is_some());
        // The result of part one should be less than or equal to the result of part two
        assert!(result_one.unwrap() <= result_two.unwrap());
    }
}
