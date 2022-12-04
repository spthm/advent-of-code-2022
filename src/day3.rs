use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day3, part1)]
pub fn part1(parsed: &Vec<String>) -> u32 {
    parsed
        .iter()
        .map(|rucksack| {
            let (first, second) = rucksack.split_at(rucksack.len() / 2);
            let c = first.chars().find(|c| second.contains(*c)).unwrap();
            priority(c)
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(parsed: &Vec<String>) -> u32 {
    parsed
        .chunks_exact(3)
        .map(|group| {
            let c = group[0]
                .chars()
                .find(|c| group[1].contains(*c) && group[2].contains(*c))
                .unwrap();
            priority(c)
        })
        .sum()
}

fn priority(c: char) -> u32 {
    assert!(c.is_ascii());

    match c {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => panic!("{} not in [a - z][A - Z]", c),
    }
}
