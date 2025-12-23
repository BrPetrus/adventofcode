use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Open the file with puzzle input
    let path = Path::new("day02/input.txt");
    let display = path.display();
    let mut file = match File::open(path) {
        Err(why) => panic!("could not open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file
    let mut problem_input = String::new();
    file.read_to_string(&mut problem_input).expect("File should exist and be UTF-8 compatible!");
    let ranges = build_ranges(&problem_input);

    println!("Part 1: {}", part1(&ranges));
    println!("Part 2: {}", part2(&ranges));
}

fn build_ranges(input: &str) -> Vec<(String, String)> {
    let mut result = Vec::new();
    let ranges = input.trim().split(',');
    for range in ranges {
        let (from, to) = range.split_once('-').unwrap();
        // result.push((from.parse().unwrap(), to.parse().unwrap()));
        result.push((from.to_owned(), to.to_owned()));
    }
    result
}

fn is_legal(number: &str) -> bool {
    let chars: Vec<char> = number.chars().collect();
    if chars.len() % 2 != 0 { return false; }
    chars.iter()
         .take(chars.len() / 2)
         .zip(chars.iter().skip(chars.len() / 2))
         .all(|(a, b)| a == b)
}

fn is_legal_complex(number: &str) -> bool {
    let chars: Vec<char> = number.chars().collect();
    for suffix_len in 1..chars.len() / 2 + 1 {
        if chars.len() % suffix_len != 0 {continue;}
        let repetitions = number.len() / suffix_len;

        let mut legal_comb = true;
        for j in 0..suffix_len {
            for i in 1..repetitions {
                if chars[j] != chars[j+suffix_len*i] {
                    legal_comb = false;
                    break;
                }
            }
        }
        if legal_comb {return true;}
    }
    return false;
}

fn part1(input_ranges: &Vec<(String, String)>) -> usize {
    input_ranges.into_iter()
        .map(|(from_s, to_s)| {
            let from: u64 = from_s.parse().expect("parse from");
            let to: u64 = to_s.parse().expect("parse to");
            (from, to)
        })
        .map(|(from, to)| {
            (from..=to)
                .filter(|&n| is_legal(&n.to_string()))
                .map(|n| n as usize)
                .sum::<usize>()
        })
        .sum()
}

fn part2(input_ranges: &Vec<(String, String)>) -> usize {
    input_ranges.into_iter()
        .map(|(from_s, to_s)| {
            let from: u64 = from_s.parse().expect("parse from");
            let to: u64 = to_s.parse().expect("parse to");
            (from, to)
        })
        .map(|(from, to)| {
            (from..=to)
                .filter(|&n| is_legal_complex(&n.to_string()))
                .map(|n| n as usize)
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input-example.txt");


    #[test]
    fn test_part1() {
        // assert_eq!(part1(EXAMPLE), 1227775554);
        let input = build_ranges(EXAMPLE);
        assert_eq!(part1(&input), 1227775554);
    }

    #[test]
    fn test_part2() {
        let input = build_ranges(EXAMPLE);
        assert_eq!(part2(&input), 4174379265);
    }

    #[test]
    fn is_legal_complex_cases() {
        let cases: &[(&str, bool)] = &[
            ("1212", true),
            ("1221", false),
            ("123123", true),
            ("12312", false),
            ("38593859", true),
            ("446446", true),
            ("1188511885", true),
            ("222222", true)
            // add as many hardcoded cases as you want
        ];

        for (input, expected) in cases {
            assert_eq!(
                is_legal_complex(&input),
                *expected,
                "case failed for input = {}",
                input
            );
        }
    }
}