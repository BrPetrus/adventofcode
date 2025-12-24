use std::cmp::max;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::mem;

fn main() {
    // Open the file with puzzle input
    let path = Path::new("day03/input.txt");
    let display = path.display();
    let mut file = match File::open(path) {
        Err(why) => panic!("could not open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file
    let mut problem_input = String::new();
    file.read_to_string(&mut problem_input)
        .expect("File should exist.");
    let lines = parse_input(&problem_input);
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn parse_input(problem_input: &str) -> Vec<&str> {
    let lines: Vec<&str> = problem_input.lines().map(|x| x).collect();
    lines
}

fn part1(inputs: &Vec<&str>) -> u64 {
    let mut solution: u64 = 0;
    for input in inputs {
        let input_chars: Vec<u32> = input
            .chars()
            .map(|x| x.to_digit(10).expect("Expected convertable"))
            .collect();
        let mut i = 0;
        let mut j = input_chars.len() - 1;

        // Find the left bound
        for idx in 1..input_chars.len() - 1 {
            if input_chars[idx] > input_chars[i] {
                i = idx
            };
        }

        // Find the right bound
        for idx in i + 1..input_chars.len() {
            if input_chars[idx] > input_chars[j] {
                j = idx
            };
        }

        // Convert to solution-like
        let partial_solution = (input_chars[i] as u64) * 10 + (input_chars[j] as u64);
        // println!("{}", partial_solu:wtion);
        solution += partial_solution;
    }

    solution
}

fn solve_complex_dp(digits: Vec<u32>, using_digits: u32) -> u64 {
    // let mut memory: [u32; digits.len()];
    let mut current_memory: Vec<u64> = vec![0; digits.len() + 1];
    let mut previous_memory: Vec<u64> = vec![0; digits.len() + 1];

    for i in 1..=using_digits as usize {
        for suffix_idx in (0..=digits.len()-i).rev() {
            current_memory[suffix_idx] = max(
                current_memory[suffix_idx + 1],
                previous_memory[suffix_idx + 1] + u64::pow(10, (i-1) as u32) * (digits[suffix_idx] as u64),
            )
        }

        mem::swap(&mut current_memory, &mut previous_memory);
        // clear current_memory for the next iteration
        current_memory.fill(0);

    }

    previous_memory[0]
}

fn part2(inputs: &Vec<&str>) -> u64 {
    let mut solution: u64 = 0;
    for input in inputs {
        let input_chars: Vec<u32> = input
            .chars()
            .map(|x| x.to_digit(10).expect("Expected convertable"))
            .collect();

        let partial_sol = solve_complex_dp(input_chars, 12);
        println!("{}", partial_sol);
        solution += partial_sol;
    }
    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input-example.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 3121910778619);
    }
}
