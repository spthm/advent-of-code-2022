use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn parse(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|kcal| kcal.split('\n').map(|s| s.parse::<u32>().unwrap()).sum())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(parsed: &Vec<u32>) -> u32 {
    *parsed.iter().max().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(parsed: &Vec<u32>) -> u32 {
    let mut v = parsed.to_vec();
    v.sort();
    v.iter().rev().take(3).sum()
}
