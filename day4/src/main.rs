use std::cmp::min;
use std::collections::{HashMap, HashSet};

use anyhow::Result;
use nom::character::complete::space0;
use nom::sequence::pair;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
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

fn part_one(input: &[Card]) -> u32 {
    input
        .iter()
        .map(|card| match card.num_matches() {
            0 => 0,
            num_matches => 2u32.pow(num_matches as u32 - 1),
        })
        .sum()
}

fn part_two(input: &[Card]) -> usize {
    let max_id = input.len() as u32 + 1;
    let lookup_map = input
        .iter()
        .map(|card| (card.id, card))
        .collect::<HashMap<u32, &Card>>();
    let mut number_of_cards = 0;
    let mut cards = input.iter().map(|card| card.id).collect::<Vec<u32>>();
    loop {
        let mut added_cards: Vec<u32> = vec![];
        cards.iter().for_each(
            |card_id| match lookup_map.get(card_id).unwrap().num_matches() {
                0 => {}
                num_matches => {
                    if card_id < &max_id {
                        (card_id + 1..=min(max_id, card_id + num_matches as u32)).for_each(|id| {
                            added_cards.push(id);
                        })
                    }
                }
            },
        );
        number_of_cards += cards.len();
        if added_cards.is_empty() {
            return number_of_cards;
        }
        cards = added_cards;
    }
}

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

impl Card {
    pub fn num_matches(&self) -> usize {
        self.my_numbers
            .intersection(&self.winning_numbers)
            .collect::<Vec<&u32>>()
            .len()
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Card> {
    map(
        separated_pair(parse_id, tag(": "), parse_number_groups),
        |(id, (my_numbers, winning_numbers))| {
            let my_numbers = my_numbers.into_iter().collect();
            let winning_numbers = winning_numbers.into_iter().collect();
            Card {
                id,
                my_numbers,
                winning_numbers,
            }
        },
    )(input)
}

fn parse_id(input: &str) -> IResult<&str, u32> {
    preceded(pair(tag("Card"), space1), complete::u32)(input)
}

fn parse_number_groups(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    separated_pair(parse_numbers, tag(" | "), parse_numbers)(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(space0, separated_list1(space1, complete::u32))(input)
}

fn parse_input(input: &'static str) -> Result<Vec<Card>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(13, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(21105, part_one(&parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(30, part_two(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(5329815, part_two(&parse_input(DATA)?));

        Ok(())
    }
}
