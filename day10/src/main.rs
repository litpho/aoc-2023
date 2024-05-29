#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::{Display, Formatter};

use anyhow::Result;

use crate::Type::{
    Ground, Horizontal, NorthEast, NorthWest, SouthEast, SouthWest, Start, Vertical,
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

fn part_one(input: &Field) -> usize {
    let start = &input.start;
    let mut current = *start;
    let mut prev = None;
    let mut count = 0usize;
    loop {
        count += 1;
        let next = input.next(current, prev);
        if next == *start {
            return count / 2;
        }
        prev = Some(current);
        current = next;
    }
}

fn part_two(input: &Field) -> usize {
    let start = &input.start;
    let mut current = *start;
    let mut prev = None;
    let mut poly = vec![];
    loop {
        // count += 1;
        let next = input.next(current, prev);
        poly.push(next);
        if next == *start {
            break;
        }
        prev = Some(current);
        current = next;
    }

    let mut result = vec![];
    for (y, v) in input.tiles.iter().enumerate() {
        for x in 0..v.len() {
            let res = is_point_in_path(x as isize, y as isize, &poly);
            println!("{x},{y} -> {res:?}");
            if res != PathResult::Out {
                result.push((x, y));
            }
        }
    }

    println!("Result: {}/{:?}", result.len(), result);
    println!("Poly: {}/{:?}", poly.len(), poly);

    result.len() - poly.len() + 1
}

fn is_point_in_path(x: isize, y: isize, poly: &[(isize, isize)]) -> PathResult {
    let num = poly.len();
    let mut j = num - 1;
    let mut c = false;
    for i in 0..num {
        if x == poly[i].0 && y == poly[i].1 {
            // corner
            return PathResult::Corner;
        }
        if (poly[i].1 > y) != (poly[j].1 > y) {
            let slope = (x - poly[i].0) * (poly[j].1 - poly[i].1)
                - (poly[j].0 - poly[i].0) * (y - poly[i].1);
            if slope == 0 {
                // boundary
                return PathResult::Boundary;
            }
            if (slope < 0) != (poly[j].1 < poly[i].1) {
                c = !c;
                j = i;
            }
        }
    }
    if c {
        PathResult::In
    } else {
        PathResult::Out
    }
}

#[derive(Debug, Eq, PartialEq)]
enum PathResult {
    Boundary,
    Corner,
    In,
    Out,
}

// def is_point_in_path(x: int, y: int, poly) -> bool:
// """Determine if the point is on the path, corner, or boundary of the polygon
//
//     Args:
//       x -- The x coordinates of point.
//       y -- The y coordinates of point.
//       poly -- a list of tuples [(x, y), (x, y), ...]
//
//     Returns:
//       True if the point is in the path or is a corner or on the boundary"""
// num = len(poly)
// j = num - 1
// c = False
// for i in range(num):
// if (x == poly[i][0]) and (y == poly[i][1]):
// # point is a corner
// return True
// if (poly[i][1] > y) != (poly[j][1] > y):
// slope = (x - poly[i][0]) * (poly[j][1] - poly[i][1]) - (
// poly[j][0] - poly[i][0]
// ) * (y - poly[i][1])
// if slope == 0:
// # point is on boundary
// return True
// if (slope < 0) != (poly[j][1] < poly[i][1]):
// c = not c
// j = i
// return c

#[derive(Debug)]
struct Field {
    tiles: Vec<Vec<Type>>,
    start: (isize, isize),
}

impl Field {
    pub fn new(tiles: Vec<Vec<Type>>, start: (isize, isize)) -> Self {
        Field { tiles, start }
    }

    pub fn extract_start(tiles: &mut [Vec<Type>]) -> (isize, isize) {
        let start = tiles
            .iter()
            .enumerate()
            .find_map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .find(|(_, typ)| typ == &&Start)
                    .map(|(col, _)| (col as isize, row as isize))
            })
            .unwrap();

        Self::replace_start_tile(tiles, start);

        start
    }

    pub fn get(&self, coords: (isize, isize)) -> &Type {
        &self.tiles[coords.1 as usize][coords.0 as usize]
    }

    pub fn next(&self, current: (isize, isize), prev: Option<(isize, isize)>) -> (isize, isize) {
        let modifiers = self.get(current).next();
        let modifier = match prev {
            None => modifiers[0],
            Some(prev_coords) => {
                if prev_coords == (current.0 + modifiers[0][0], current.1 + modifiers[0][1]) {
                    modifiers[1]
                } else {
                    modifiers[0]
                }
            }
        };

        (current.0 + modifier[0], current.1 + modifier[1])
    }

    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_wrap)]
    fn replace_start_tile(tiles: &mut [Vec<Type>], start: (isize, isize)) {
        const NORTH: usize = 1;
        const EAST: usize = 2;
        const SOUTH: usize = 4;
        const WEST: usize = 8;
        const NORTH_SOUTH: usize = NORTH + SOUTH;
        const EAST_WEST: usize = EAST + WEST;
        const NORTH_EAST: usize = NORTH + EAST;
        const NORTH_WEST: usize = NORTH + WEST;
        const SOUTH_WEST: usize = SOUTH + WEST;
        const SOUTH_EAST: usize = SOUTH + EAST;

        let max_rows = tiles.len();
        let max_cols = tiles[0].len();
        let north = Self::direction(
            || {
                if start.1 == 0 {
                    None
                } else {
                    Some(tiles[(start.1 - 1) as usize][start.0 as usize])
                }
            },
            [Vertical, SouthWest, SouthEast],
            NORTH,
        );
        let east = Self::direction(
            || {
                if start.0 == (max_cols - 1) as isize {
                    None
                } else {
                    Some(tiles[start.1 as usize][(start.0 + 1) as usize])
                }
            },
            [Horizontal, SouthWest, NorthWest],
            EAST,
        );
        let south = Self::direction(
            || {
                if start.1 == (max_rows - 1) as isize {
                    None
                } else {
                    Some(tiles[(start.1 + 1) as usize][start.0 as usize])
                }
            },
            [Vertical, NorthWest, NorthEast],
            SOUTH,
        );
        let west = Self::direction(
            || {
                if start.0 == 0 {
                    None
                } else {
                    Some(tiles[start.1 as usize][(start.0 - 1) as usize])
                }
            },
            [Horizontal, SouthEast, NorthEast],
            WEST,
        );

        let new_type = match north + east + south + west {
            NORTH_SOUTH => Vertical,
            EAST_WEST => Horizontal,
            NORTH_EAST => NorthEast,
            NORTH_WEST => NorthWest,
            SOUTH_WEST => SouthWest,
            SOUTH_EAST => SouthEast,
            _ => panic!(),
        };

        tiles[start.1 as usize][start.0 as usize] = new_type;
    }

    fn direction<F>(f: F, valid: [Type; 3], default: usize) -> usize
    where
        F: Fn() -> Option<Type>,
    {
        f().filter(|x| valid.contains(x)).map_or(0, |_| default)
    }

    fn broaden(self) -> Self {
        let start = (self.start.0 * 2, self.start.1 * 2);
        let mut tiles = vec![];
        for (i, lines) in self.tiles.windows(3).enumerate() {
            let mut new_lines = vec![];
            if i == 0 {
                new_lines = Self::expand(&lines[0..1]);
            }

            new_lines.into_iter().for_each(|line| tiles.push(line));
        }
        // for y in 0..tiles.len() {
        //     for x in 0..tiles.len() {
        //
        //     }
        // }

        Field { tiles, start }
    }

    fn expand(lines: &[Vec<Type>]) -> Vec<Vec<Type>> {
        todo!()
        // let mut new_lines = vec![];
        // if lines.len() == 2 {
        //     let mut new_line = vec![];
        //     lines[0].windows(3).enumerate().for_each(|(i,t)| {
        //         match (t[0], t[1], t[2]) {
        //             (_, Vertical, _) => {}
        //             Horizontal => {}
        //             NorthEast => {}
        //             NorthWest => {}
        //             SouthWest => {}
        //             SouthEast => {}
        //             Ground => {}
        //             Start => {}
        //         }
        //     });
        //     new_lines.push(new_line);
        // }
        // new_lines
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for col in row {
                write!(f, "{col}")?;
            }
            f.write_str("\r\n")?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Type {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Type {
    pub fn next(self) -> [[isize; 2]; 2] {
        match self {
            Vertical => [[0, -1], [0, 1]],
            Horizontal => [[-1, 0], [1, 0]],
            NorthEast => [[0, -1], [1, 0]],
            NorthWest => [[0, -1], [-1, 0]],
            SouthWest => [[0, 1], [-1, 0]],
            SouthEast => [[0, 1], [1, 0]],
            Ground => panic!(),
            Start => panic!(),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Vertical => '│',
            Horizontal => '─',
            NorthEast => '└',
            NorthWest => '┘',
            SouthWest => '┐',
            SouthEast => '┌',
            Ground => '.',
            Start => 'S',
        };
        write!(f, "{c}")
    }
}

impl TryFrom<char> for Type {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '|' => Ok(Vertical),
            '-' => Ok(Horizontal),
            'L' => Ok(NorthEast),
            'J' => Ok(NorthWest),
            '7' => Ok(SouthWest),
            'F' => Ok(SouthEast),
            '.' => Ok(Ground),
            'S' => Ok(Start),
            _ => Err(anyhow::Error::msg(format!("Failed to parse {value}"))),
        }
    }
}

fn parse(input: &str) -> Result<Field> {
    let mut tiles = input
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<Vec<Type>>>>()?;
    let start = Field::extract_start(&mut tiles);

    Ok(Field::new(tiles, start))
}

fn parse_line(input: &str) -> Result<Vec<Type>> {
    input
        .chars()
        .map(Type::try_from)
        .collect::<Result<Vec<Type>>>()
}

fn parse_input(input: &'static str) -> Result<Field> {
    let input = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");
    const TESTDATA2: &str = include_str!("test2.txt");
    const TESTDATA3: &str = include_str!("test3.txt");
    const TESTDATA4: &str = include_str!("test4.txt");
    const TESTDATA5: &str = include_str!("test5.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(4, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one_testdata2() -> Result<()> {
        assert_eq!(8, part_one(&parse_input(TESTDATA2)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(6690, part_one(&parse_input(DATA)?));

        Ok(())
    }

    // #[test]
    // fn test_part_two_testdata() -> Result<()> {
    //     assert_eq!(4, part_two(parse_input(TESTDATA3)?));
    //
    //     Ok(())
    // }
    //
    // #[test]
    // fn test_part_two_testdata2() -> Result<()> {
    //     assert_eq!(8, part_two(parse_input(TESTDATA4)?));
    //
    //     Ok(())
    // }

    // #[test]
    // fn test_part_two_testdata3() -> Result<()> {
    //     assert_eq!(10, part_two(parse_input(TESTDATA5)?));
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
