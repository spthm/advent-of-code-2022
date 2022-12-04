use std::num::ParseIntError;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Assignment {
    start: u32,
    end: u32,
}

impl Assignment {
    fn overlaps(&self, other: &Self) -> bool {
        return self.start <= other.end && self.end >= other.start;
    }

    fn is_superset(&self, other: &Self) -> bool {
        return self.start <= other.start && self.end >= other.end;
    }
}

impl FromStr for Assignment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once('-').unwrap();
        let start = l.parse::<u32>()?;
        let end = r.parse::<u32>()?;
        Ok(Self { start, end })
    }
}

#[derive(Debug)]
pub struct Pair {
    first: Assignment,
    second: Assignment,
}

impl FromStr for Pair {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once(',').unwrap();

        let first = l.parse::<Assignment>()?;
        let second = r.parse::<Assignment>()?;
        Ok(Self { first, second })
    }
}

#[aoc_generator(day4)]
pub fn parse(input: &str) -> Vec<Pair> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day4, part1)]
pub fn part1(parsed: &Vec<Pair>) -> usize {
    parsed
        .iter()
        .filter(|pair| pair.first.is_superset(&pair.second) || pair.second.is_superset(&pair.first))
        .count()
}

#[aoc(day4, part2)]
pub fn part2(parsed: &Vec<Pair>) -> usize {
    parsed
        .iter()
        .filter(|pair| pair.first.overlaps(&pair.second))
        .count()
}
