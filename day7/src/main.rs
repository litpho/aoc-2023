use anyhow::Result;
use itertools::Itertools;
use nom::{
    character::complete::{self, line_ending, one_of, space1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use std::cmp::Ordering;

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {took}");
    let input = result?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[Hand]) -> u32 {
    input
        .iter()
        .sorted()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.score)
        .sum()
}

fn part_two(input: Vec<Hand>) -> u32 {
    input
        .into_iter()
        .map(Hand::use_jokers)
        .sorted()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.score)
        .sum()
}

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<Label>,
    rank: Rank,
    score: u32,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.cards.eq(&other.cards)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let rank_order = self.rank.cmp(&other.rank);
        if rank_order != Ordering::Equal {
            return rank_order;
        }
        self.cards.cmp(&other.cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    pub fn new(cards: Vec<Label>, score: u32) -> Self {
        let rank = Self::determine_rank(&cards);
        Self { cards, rank, score }
    }

    pub fn use_jokers(self) -> Self {
        let cards = self
            .cards
            .into_iter()
            .map(|label| match label {
                Label::Jack => Label::Joker,
                other => other,
            })
            .collect::<Vec<Label>>();
        let rank = Self::determine_rank_jokers(&cards);
        Self {
            cards,
            rank,
            score: self.score,
        }
    }

    fn determine_rank(cards: &[Label]) -> Rank {
        let map = cards.iter().counts();
        if map.len() == 1 {
            return Rank::FiveOfAKind;
        }
        if map.values().contains(&4) {
            return Rank::FourOfAKind;
        }
        if map.len() == 2 && map.values().contains(&2) && map.values().contains(&3) {
            return Rank::FullHouse;
        }
        if map.values().contains(&3) {
            return Rank::ThreeOfAKind;
        }
        if map.values().sorted().eq(vec![&1usize, &2usize, &2usize]) {
            return Rank::TwoPair;
        }
        if map.values().contains(&2) {
            return Rank::OnePair;
        }

        Rank::HighCard
    }

    fn determine_rank_jokers(cards: &[Label]) -> Rank {
        let map = cards.iter().counts();
        let num_jokers = *map.get(&Label::Joker).unwrap_or(&0);
        let largest_group = *map
            .iter()
            .filter_map(|(label, count)| match label {
                Label::Joker => None,
                _ => Some(count),
            })
            .max()
            .unwrap_or(&0);
        if num_jokers + largest_group == 5 {
            return Rank::FiveOfAKind;
        }
        if num_jokers + largest_group == 4 {
            return Rank::FourOfAKind;
        }
        if num_jokers == 0
            && map.len() == 2
            && map.values().contains(&2)
            && map.values().contains(&3)
        {
            return Rank::FullHouse;
        }
        if num_jokers == 1
            && map.len() == 3
            && map.values().sorted().eq(vec![&1usize, &2usize, &2usize])
        {
            return Rank::FullHouse;
        }
        if num_jokers + largest_group == 3 {
            return Rank::ThreeOfAKind;
        }
        if num_jokers == 0 && map.values().sorted().eq(vec![&1usize, &2usize, &2usize]) {
            return Rank::TwoPair;
        }
        if num_jokers == 1 && map.values().contains(&2) {
            return Rank::TwoPair;
        }
        if num_jokers + largest_group == 2 {
            return Rank::OnePair;
        }

        Rank::HighCard
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Label {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Label {
    type Error = ();

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '2' => Ok(Label::Two),
            '3' => Ok(Label::Three),
            '4' => Ok(Label::Four),
            '5' => Ok(Label::Five),
            '6' => Ok(Label::Six),
            '7' => Ok(Label::Seven),
            '8' => Ok(Label::Eight),
            '9' => Ok(Label::Nine),
            'T' => Ok(Label::Ten),
            'J' => Ok(Label::Jack),
            'Q' => Ok(Label::Queen),
            'K' => Ok(Label::King),
            'A' => Ok(Label::Ace),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn parse(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Hand> {
    map(
        separated_pair(many1(one_of("23456789TJQKA")), space1, complete::u32),
        |(cards, score)| {
            let cards = cards
                .into_iter()
                .map(|c| {
                    c.try_into()
                        .unwrap_or_else(|()| panic!("{c} is not a valid label"))
                })
                .collect::<Vec<Label>>();
            Hand::new(cards, score)
        },
    )(input)
}

fn parse_input(input: &'static str) -> Result<Vec<Hand>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(6440, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(253313241, part_one(&parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(5905, part_two(parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(253362743, part_two(parse_input(DATA)?));

        Ok(())
    }
}
