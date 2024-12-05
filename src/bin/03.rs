advent_of_code::solution!(3);

use miette::miette;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, u32},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Mul(u32, u32),
    // Add(u32, u32),
    Do,
    Dont,
}
#[derive(PartialEq, Eq)]
enum ShouldProcess {
    Do,
    Dont,
}

fn parse_mul_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(tag("("), separated_pair(u32, tag(","), u32), tag(")"))(input)?;
    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn parser_rules(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        parse_mul_instruction,
    ))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, parser_rules).map(|(_discard, instructions)| instructions))(input)
}

fn execute(instructions: &[Instruction]) -> Option<u32> {
    let (_, sum) = instructions.iter().fold(
        (ShouldProcess::Do, 0),
        |(should_process, sum), instruction| match instruction {
            Instruction::Mul(a, b) => {
                if should_process == ShouldProcess::Do {
                    (should_process, sum + a * b)
                } else {
                    (should_process, sum)
                }
            }
            // Instruction::Add(a, b) => {
            //     if should_process == ShouldProcess::Do {
            //         (should_process, sum + a + b)
            //     } else {
            //         (should_process, sum)
            //     }
            // }
            Instruction::Do => (ShouldProcess::Do, sum),
            Instruction::Dont => (ShouldProcess::Dont, sum),
        },
    );
    Some(sum)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, instructions) = parse(input)
        .map_err(|e| miette!("parse failed! {}", e))
        .ok()?;
    execute(&instructions)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, instructions) = parse(input)
        .map_err(|e| miette!("parse failed! {}", e))
        .ok()?;
    execute(&instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let example = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(example, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
