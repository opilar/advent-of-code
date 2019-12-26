#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> u32 {
    input
        .iter()
        .copied()
        .map(calc_fuel)
        .sum()
}

fn calc_fuel(mass: u32) -> u32 {
    if mass <= 6 {
        0
    } else {
        mass/3 - 2
    }
}

#[aoc(day1, part2)]
pub fn part2(input: &[u32]) -> u32 {
    input
        .iter()
        .copied()
        .map(calc_fuel_with_additional_fuel)
        .sum()
}

fn calc_fuel_with_additional_fuel(mass: u32) -> u32 {
    let mut fuel_needed = calc_fuel(mass);
    let mut result = fuel_needed;
    while fuel_needed != 0 {
        fuel_needed = calc_fuel(fuel_needed);
        result += fuel_needed;
    }
    result
}
