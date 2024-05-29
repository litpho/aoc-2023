use std::ops::Range;

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    combinator::eof,
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
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

fn part_one(input: &Almanac) -> u64 {
    input
        .seeds
        .iter()
        .map(|seed| input.seed_to_location(*seed))
        .min()
        .unwrap()
}

fn part_two(input: &Almanac) -> u64 {
    input
        .seeds
        .chunks(2)
        .flat_map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .map(|seed| input.seed_to_location(seed))
        .min()
        .unwrap()
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: AlmanacMap,
    soil_to_fertilizer: AlmanacMap,
    fertilizer_to_water: AlmanacMap,
    water_to_light: AlmanacMap,
    light_to_temperature: AlmanacMap,
    temperature_to_humidity: AlmanacMap,
    humidity_to_location: AlmanacMap,
}

impl Almanac {
    pub fn seed_to_location(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.get(seed);
        let fertilizer = self.soil_to_fertilizer.get(soil);
        let water = self.fertilizer_to_water.get(fertilizer);
        let light = self.water_to_light.get(water);
        let temperature = self.light_to_temperature.get(light);
        let humidity = self.temperature_to_humidity.get(temperature);
        self.humidity_to_location.get(humidity)
    }
}

#[derive(Debug)]
struct AlmanacMap {
    ranges: Vec<AlmanacRange>,
}

#[derive(Debug)]
struct AlmanacRange {
    range: Range<u64>,
    base: u64,
}

impl AlmanacMap {
    pub fn get(&self, key: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|r| {
                if r.range.contains(&key) {
                    Some(key - r.range.start + r.base)
                } else {
                    None
                }
            })
            .unwrap_or(key)
    }
}

fn parse(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = parse_seeds(input)?;
    let (input, seed_to_soil) = parse_map(input, "seed-to-soil")?;
    let (input, soil_to_fertilizer) = parse_map(input, "soil-to-fertilizer")?;
    let (input, fertilizer_to_water) = parse_map(input, "fertilizer-to-water")?;
    let (input, water_to_light) = parse_map(input, "water-to-light")?;
    let (input, light_to_temperature) = parse_map(input, "light-to-temperature")?;
    let (input, temperature_to_humidity) = parse_map(input, "temperature-to-humidity")?;
    let (input, humidity_to_location) = parse_map(input, "humidity-to-location")?;
    let almanac = Almanac {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    };
    Ok((input, almanac))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    delimited(
        tag("seeds: "),
        separated_list1(space1, complete::u64),
        line_ending,
    )(input)
}

fn parse_map<'a>(input: &'a str, _label: &str) -> IResult<&'a str, AlmanacMap> {
    let (input, _) = delimited(
        line_ending,
        tag(format!("{_label} map:").as_str()),
        line_ending,
    )(input)?;
    let (input, lines) = terminated(
        separated_list1(
            line_ending,
            tuple((complete::u64, space1, complete::u64, space1, complete::u64)),
        ),
        alt((line_ending, eof)),
    )(input)?;
    let ranges = lines
        .iter()
        .map(|line| {
            let range = line.2..(line.2 + line.4);
            let base = line.0;
            AlmanacRange { range, base }
        })
        .collect::<Vec<AlmanacRange>>();

    Ok((input, AlmanacMap { ranges }))
}

fn parse_input(input: &'static str) -> Result<Almanac> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        let almanac = parse_input(TESTDATA)?;

        assert_eq!(81, almanac.seed_to_soil.get(79));
        assert_eq!(14, almanac.seed_to_soil.get(14));
        assert_eq!(57, almanac.seed_to_soil.get(55));
        assert_eq!(13, almanac.seed_to_soil.get(13));

        assert_eq!(82, almanac.seed_to_location(79));
        assert_eq!(43, almanac.seed_to_location(14));
        assert_eq!(86, almanac.seed_to_location(55));
        assert_eq!(35, almanac.seed_to_location(13));

        assert_eq!(35, part_one(&almanac));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(177942185, part_one(&parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(46, part_two(&parse_input(TESTDATA)?));

        Ok(())
    }

    // #[test]
    // fn test_part_two() -> Result<()> {
    //     assert_eq!(69841803, part_two(&parse_input(DATA)?));
    //
    //     Ok(())
    // }
}
