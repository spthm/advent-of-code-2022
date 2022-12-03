use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<String> {
    input.split('\n').map(|strat| strat.to_string()).collect()
}

#[aoc(day2, part1)]
pub fn part1(parsed: &Vec<String>) -> u32 {
    parsed
        .iter()
        .map(|strat| match strat.as_str() {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => panic!("unknown strategy {}", strat),
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(parsed: &Vec<String>) -> u32 {
    parsed
        .iter()
        .map(|strat| match strat.as_str() {
            "A X" => 3 + 0,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            _ => panic!("unknown strategy {}", strat),
        })
        .sum()
}
