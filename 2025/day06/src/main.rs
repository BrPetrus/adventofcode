use std::iter::zip;


fn part1(input: &str) -> u64 {
    let mut problems: Vec<Vec<u64>> = Vec::new();
    let mut operators: Vec<&str> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts[0] == "*" || parts[0] == "+" {
            // We found the operators lines
            operators = parts;
        }
        else {
            for (pos, number) in parts.iter().enumerate() {
                if problems.len() <= pos {problems.push(Vec::new());}
                problems[pos].push(number.parse().expect("Cannot parse number"));
            }
        }
    }

    if operators.len() != problems.len() {
        panic!("Incorrect number of problems and operators encountered!");
    }

    // Actually do the arithmetic
    let mut solution: u64 = 0;
    for (problem, operator) in zip(problems, operators) {
        let mut partial: u64= problem[0];
        for num in problem.iter().skip(1) {
            if operator == "+" {partial += num;}
            else if operator == "*" {partial *= num;}
            else {panic!("Incorrect operator encountered!");}
        }
        solution += partial;
    }
    solution

}

fn part2(input: &str) -> u64 {
    let mut solution: u64 = 0;
    let input_lines: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    // Read the operators
    let mut operators: Vec<(usize, char)> = Vec::new();
    for (pos, ch) in input_lines[input_lines.len() - 1].iter().enumerate() {
        if *ch == '*' || *ch == '+' {
            operators.push((pos, *ch));
        }
        else if *ch == ' ' {}
        else {panic!("Encountered invalid character!");}
    }

    let columns = input_lines[0].len();
    for block_idx in 0..operators.len() {
        // Where does this block start
        let block_column_start = operators[block_idx].0;
        let operator_chr = operators[block_idx].1;

        // Find the dividing vertical line
        let block_column_end: usize;
        if block_idx == operators.len() - 1 {
            block_column_end = columns;
        }
        else {
            block_column_end = operators[block_idx+1].0-1;
        }

        // Read the numbers column by column
        let mut numbers = Vec::new();
        for column_idx in block_column_start..block_column_end {
            let mut number = 0;
            for row_idx in 0..input_lines.len()-1 {
                let chr = input_lines[row_idx][column_idx];
                if chr == ' ' {continue;}
                let digit = chr.to_digit(10).unwrap() as u64;
                number = 10*number + digit;
            }
            numbers.push(number);
        }

        // Apply the operation
        let mut partial_sol = numbers[0];
        for number in numbers.iter().skip(1).copied() {
            if operator_chr == '*' {
                partial_sol *= number;
            }
            else if operator_chr == '+' {
                partial_sol += number;
            }
            else {
                panic!("Unexpceted operator!");
            }
        }
        
        // Acumulate
        solution += partial_sol;
    }

    solution
}

fn main() {
    const INPUT: &str = include_str!("../input.txt");

    println!("Part 1 = {}", part1(INPUT));
    println!("Part 2 = {}", part2(INPUT));
}

mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../input-example.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 3263827);
    }
}

