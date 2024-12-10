use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::{
        self,
        complete::{line_ending, space1},
    },
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(7);

#[derive(Debug)]
struct Equation {
    result: u64,
    operands: Vec<u64>,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}
impl Operation {
    fn all() -> Vec<Operation> {
        vec![Operation::Add, Operation::Multiply, Operation::Concatenate]
    }
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
            Operation::Concatenate => {
                let mut a_str = a.to_string();
                let b_str = b.to_string();
                a_str.push_str(&b_str);
                a_str.parse::<u64>().unwrap()
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, equations) = parse_equations(input).ok()?;
    let sum: u64 = equations
        .iter()
        .filter_map(|equation| {
            // There is one less operator than operands (it fits in between each operand!)
            let operator_count = equation.operands.len() - 1;
            (0..operator_count)
                .map(|_| vec![Operation::Add, Operation::Multiply])
                .multi_cartesian_product()
                .any(|ops| {
                    let mut ops_iter = ops.iter();
                    let result = equation
                        .operands
                        .iter()
                        .copied()
                        .reduce(|acc, next_number| ops_iter.next().unwrap().apply(acc, next_number))
                        .unwrap();
                    equation.result == result
                })
                .then_some(equation.result)
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, equations) = parse_equations(input).ok()?;
    let sum: u64 = equations
        .iter()
        .filter_map(|equation| {
            // There is one less operator than operands (it fits in between each operand!)
            let operator_count = equation.operands.len() - 1;
            (0..operator_count)
                .map(|_| Operation::all())
                .multi_cartesian_product()
                .any(|ops| {
                    let mut ops_iter = ops.iter();
                    let result = equation
                        .operands
                        .iter()
                        .copied()
                        .reduce(|acc, next_number| ops_iter.next().unwrap().apply(acc, next_number))
                        .unwrap();
                    equation.result == result
                })
                .then_some(equation.result)
        })
        .sum();
    Some(sum)
}

fn parse_equations(input: &str) -> IResult<&str, Vec<Equation>> {
    separated_list1(
        line_ending,
        map(parse_equation, |(result, operands)| Equation {
            result,
            operands,
        }),
    )(input)
}
fn parse_equation(input: &str) -> IResult<&str, (u64, Vec<u64>)> {
    // Each line has a result, followed by a colon, followed by a list of space-separated operands.
    separated_pair(character::complete::u64, tag(": "), parse_operands)(input)
}
fn parse_operands(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, character::complete::u64)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
