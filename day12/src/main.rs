use ahash::{HashMap, HashMapExt};
use anyhow::Result;
use itertools::Itertools;
use nom::character::complete;
use nom::character::complete::one_of;
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::separated_pair;
use nom::{character::complete::line_ending, multi::separated_list1, IResult};
use once_cell::sync::Lazy;
use std::sync::Mutex;

const DATA: &str = include_str!("input.txt");

static GLOBAL_DATA: Lazy<Mutex<HashMap<u64, Vec<usize>>>> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});

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

fn part_one(input: &[Row]) -> u64 {
    input.iter().map(solve).sum()
}

fn part_two(input: &[Row]) -> u64 {
    input
        .iter()
        .map(|r| {
            let new_content = format!(
                "{}?{}?{}?{}?{}",
                r.content, r.content, r.content, r.content, r.content
            );
            let new_sizes = r.sizes.repeat(5);
            Row::new(new_content, new_sizes)
        })
        .enumerate()
        .map(|(_, r)| solve(&r))
        .sum()
}

fn solve(row: &Row) -> u64 {
    let marks = row
        .content
        .char_indices()
        .filter_map(|(i, c)| if c == '?' { Some(i) } else { None })
        .collect::<Vec<usize>>();

    let mut count = 0;
    for r in (0..marks.len())
        .map(|_| ".#".chars())
        .multi_cartesian_product()
    {
        let new_str = replace_marks(row, &marks, &r);
        if calculate_bit_vec_sizes(new_str) == row.sizes {
            count += 1;
        }
    }

    count
}

fn replace_marks(row: &Row, marks: &[usize], replacements: &[char]) -> u64 {
    let mut idx = 0;
    let mut next_mark = marks.get(idx);
    row.content
        .char_indices()
        .map(|(i, c)| match next_mark {
            Some(j) if i == *j => {
                idx += 1;
                next_mark = marks.get(idx);
                (i, replacements[idx - 1])
            }
            _ => (i, c),
        })
        .fold(0, |acc, (i, c)| if c == '#' { acc | 1 << i } else { acc })
}

fn calculate_bit_vec_sizes(input: u64) -> Vec<usize> {
    if let Some(value) = GLOBAL_DATA.lock().unwrap().get(&input) {
        // println!("Cache hit for {input}");
        return value.clone();
    }

    let mut number = input;
    let mut result = vec![];
    let mut count = 0usize;
    loop {
        if number & 1 == 1 {
            count += 1;
        } else if count > 0 {
            result.push(count);
            count = 0;
        }
        if number == 0 {
            GLOBAL_DATA.lock().unwrap().insert(input, result.clone());
            return result;
        }
        number >>= 1;
    }
}

#[derive(Debug)]
struct Row {
    content: String,
    sizes: Vec<usize>,
}

impl Row {
    pub fn new(content: String, sizes: Vec<usize>) -> Self {
        Row { content, sizes }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Row>> {
    separated_list1(line_ending, parse_row)(input)
}

fn parse_row(input: &str) -> IResult<&str, Row> {
    map(
        separated_pair(parse_content, complete::char(' '), parse_sizes),
        |(content, sizes)| Row::new(content, sizes),
    )(input)
}

fn parse_content(input: &str) -> IResult<&str, String> {
    map(many1(one_of(".#?")), |v| v.iter().collect::<String>())(input)
}

fn parse_sizes(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(complete::char(','), map(complete::u8, |v| v as usize))(input)
}

fn parse_input(input: &'static str) -> Result<Vec<Row>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_calculate_bit_vec_sizes() -> Result<()> {
        // ###.#
        assert_eq!(vec![3, 1], calculate_bit_vec_sizes(23));
        // #.#.###
        assert_eq!(vec![1, 1, 3], calculate_bit_vec_sizes(117));
        // ###.###
        assert_eq!(vec![3, 3], calculate_bit_vec_sizes(119));
        Ok(())
    }

    #[test]
    fn test_row() -> Result<()> {
        assert_eq!(1, solve(&parse_row("???.### 1,1,3")?.1));
        assert_eq!(4, solve(&parse_row(".??..??...?##. 1,1,3")?.1));
        assert_eq!(1, solve(&parse_row("?#?#?#?#?#?#?#? 1,3,1,6")?.1));
        assert_eq!(1, solve(&parse_row("????.#...#... 4,1,1")?.1));
        assert_eq!(4, solve(&parse_row("????.######..#####. 1,6,5")?.1));
        assert_eq!(10, solve(&parse_row("?###???????? 3,2,1")?.1));
        Ok(())
    }

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(21, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(7653, part_one(&parse_input(DATA)?));

        Ok(())
    }

    // #[test]
    // fn test_part_two_testdata() -> Result<()> {
    //     assert_eq!(525152, part_two(&parse_input(TESTDATA)?));
    //
    //     Ok(())
    // }

    // #[test]
    // fn test_part_two() -> Result<()> {
    //     assert_eq!(649862989626, part_two(parse_input(DATA)?));
    //
    //     Ok(())
    // }
}
