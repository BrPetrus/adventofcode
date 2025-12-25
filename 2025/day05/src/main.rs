use std::{cmp::{max, min}, fs::File, io::Read, path::Path, vec};
use std::collections::HashSet;

#[derive(Debug)]
struct Range {
    from: u64,
    to: u64,
}
impl Range {
    fn inside_range(&self, num: &u64) -> bool {
        self.from <= *num && self.to >= *num
    }
}

fn parse_input(input: &str) -> (Vec<Range>, Vec<u64>) {
    let mut valid_ranges: Vec<Range> = Vec::new();
    let mut ingredients_ids: Vec<u64> = Vec::new();


    // While we encounter legal ranges
    for line in input.lines() {
        if line.contains("-")  {
            let (before,after) = line.split_once('-').unwrap();
            valid_ranges.push(Range{
                from: before.parse().unwrap(),
                to: after.parse().unwrap(),
            });
        }
        else if line.is_empty() {continue;}
        else {
            ingredients_ids.push(line.parse().unwrap());
        }
    }

    (valid_ranges, ingredients_ids)
}

fn part1(valid_ranges: &Vec<Range>, ingredient_ids: &Vec<u64>) -> u64 {
    ingredient_ids.iter()
        .filter(
            |num| valid_ranges.iter().any(
                |range|range.inside_range(num)
            )
        )
        .count() as u64
}

fn part2_bruteforce(valid_ranges: &Vec<Range>) -> u64 {
    let biggest = valid_ranges.iter().fold(0, |maximum, range| max(maximum, range.to));
    let smallest = valid_ranges.iter().fold(0, |maximum, range| min(maximum, range.to));
    (smallest..=biggest).into_iter()
        .filter(
            |num| valid_ranges.iter().any(
                |range| range.inside_range(num)
            )
        )
        .count() as u64
}

fn part2_bruteforce_memory(valid_ranges: &Vec<Range>) -> u64 {
    let biggest = valid_ranges.iter().fold(0, |maximum, range| max(maximum, range.to)) as usize;
    let smallest = valid_ranges.iter().fold(0, |maximum, range| min(maximum, range.to)) as usize;
    let mut memory: Vec<bool> = vec![false; (biggest-smallest+1) as usize];

    let mut solution = 0;
    for range in valid_ranges {
        for num in range.from..=range.to {
            let num = num as usize;
            if !memory[num-smallest] {solution += 1;}
            memory[num-smallest] = true;
        }
    }
    solution
}

fn part2_using_sets(valid_ranges: &Vec<Range>) -> u64 {
    let mut valid_numbers: HashSet<u64> = HashSet::new();
    let mut i = 0;
    for range in valid_ranges {
        println!("{} / {}", i, valid_ranges.len());
        for num in range.from..=range.to {
            valid_numbers.insert(num);
        }
        i += 1;
    }

    valid_numbers.len() as u64
}

fn part2(valid_ranges: &Vec<Range>) -> u64 {
    // part2_bruteforce_memory(valid_ranges)
    part2_using_sets(valid_ranges)
}

fn main() {
    let path = Path::new("day05/input.txt");
    let mut file = File::open(path).expect("Cannot find input!");
    let mut problem_input = String::new();
    file.read_to_string(&mut problem_input)
        .expect("Cannot read file");

    let (valid_ranges, ingredients) = parse_input(&problem_input);
    println!("Part 1: {}", part1(&valid_ranges, &ingredients));
    println!("Part 2: {}", part2(&valid_ranges));
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../input-example.txt");

    #[test]
    fn test_part1() {
        let (valid_ranges, ingredients) = parse_input(EXAMPLE);
        assert_eq!(part1(&valid_ranges, &ingredients), 3);
    }

    #[test]
    fn test_part2() {
        let (valid_ranges, _) = parse_input(EXAMPLE);
        assert_eq!(part2(&valid_ranges), 14);
    }
}
