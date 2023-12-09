use anyhow::Result;
use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {}", took);
    let mut input = result?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(&mut input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[Vec<i32>]) -> i32 {
    input.iter().map(|line| solve_line(line)).sum()
}

fn part_two(input: &mut [Vec<i32>]) -> i32 {
    input
        .iter_mut()
        .map(|line| {
            line.reverse();
            solve_line(line)
        })
        .sum()
}

fn solve_line(input: &[i32]) -> i32 {
    let solution = std::iter::successors(Some(input.to_vec()), |prev_line| {
        let next_line = prev_line
            .windows(2)
            .map(|x| x[1] - x[0])
            .collect::<Vec<i32>>();
        if next_line.iter().sum::<i32>() == 0 {
            None
        } else {
            Some(next_line)
        }
    })
    .collect::<Vec<Vec<i32>>>();

    solution
        .iter()
        .rev()
        .fold(0, |acc, x| x.last().unwrap() + acc)
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space1, complete::i32)(input)
}

fn parse_input(input: &'static str) -> Result<Vec<Vec<i32>>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(114, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(1853145119, part_one(&parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(2, part_two(&mut parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(923, part_two(&mut parse_input(DATA)?));

        Ok(())
    }
}
