use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Open the file with puzzle input
    let path = Path::new("day01/input.txt");
    let display = path.display();
    let mut file = match File::open(path) {
        Err(why) => panic!("could not open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file
    let mut problem_input = String::new();
    file.read_to_string(&mut problem_input).expect("File should exist and be UTF-8 compatible!");

    println!("Part 1: {}", part1(&problem_input));
    println!("Part 2: {}", part2(&problem_input));
}

fn part1(input: &str) -> usize {
    let mut val = 50;
    let mut zeros_encountered: u32 = 0;
    for line in input.lines() {
        let (first_char, rest) = line.split_at(1);
        let direction = first_char.chars().next().unwrap();
        let number: i32 = rest.parse().unwrap();

        if direction == 'L' {
            val -= number;
            while val < 0 {
                val = 100 + val;
            }
        }
        else {
            val += number;
            val %= 100;
        }

        if val == 0 {
            zeros_encountered += 1;
        }
    }    
    return zeros_encountered.try_into().unwrap();
}

fn part2(input: &str) -> usize {
    let mut val = 50;
    let mut zeros_encountered: u32 = 0;
    for line in input.lines() {
        let (first_char, rest) = line.split_at(1);
        let direction = first_char.chars().next().unwrap();
        let mut number: i32 = rest.parse().unwrap();

        while number > 0 {
            val = if direction == 'L' {val - 1} else {val + 1};
            if val == 0 || val == 100 {
                zeros_encountered += 1;
            }
            else if val < 0 {
                val += 100
            }
            else if val > 100 {
                val -= 100;
            }
            number -= 1;
        }
    }    
    return zeros_encountered.try_into().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input-example.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 6);
    }
}