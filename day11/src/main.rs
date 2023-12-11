use anyhow::Result;
use bit_vec::BitVec;
use itertools::Itertools;

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {}", took);
    let input = result?;

    let (took, result) = took::took(|| part_one(input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {}", took);
    let input = result?;

    let (took, result) = took::took(|| part_two(input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(mut input: Galaxy) -> u64 {
    input.expand(2);
    sum_manhattan_distance(&input)
}

fn part_two(mut input: Galaxy) -> u64 {
    input.expand(1_000_000);
    // input.expand(999_999);
    sum_manhattan_distance(&input)
}

fn sum_manhattan_distance(input: &Galaxy) -> u64 {
    input
        .stars
        .iter()
        .combinations(2)
        .map(|v| v[0].0.abs_diff(v[1].0) + v[0].1.abs_diff(v[1].1))
        .sum()
}

#[derive(Debug)]
struct Galaxy {
    stars: Vec<(u64, u64)>,
    size: (usize, usize),
}

impl Galaxy {
    fn expand(&mut self, multiplier: u64) {
        let mut new_cols = BitVec::from_elem(self.size.0, true);
        let mut new_rows = BitVec::from_elem(self.size.1, true);
        self.stars.iter().for_each(|(x, y)| {
            new_cols.set(*x as usize, false);
            new_rows.set(*y as usize, false);
        });

        for (x, y) in self.stars.iter_mut() {
            *x += (0..*x)
                .filter(|z| new_cols.get(*z as usize).unwrap())
                .count() as u64
                * (multiplier - 1);
            *y += (0..*y)
                .filter(|z| new_rows.get(*z as usize).unwrap())
                .count() as u64
                * (multiplier - 1);
        }
    }
}

fn parse(input: &str) -> Galaxy {
    let stars = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as u64, y as u64))
                } else {
                    None
                }
            })
        })
        .collect();
    let size = (input.lines().next().unwrap().len(), input.lines().count());

    Galaxy { stars, size }
}

fn parse_input(input: &'static str) -> Result<Galaxy> {
    let input = parse(input);

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(374, part_one(parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(10289334, part_one(parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        let mut galaxy = parse_input(TESTDATA)?;
        galaxy.expand(10);
        assert_eq!(1030, sum_manhattan_distance(&galaxy));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata2() -> Result<()> {
        let mut galaxy = parse_input(TESTDATA)?;
        galaxy.expand(100);
        assert_eq!(8410, sum_manhattan_distance(&galaxy));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(649862989626, part_two(parse_input(DATA)?));

        Ok(())
    }
}
