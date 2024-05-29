#![allow(dead_code)]
#![allow(unused_variables)]

use anyhow::Result;
use nom::{
    character::{complete::line_ending, complete::one_of},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::pair,
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

fn part_one_a(input: &[Pattern]) -> u32 {
    let mut count = 0;
    for pattern in input {
        let horizontal = pattern.lines.windows(2).enumerate().find_map(|(i, v)| {
            println!("{i} = {}: {:?}", v[0] == v[1], v);
            if v[0] == v[1] {
                Some((i - 1) as u32 * 100)
            } else {
                None
            }
        });
        count += horizontal.unwrap_or_else(|| {
            let lines = Pattern::transpose(&pattern.lines);
            lines
                .windows(2)
                .enumerate()
                .find_map(|(i, v)| if v[0] == v[1] { Some(i as u32) } else { None })
                .unwrap()
        });
    }
    count
}

fn part_one(input: &[Pattern]) -> u32 {
    input
        .iter()
        .map(|pattern| {
            let v = pattern.find_vertical_mirror();
            let h = pattern.find_horizontal_mirror();
            dbg!(v, h);
            pattern
                .find_horizontal_mirror()
                .map(|h| h * 100)
                .or(pattern.find_vertical_mirror())
                .unwrap()
        })
        .sum()
}

fn part_two(input: &[Pattern]) -> u32 {
    let pattern = input.get(1).unwrap();
    let result = pattern.find_almost_mirror();
    // let columns = Pattern::find_almost_mirror(&pattern.columns);

    println!("Result: {result:?}");
    // println!("Columns: {columns:?}");

    0
}

#[derive(Debug)]
struct Pattern {
    lines: Vec<Vec<char>>,
    rows: Vec<u32>,
    columns: Vec<u32>,
}

impl Pattern {
    pub fn new(lines: Vec<Vec<char>>) -> Self {
        let columns = Self::transform(&Self::transpose(&lines));
        let rows = Self::transform(&lines);

        Pattern {
            lines,
            rows,
            columns,
        }
    }

    pub fn find_horizontal_mirror(&self) -> Option<u32> {
        Self::find_mirror(&self.rows)
        // Self::find_mirror_char(&self.lines)
    }

    pub fn find_vertical_mirror(&self) -> Option<u32> {
        let lines = Self::transpose(&self.lines);
        Self::find_mirror(&self.columns)
        // Self::find_mirror_char(&lines)
    }

    fn find_mirror(input: &[u32]) -> Option<u32> {
        for i in 1..input.len() {
            let (front, back) = input.split_at(i);
            // println!("Comparing {:?} and {:?}", front, back);
            if front.last() == back.first() {
                let mut back = back.to_vec();
                back.reverse();
                // println!("Reversed {:?} and {:?}", front, back);
                if front.ends_with(&back) || back.ends_with(front) {
                    return Some(i as u32);
                }
            }
        }

        None
    }

    // fn find_mirror_char(input: &[Vec<char>]) -> Option<u32> {
    //     for i in 1..input.len() {
    //         let (front, back) = input.split_at(i);
    //         // println!("Comparing {:?} and {:?}", front, back);
    //         if front.last() == back.first() {
    //             let mut back = back.to_vec();
    //             back.reverse();
    //             // println!("Reversed {:?} and {:?}", front, back);
    //             let diff = diff_with(front.into_iter(), back.into_iter(), |a,b| a == &b);
    //             match diff {
    //                 None => return None,
    //                 Some(diff) => {
    //                     println!("{}", diff);
    //                     return None;
    //                 }
    //             }
    //             if front.ends_with(&back) || back.ends_with(front) {
    //                 return Some(i as u32);
    //             }
    //         }
    //     }
    //
    //     None
    // }

    pub fn find_almost_mirror(&self) -> Option<(usize, usize, Option<u32>)> {
        let existing = self
            .find_horizontal_mirror()
            .map(|h| h * 100)
            .or(self.find_vertical_mirror());
        for y in 0..self.lines.len() {
            for x in 0..self.lines[0].len() {
                let mut lines = self.lines.clone();
                let c = lines.get_mut(y).unwrap().get_mut(x).unwrap();
                if c == &'1' {
                    *c = '0';
                } else {
                    *c = '1';
                }
                let pattern = Pattern::new(lines);
                let new = pattern
                    .find_horizontal_mirror()
                    .map(|h| h * 100)
                    .or(pattern.find_vertical_mirror());
                if new.is_some() && new != existing {
                    return Some((x, y, new));
                }
            }
        }

        None
    }

    fn find_smudge(long: &[u32], short: &[u32]) -> Option<(u32, u32)> {
        let mut smudge = None;
        for i in 0..short.len() {
            if short[i] != long[long.len() - 1 - i] {
                if smudge.is_none() {
                    smudge = Some((short[i], long[long.len() - 1 - i]));
                    println!("Temp smudge: {smudge:?}");
                } else {
                    return None;
                }
            }
        }

        None
    }

    pub fn transpose(v: &[Vec<char>]) -> Vec<Vec<char>> {
        let rows = v.len();
        let cols = v[0].len();

        let transposed: Vec<Vec<_>> = (0..cols)
            .map(|col| (0..rows).map(|row| v[row][col]).collect())
            .collect();

        transposed
    }

    fn transform(v: &[Vec<char>]) -> Vec<u32> {
        v.iter()
            .map(|line| {
                let string = line.iter().collect::<String>();
                u32::from_str_radix(string.as_str(), 2).unwrap()
            })
            .collect::<Vec<u32>>()
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Pattern>> {
    separated_list1(pair(line_ending, line_ending), parse_pattern)(input)
}

fn parse_pattern(input: &str) -> IResult<&str, Pattern> {
    map(separated_list1(line_ending, parse_line), |v| {
        Pattern::new(v)
    })(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<char>> {
    many1(map(one_of(".#"), |c| if c == '.' { '0' } else { '1' }))(input)
}

fn parse_input(input: &'static str) -> Result<Vec<Pattern>> {
    let (_, input) = parse(input)?;

    println!("{input:?}");

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(405, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(35232, part_one(&parse_input(DATA)?));

        Ok(())
    }

    // #[test]
    // fn test_part_two_testdata() -> Result<()> {
    //     assert_eq!(400, part_two(&parse_input(TESTDATA)?));
    //
    //     Ok(())
    // }

    // #[test]
    // fn test_part_two() -> Result<()> {
    //     assert_eq!(42948149, part_two(&parse_input(DATA)?));
    //
    //     Ok(())
    // }
}
