advent_of_code::solution!(3);

use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, u32},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

pub fn part_one(input: &str) -> Option<u32> {
    let (_, instructions) = parse(input)
        .map_err(|e| miette!("parse failed! {}", e))
        .ok()?;
    dbg!(&instructions);
    let sum = instructions
        .iter()
        .map(|i| match i {
            Instruction::Mul(a, b) => a * b,
            Instruction::Add(a, b) => a + b,
        })
        .sum();
    Some(sum)
}

#[derive(Debug)]
enum Instruction {
    Mul(u32, u32),
    Add(u32, u32),
}

fn parse_mul_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(tag("("), separated_pair(u32, tag(","), u32), tag(")"))(input)?;
    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, parse_mul_instruction).map(|(_discard, instructions)| instructions))(
        input,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        // assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
