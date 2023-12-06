use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    combinator::eof,
    multi::separated_list1,
    sequence::{delimited, pair},
    IResult,
};

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {}", took);
    let input = result?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[Race]) -> u64 {
    input.iter().map(solve_race).product()
}

fn part_two(input: &[Race]) -> u64 {
    let race = Race::merge(input);

    solve_race(&race)
}

fn solve_race(race: &Race) -> u64 {
    (1..race.duration - 1)
        .filter(|x| x * (race.duration - x) > race.distance)
        .count() as u64
}

#[derive(Debug)]
struct Race {
    duration: u64,
    distance: u64,
}

impl Race {
    pub fn merge(races: &[Race]) -> Self {
        let mut durations = vec![];
        let mut distances = vec![];
        for race in races {
            durations.push(race.duration);
            distances.push(race.distance);
        }

        Self {
            duration: durations
                .iter()
                .map(|v| v.to_string())
                .collect::<String>()
                .parse()
                .unwrap(),
            distance: distances
                .iter()
                .map(|v| v.to_string())
                .collect::<String>()
                .parse()
                .unwrap(),
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, durations) = parse_line(input, "Time:")?;
    let (input, distances) = parse_line(input, "Distance:")?;

    let races = durations
        .iter()
        .zip(distances)
        .map(|(duration, distance)| Race {
            duration: *duration,
            distance,
        })
        .collect();

    Ok((input, races))
}

fn parse_line<'a>(input: &'a str, label: &str) -> IResult<&'a str, Vec<u64>> {
    delimited(
        pair(tag(label), space1),
        separated_list1(space1, complete::u64),
        alt((line_ending, eof)),
    )(input)
}

fn parse_input(input: &'static str) -> Result<Vec<Race>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(288, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(227850, part_one(&parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(30, part_two(&parse_input(TESTDATA)?));

        Ok(())
    }

    // #[test]
    // fn test_part_two() -> Result<()> {
    //     assert_eq!(5329815, part_two(&parse_input(DATA)?));
    //
    //     Ok(())
    // }
}
