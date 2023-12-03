use std::collections::HashSet;
use std::ops::Range;

use anyhow::Result;

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {}", took);
    let input = result?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    // let (took, result) = took::took(|| part_two(&input));
    // println!("Result part two: {result}");
    // println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &EngineMap) -> u32 {
    let mut numbers: HashSet<Number> = HashSet::new();
    for symbol in input.symbols.iter() {
        for number in input.numbers.iter() {
            if number.near_symbol(symbol) {
                numbers.insert(number.clone());
            }
        }
    }

    numbers.iter().map(|number| number.value).sum()
}

fn part_two(_input: &EngineMap) -> u32 {
    todo!()
}

#[derive(Debug)]
struct EngineMap {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Number {
    value: u32,
    length: usize,
    position: (usize, usize),
}

impl Number {
    fn near_symbol(&self, symbol: &Symbol) -> bool {
        symbol.position.1.abs_diff(self.position.1) <= 1
            && self.horizontal_range().contains(&symbol.position.0)
    }

    fn horizontal_range(&self) -> Range<usize> {
        if self.position.0 == 0 {
            self.position.0..(self.position.0 + self.length + 1)
        } else {
            (self.position.0 - 1)..(self.position.0 + self.length + 1)
        }
    }
}

#[derive(Debug)]
struct Symbol {
    value: char,
    position: (usize, usize),
}

fn parse(input: &str) -> Result<EngineMap> {
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];
    for (row, line) in input.lines().enumerate() {
        let mut number_chars = vec![];
        let mut number_pos = (0, row);
        for (col, c) in line.char_indices() {
            if c.is_ascii_digit() {
                if number_chars.is_empty() {
                    number_pos = (col, row);
                }
                number_chars.push(c);
            } else {
                if !number_chars.is_empty() {
                    handle_end_of_number(&mut number_chars, &mut numbers, number_pos)?;
                }
                if c != '.' {
                    symbols.push(Symbol {
                        value: c,
                        position: (col, row),
                    });
                }
            }
        }
        if !number_chars.is_empty() {
            handle_end_of_number(&mut number_chars, &mut numbers, number_pos)?;
        }
    }

    Ok(EngineMap { numbers, symbols })
}

fn handle_end_of_number(
    number_chars: &mut Vec<char>,
    numbers: &mut Vec<Number>,
    number_pos: (usize, usize),
) -> Result<()> {
    let value = number_chars.iter().collect::<String>().parse::<u32>()?;
    let length = number_chars.len();
    numbers.push(Number {
        value,
        length,
        position: number_pos,
    });
    number_chars.clear();

    Ok(())
}

fn parse_input(input: &'static str) -> Result<EngineMap> {
    let input = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(4361, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(560670, part_one(&parse_input(DATA)?));

        Ok(())
    }

    // #[test]
    // fn test_part_two_testdata() -> Result<()> {
    //     assert_eq!(2286, part_two(&parse_input(TESTDATA)?));
    //
    //     Ok(())
    // }
    //
    // #[test]
    // fn test_part_two() -> Result<()> {
    //     assert_eq!(71220, part_two(&parse_input(DATA)?));
    //
    //     Ok(())
    // }
}
