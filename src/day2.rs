#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[u32]) -> u32 {
    let mut program = Vec::from(input);
    program = fix(program);
    program = run(program);
    program[0]
}

fn fix(state: Vec<u32>) -> Vec<u32> {
    fix_with(state, 12, 2)
}

fn fix_with(mut state: Vec<u32>, first: u32, second: u32) -> Vec<u32> {
    state[1] = first;
    state[2] = second;

    state
}

fn run(mut state: Vec<u32>) -> Vec<u32> {
    let mut index = 0;
    while index < state.len() {
        let opcode = state[index];
        match opcode {
            1 | 2 => {
                let left_pos = state[index+1] as usize;
                let right_pos = state[index+2] as usize;
                let result_pos = state[index+3] as usize;

                let left_operand = state[left_pos];
                let right_operand = state[right_pos];
                let result = if opcode == 1 {
                    left_operand + right_operand
                } else {
                    left_operand * right_operand
                };

                state[result_pos] = result;
                index += 4;
            }
            99 => { break }
            _ => panic!("Unexpected opcode {}", opcode)
        }
    }
    state
}

#[aoc(day2, part2)]
pub fn part2(input: &[u32]) -> u32 {
    const RESULT_TO_FIND: u32 = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = Vec::from(input);
            program = fix_with(program, noun, verb);
            let result = run_and_return(program);
            if result == RESULT_TO_FIND {
                return 100*noun + verb;
            }
        }
    }

    panic!("Failed to find verb and noun");
}

fn run_and_return(mut program: Vec<u32>) -> u32 {
    program = run(program);
    program[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(run([1,0,0,0,99].to_vec()), [2,0,0,0,99]);
        assert_eq!(run([2,3,0,3,99].to_vec()), [2,3,0,6,99]);
        assert_eq!(run([2,4,4,5,99,0].to_vec()), [2,4,4,5,99,9801]);
        assert_eq!(run([1,1,1,4,99,5,6,0,99].to_vec()), [30,1,1,4,2,5,6,0,99]);
    }
}
