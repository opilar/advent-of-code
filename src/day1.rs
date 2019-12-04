#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve(input: &[u32]) -> u32 {
    input
        .iter()
        .map(|mass| {
            mass/3 - 2
        })
        .sum()
}
