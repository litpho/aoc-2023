use anyhow::Result;

const DATA: &str = include_str!("input.txt");
const DIGITS_AS_WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let (took, result) = took::took(|| parse_input_one(DATA));
    println!("Time spent parsing: {took}");
    let input = result;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| parse_input_two(DATA));
    println!("Time spent parsing: {took}");
    let input = result;

    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");
}

fn part_one(input: &[u32]) -> u32 {
    input.iter().sum()
}

fn part_two(input: &[u32]) -> u32 {
    input.iter().sum()
}

fn parse_input_one(input: &'static str) -> Vec<u32> {
    parse(input, false)
}

fn parse_input_two(input: &'static str) -> Vec<u32> {
    parse(input, true)
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
        .collect::<Vec<u32>>();

    digits.first().unwrap() * 10 + digits.last().unwrap()
}

fn match_to_char(input: &str, use_words: bool) -> Option<u32> {
    if let Some(first_char) = input.chars().next() {
        if let Some(digit) = first_char.to_digit(10) {
            return Some(digit);
        }
    }

    if use_words {
        if let Some(digit) = DIGITS_AS_WORDS
            .iter()
            .enumerate()
            .find(|(_, word)| input.starts_with(*word))
            .map(|(i, _)| i as u32)
        {
            return Some(digit);
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
    fn test_part_one_testdata() {
        assert_eq!(142, part_one(&parse_input_one(TESTDATA)));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(55712, part_one(&parse_input_one(DATA)));
    }

    #[test]
    fn test_part_two_testdata() {
        assert_eq!(281, part_two(&parse_input_two(TESTDATA2)));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(55413, part_two(&parse_input_two(DATA)));
    }
}
