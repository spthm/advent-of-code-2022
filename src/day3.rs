use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Vec<String> {
    input.split('\n').map(|line| line.to_string()).collect()
}

#[aoc(day3, part1)]
pub fn part1(parsed: &Vec<String>) -> u32 {
    parsed
        .iter()
        .map(|rucksack| {
            let (first, second) = rucksack.split_at(rucksack.len() / 2);
            let c = common(first, second);
            priority(c.as_bytes()[0])
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(parsed: &Vec<String>) -> u32 {
    parsed
        .chunks(3)
        .map(|group| {
            let mut c = common(group[0].as_str(), group[1].as_str());
            c = common(c.as_str(), group[2].as_str());
            priority(c.as_bytes()[0])
        })
        .sum()
}

fn common(first: &str, second: &str) -> String {
    let fs: HashSet<char> = HashSet::from_iter(first.chars());
    let ss: HashSet<char> = HashSet::from_iter(second.chars());

    return fs.intersection(&ss).collect::<String>()
}

fn priority(c: u8) -> u32 {
    assert!(c.is_ascii());

    match c as char {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => panic!("{} not in [a - z][A - Z]", c)
    }
}
