use std::{cmp::{max, min}, fs::File, io::Read, path::Path, vec};
use std::collections::HashSet;

#[derive(Debug)]
#[derive(Clone, Copy)]
struct Range {
    from: u128,
    to: u128,
}
impl Range {
    fn inside_range(&self, num: &u128) -> bool {
        self.from <= *num && self.to >= *num
    }
}

fn parse_input(input: &str) -> (Vec<Range>, Vec<u128>) {
    let mut valid_ranges: Vec<Range> = Vec::new();
    let mut ingredients_ids: Vec<u128> = Vec::new();


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

fn part1(valid_ranges: &Vec<Range>, ingredient_ids: &Vec<u128>) -> u128 {
    ingredient_ids.iter()
        .filter(
            |num| valid_ranges.iter().any(
                |range|range.inside_range(num)
            )
        )
        .count() as u128
}

fn part2_bruteforce(valid_ranges: &Vec<Range>) -> u128 {
    let biggest = valid_ranges.iter().fold(0, |maximum, range| max(maximum, range.to));
    let smallest = valid_ranges.iter().fold(0, |maximum, range| min(maximum, range.to));
    (smallest..=biggest).into_iter()
        .filter(
            |num| valid_ranges.iter().any(
                |range| range.inside_range(num)
            )
        )
        .count() as u128
}

fn part2_bruteforce_memory(valid_ranges: &Vec<Range>) -> u128 {
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

fn part2_using_sets(valid_ranges: &Vec<Range>) -> u128 {
    let mut valid_numbers: HashSet<u128> = HashSet::new();
    let mut i = 0;
    for range in valid_ranges {
        println!("{} / {}", i, valid_ranges.len());
        for num in range.from..=range.to {
            valid_numbers.insert(num);
        }
        i += 1;
    }

    valid_numbers.len() as u128
}

fn part2_merging_ranges(valid_ranges: &Vec<Range>) -> u128 {
    // Sort by the from field
    let mut valid_ranges: Vec<Range> = valid_ranges.iter().map(|r| Range { from: r.from, to: r.to }).collect();
    valid_ranges.sort_by_key(|r| r.from);

    let mut merged_ranges: Vec<Range> = Vec::new();
    let mut current_range: Range = Range { from: (valid_ranges[0].from), to: (valid_ranges[0].to) };
    for idx in 1..valid_ranges.len() {
        if current_range.to < valid_ranges[idx].from {
            // No merging
            merged_ranges.push(current_range);
            current_range = valid_ranges[idx].clone();
        }
        else { 
            // Overlap => expand
            assert!(current_range.to >= valid_ranges[idx].from);
            current_range.to = max(valid_ranges[idx].to, current_range.to);
        }
        if idx == valid_ranges.len() - 1 {
            // Edge case
            merged_ranges.push(current_range);
        }
    }

    // for r in &merged_ranges {
    //     println!("{:?}", &r);
    // }

    // Collect
    merged_ranges.iter()
        .map(|range| range.to - range.from + 1)
        .sum()
}

fn part2(valid_ranges: &Vec<Range>) -> u128 {
    // part2_bruteforce_memory(valid_ranges)
    // part2_using_sets(valid_ranges)
    part2_merging_ranges(valid_ranges)
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
