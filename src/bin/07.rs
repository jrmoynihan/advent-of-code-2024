use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit0, digit1},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};

advent_of_code::solution!(7);

#[derive(Debug)]
struct Equation {
    result: u32,
    operands: Vec<u32>,
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}
impl Operation {
    fn all() -> Vec<Operation> {
        vec![Operation::Add, Operation::Multiply]
    }
    fn apply(&self, a: u32, b: u32) -> u32 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, equations) = parse_equations(input).ok()?;
    for equation in &equations {
        println!("{:?}", equation);
    }
    let sum = equations
        .into_iter()
        .filter_map(|equation| {
            // Generate all possible operation combinations for n-1 operations
            let num_operations = (equation.operands.len() - 1) ^ 2;
            Operation::all()
                .iter()
                .combinations_with_replacement(num_operations)
                .find_map(|ops| {
                    // Start with first operand and apply operations sequentially
                    let result = ops
                        .iter()
                        .zip(equation.operands[1..].iter())
                        .fold(equation.operands[0], |acc, (op, operand)| {
                            op.apply(acc, *operand)
                        });

                    (result == equation.result).then_some(result)
                })
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}
fn parse_result(input: &str) -> IResult<&str, u32> {
    map_res(tuple((digit1, char(':'))), |(digits, _): (&str, char)| {
        digits.parse::<u32>()
    })(input)
}
fn parse_operands(input: &str) -> IResult<&str, Vec<u32>> {
    many1(map_res(digit1, |digits: &str| digits.parse::<u32>()))(input)
}

fn parse_equations(input: &str) -> IResult<&str, Vec<Equation>> {
    let mut equations = Vec::new();

    for line in input.lines() {
        let (_, equation) = parse_equation(line)?;
        println!("equation: {:?}", line);
        equations.push(equation);
    }
    Ok((input, equations))
}
fn parse_equation(input: &str) -> IResult<&str, Equation> {
    // Use nom to parse a line into an equation.  Each line has a result, followed by a colon, followed by a list of space-separated operands.
    let (input, result) = parse_result(input)?;
    let (input, operands) = parse_operands(input)?;
    Ok((input, Equation { result, operands }))
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
        assert_eq!(result, None);
    }
}
