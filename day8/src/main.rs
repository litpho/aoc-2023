// use std::collections::HashMap;
use ahash::{HashMap, HashMapExt};

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, line_ending},
    combinator::{map, value},
    multi::{many1, separated_list1},
    sequence::{delimited, pair, separated_pair},
    IResult,
};
use num::integer::lcm;

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {took}");
    let Input { directions, map } = result?;

    let (took, result) = took::took(|| part_one(&directions, &map));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(&directions, &map));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(directions: &[Direction], map: &HashMap<&str, [&str; 2]>) -> u64 {
    counting_steps(directions, map, "AAA", |location| location == "ZZZ")
}

fn part_two(directions: &[Direction], map: &HashMap<&str, [&str; 2]>) -> u64 {
    let steps = map
        .keys()
        .filter_map(|k| if k.ends_with('A') { Some(*k) } else { None })
        .map(|location| counting_steps(directions, map, location, |loc| loc.ends_with('Z')))
        .collect::<Vec<u64>>();

    steps
        .iter()
        .skip(1)
        .fold(*steps.first().unwrap(), |acc, next| lcm(acc, *next))
}

fn counting_steps(
    directions: &[Direction],
    map: &HashMap<&str, [&str; 2]>,
    start_location: &str,
    check_fn: fn(&str) -> bool,
) -> u64 {
    let mut counter = 0;
    let mut location = start_location;
    loop {
        for direction in directions.iter().cycle() {
            counter += 1;
            location = match direction {
                Direction::Left => map.get(location).unwrap()[0],
                Direction::Right => map.get(location).unwrap()[1],
            };

            if check_fn(location) {
                return counter;
            }
        }
    }
}

#[derive(Debug)]
struct Input<'a> {
    directions: Vec<Direction>,
    map: HashMap<&'a str, [&'a str; 2]>,
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

fn parse(input: &str) -> IResult<&str, Input> {
    let (input, directions) = parse_directions(input)?;
    let (input, _) = pair(line_ending, line_ending)(input)?;
    let (input, lines) = separated_list1(line_ending, parse_line)(input)?;
    let mut map = HashMap::new();
    for (key, values) in lines {
        map.insert(key, values);
    }

    Ok((input, Input { directions, map }))
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(alt((parse_left, parse_right)))(input)
}

fn parse_left(input: &str) -> IResult<&str, Direction> {
    value(Direction::Left, complete::char('L'))(input)
}

fn parse_right(input: &str) -> IResult<&str, Direction> {
    value(Direction::Right, complete::char('R'))(input)
}

fn parse_line(input: &str) -> IResult<&str, (&str, [&str; 2])> {
    separated_pair(alphanumeric1, tag(" = "), parse_destinations)(input)
}

fn parse_destinations(input: &str) -> IResult<&str, [&str; 2]> {
    map(
        delimited(
            complete::char('('),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            complete::char(')'),
        ),
        |(a, b)| [a, b],
    )(input)
}

fn parse_input(input: &'static str) -> Result<Input> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");
    const TESTDATA2: &str = include_str!("test2.txt");
    const TESTDATA3: &str = include_str!("test3.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        let Input { directions, map } = parse_input(TESTDATA)?;

        assert_eq!(2, part_one(&directions, &map));

        Ok(())
    }

    #[test]
    fn test_part_one_testdata2() -> Result<()> {
        let Input { directions, map } = parse_input(TESTDATA2)?;

        assert_eq!(6, part_one(&directions, &map));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let Input { directions, map } = parse_input(DATA)?;

        assert_eq!(16531, part_one(&directions, &map));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        let Input { directions, map } = parse_input(TESTDATA3)?;

        assert_eq!(6, part_two(&directions, &map));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let Input { directions, map } = parse_input(DATA)?;

        assert_eq!(24035773251517, part_two(&directions, &map));

        Ok(())
    }
}
