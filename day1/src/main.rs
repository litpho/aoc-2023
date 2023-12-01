use anyhow::Result;
use nom::{
    character::complete::{alpha0, line_ending, one_of},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::delimited,
    IResult,
};

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input_one(DATA));
    println!("Time spent parsing: {}", took);
    let input = result?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| parse_input_two(DATA));
    println!("Time spent parsing: {}", took);
    let input = result?;

    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[u32]) -> u32 {
    input.iter().sum()
}

fn part_two(input: &[u32]) -> u32 {
    input.iter().sum()
}

fn parse_one(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, u32> {
    map_res(parse_digits, |v| {
        let a = v.first().unwrap();
        let b = v.last().unwrap();
        format!("{a}{b}").parse::<u32>()
    })(input)
}

fn parse_digits(input: &str) -> IResult<&str, Vec<char>> {
    many1(delimited(alpha0, parse_single_digit, alpha0))(input)
}

fn parse_single_digit(input: &str) -> IResult<&str, char> {
    one_of("1234567890")(input)
}

fn parse_input_one(input: &'static str) -> Result<Vec<u32>> {
    let (_, input) = parse_one(input)?;

    Ok(input)
}

fn simple_parse(input: &str) -> Vec<u32> {
    input.lines().map(simple_parse_line).collect()
}

fn simple_parse_line(input: &str) -> u32 {
    let digits = (0..input.len())
        .filter_map(|i| match_to_char(&input[i..input.len()]))
        .collect::<Vec<char>>();

    format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
        .parse::<u32>()
        .unwrap()
}

fn match_to_char(input: &str) -> Option<char> {
    if input.starts_with('1') || input.starts_with("one") {
        return Some('1');
    }
    if input.starts_with('2') || input.starts_with("two") {
        return Some('2');
    }
    if input.starts_with('3') || input.starts_with("three") {
        return Some('3');
    }
    if input.starts_with('4') || input.starts_with("four") {
        return Some('4');
    }
    if input.starts_with('5') || input.starts_with("five") {
        return Some('5');
    }
    if input.starts_with('6') || input.starts_with("six") {
        return Some('6');
    }
    if input.starts_with('7') || input.starts_with("seven") {
        return Some('7');
    }
    if input.starts_with('8') || input.starts_with("eight") {
        return Some('8');
    }
    if input.starts_with('9') || input.starts_with("nine") {
        return Some('9');
    }
    if input.starts_with('0') {
        return Some('0');
    }

    None
}

fn parse_input_two(input: &'static str) -> Result<Vec<u32>> {
    let input = simple_parse(input);

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");
    const TESTDATA2: &str = include_str!("test2.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(142, part_one(&parse_input_one(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(55712, part_one(&parse_input_one(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(281, part_two(&parse_input_two(TESTDATA2)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(55413, part_two(&parse_input_two(DATA)?));

        Ok(())
    }
}
