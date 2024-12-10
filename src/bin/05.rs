#![feature(num_midpoint)]
use nom::multi::separated_list1;
use nom::{
    character::complete::{char, digit1, line_ending},
    combinator::map_res,
    multi::fold_many1,
    sequence::{separated_pair, terminated},
    IResult,
};
use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    if let Ok((_input, (rules, updates))) =
        separated_pair(parse_rules, line_ending, parse_updates)(input)
    {
        let middle_page_sum = updates
            .iter()
            .map(|update| {
                let is_valid = update.iter().enumerate().all(|(i, &page)| {
                    // If there's no rule for this page, it's valid
                    match rules.get(&page) {
                        None => true,
                        Some(afters) => {
                            // Check if all subsequent pages follow the rule
                            update[i + 1..]
                                .iter()
                                .all(|next_page| afters.contains(next_page))
                                && !update[..i].iter().any(|p| afters.contains(p))
                        }
                    }
                });

                if is_valid {
                    #[cfg(debug_assertions)]
                    dbg!(&update);
                    update[0usize.midpoint(update.len())]
                } else {
                    0
                }
            })
            .sum();
        Some(middle_page_sum)
    } else {
        None
    }
}

fn parse_rules(input: &str) -> IResult<&str, HashMap<u32, Vec<u32>>> {
    fold_many1(
        terminated(parse_rule, line_ending),
        HashMap::default,
        |mut map, (page, after)| {
            map.entry(page)
                .and_modify(|afters: &mut Vec<u32>| afters.push(after))
                .or_insert(vec![after]);
            map
        },
    )(input)
}
fn parse_updates(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(line_ending, parse_update)(input)
}
fn parse_update(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(char(','), parse_u32)(input)
}
fn parse_rule(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(parse_u32, char('|'), parse_u32)(input)
}
fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    if let Ok((_input, (rules, updates))) =
        separated_pair(parse_rules, line_ending, parse_updates)(input)
    {
        let middle_page_sum = updates
            .iter()
            .inspect(|&update| {
                #[cfg(debug_assertions)]
                dbg!("Checking update:", update);
            })
            .filter(|update| {
                let invalid = !is_valid_order(update, &rules);
                #[cfg(debug_assertions)]
                if invalid {
                    dbg!("Invalid update:", update);
                }
                invalid
            })
            .map(|update| {
                let mut pages = update.clone();
                let mut attempts = 0;
                const MAX_ATTEMPTS: usize = 1000;

                while !is_valid_order(&pages, &rules) {
                    attempts += 1;
                    if attempts > MAX_ATTEMPTS {
                        #[cfg(debug_assertions)]
                        println!("Failed to find valid order for: {:?}", update);
                        return 0;
                    }

                    let mut made_swap = false;

                    // Forward pass - check each position with the next
                    for i in 0..pages.len() - 1 {
                        if should_swap(&pages, i, i + 1, &rules) {
                            pages.swap(i, i + 1);
                            made_swap = true;
                            break;
                        }
                    }

                    // If no forward swaps worked, try backward pass
                    if !made_swap {
                        for i in (1..pages.len()).rev() {
                            if should_swap(&pages, i - 1, i, &rules) {
                                pages.swap(i - 1, i);
                                made_swap = true;
                                break;
                            }
                        }
                    }

                    if !made_swap {
                        #[cfg(debug_assertions)]
                        println!("Stuck - no valid swaps possible for: {:?}", update);
                        return 0;
                    }
                }

                #[cfg(debug_assertions)]
                dbg!("Corrected to:", &pages);

                pages[0usize.midpoint(pages.len())]
            })
            .sum();
        Some(middle_page_sum)
    } else {
        None
    }
}

// Helper function to determine if two adjacent positions should be swapped
fn should_swap(pages: &[u32], i: usize, j: usize, rules: &HashMap<u32, Vec<u32>>) -> bool {
    let page_i = pages[i];
    let page_j = pages[j];

    // Check if i must come after j (meaning we should swap)
    if let Some(rules_j) = rules.get(&page_j) {
        if rules_j.contains(&page_i) {
            return true;
        }
    }

    // Check if j must come after i (meaning we shouldn't swap)
    if let Some(rules_i) = rules.get(&page_i) {
        if rules_i.contains(&page_j) {
            return false;
        }
    }

    // If no direct rules, check transitive relationships
    // Look at all rules that mention either i or j
    for (page, must_come_after) in rules {
        if must_come_after.contains(&page_i) && *page == page_j {
            return false; // j must come before i
        }
        if must_come_after.contains(&page_j) && *page == page_i {
            return true; // i must come before j
        }
    }

    // If no rules dictate order, maintain current position
    false
}

// Add this helper function
fn is_valid_order(update: &[u32], rules: &HashMap<u32, Vec<u32>>) -> bool {
    update
        .iter()
        .enumerate()
        .all(|(i, &page)| match rules.get(&page) {
            None => true,
            Some(afters) => {
                update[i + 1..]
                    .iter()
                    .all(|next_page| afters.contains(next_page))
                    && !update[..i].iter().any(|p| afters.contains(p))
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rules() {
        let input = "12|45\n45|78\n12|34\n";
        let (input, rules) = parse_rules(input).unwrap();
        dbg!(&input, &rules);
        assert_eq!(rules, HashMap::from([(12, vec![45, 34]), (45, vec![78])]));
    }
    #[test]
    fn test_parse_updates() {
        let input = "12,45\n45,78\n12,34\n";
        let (input, updates) = parse_updates(input).unwrap();
        dbg!(&input, &updates);
        assert_eq!(updates, vec![vec![12, 45], vec![45, 78], vec![12, 34]]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        dbg!(&result);
        assert_eq!(result, Some(123));
    }
}
