use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, pair, terminated},
    IResult,
};

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {took}");
    let input = result?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[Game]) -> u32 {
    input
        .iter()
        .filter(|game| game.max_red() <= 12 && game.max_green() <= 13 && game.max_blue() <= 14)
        .map(|game| game.id)
        .sum()
}

fn part_two(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|game| game.max_red() * game.max_green() * game.max_blue())
        .sum()
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    pub fn max_red(&self) -> u32 {
        self.rounds.iter().map(|r| r.red).max().unwrap_or_default()
    }

    pub fn max_green(&self) -> u32 {
        self.rounds
            .iter()
            .map(|r| r.green)
            .max()
            .unwrap_or_default()
    }

    pub fn max_blue(&self) -> u32 {
        self.rounds.iter().map(|r| r.blue).max().unwrap_or_default()
    }
}

#[derive(Debug, Default)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn parse(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Game> {
    map(
        pair(parse_id, separated_list1(tag("; "), parse_round)),
        |(id, rounds)| Game { id, rounds },
    )(input)
}

fn parse_id(input: &str) -> IResult<&str, u32> {
    delimited(tag("Game "), complete::u32, tag(": "))(input)
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    map(
        separated_list1(tag(", "), alt((parse_red, parse_blue, parse_green))),
        |v| {
            v.iter().fold(Round::default(), |mut round, cube| {
                match cube {
                    Cube::Red(c) => round.red += c,
                    Cube::Blue(c) => round.blue += c,
                    Cube::Green(c) => round.green += c,
                };
                round
            })
        },
    )(input)
}

fn parse_red(input: &str) -> IResult<&str, Cube> {
    map(terminated(complete::u32, tag(" red")), Cube::Red)(input)
}

fn parse_green(input: &str) -> IResult<&str, Cube> {
    map(terminated(complete::u32, tag(" green")), Cube::Green)(input)
}

fn parse_blue(input: &str) -> IResult<&str, Cube> {
    map(terminated(complete::u32, tag(" blue")), Cube::Blue)(input)
}

fn parse_input(input: &'static str) -> Result<Vec<Game>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(8, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(2377, part_one(&parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(2286, part_two(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(71220, part_two(&parse_input(DATA)?));

        Ok(())
    }
}
