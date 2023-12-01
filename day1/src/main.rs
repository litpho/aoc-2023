use anyhow::Result;

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

fn parse_input_one(input: &'static str) -> Result<Vec<u32>> {
    let input = parse(input, false);

    Ok(input)
}

fn parse_input_two(input: &'static str) -> Result<Vec<u32>> {
    let input = parse(input, true);

    Ok(input)
}

fn parse(input: &str, use_words: bool) -> Vec<u32> {
    input
        .lines()
        .map(|line| parse_line(line, use_words))
        .collect()
}

fn parse_line(line: &str, use_words: bool) -> u32 {
    let digits = (0..line.len())
        .filter_map(|i| match_to_char(&line[i..], use_words))
        .collect::<Vec<char>>();

    format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
        .parse::<u32>()
        .unwrap()
}

fn match_to_char(input: &str, use_words: bool) -> Option<char> {
    if let Some(first_char) = input.chars().next() {
        if first_char.is_ascii_digit() {
            return Some(first_char);
        }
    }

    if use_words {
        if let Some(c) = [
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .iter()
        .enumerate()
        .find(|(_, word)| input.starts_with(*word))
        .and_then(|(i, _)| char::from_digit(i as u32, 10))
        {
            return Some(c);
        }
    }

    None
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
