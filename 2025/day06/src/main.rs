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

fn main() {
    const INPUT: &str = include_str!("../input.txt");

    println!("Part 1 = {}", part1(INPUT));

}

mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../input-example.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 4277556);
    }

    // #[test]
    // fn test_part2() {
    //     let (valid_ranges, _) = parse_input(EXAMPLE);
    //     assert_eq!(part2(&valid_ranges), 14);
    // }
}

