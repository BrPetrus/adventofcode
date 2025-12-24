use std::fs::File;
use std::io::prelude::*;
use std::mem;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    let path = Path::new("day04/input.txt");
    let content = std::fs::read_to_string(path).expect("Cannot read input file");
    let grid: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    println!("Part 1 {}, ", part1(&grid));
    println!("Part 2 {}, ", part2(&grid));
}

fn find_paperrolls(map: &Vec<Vec<char>>, r: usize, c: usize) -> u32 {
    let rows = map.len();
    let cols = map[0].len();
    let mut count: u32 = 0;
    let rows_is = rows as isize;
    let cols_is = cols as isize;

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 { continue; } // skip center
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr < 0 || nr >= rows_is || nc < 0 || nc >= cols_is { continue; }
            if map[nr as usize][nc as usize] == '@' {
                count += 1;
            }
        }
    }

    count
}

fn part1(map: &Vec<Vec<char>>) -> u32{
    let rows = map.len();
    let cols = map[0].len();
    let mut solution: u32 = 0;

    for r in 0..rows {
        for c in 0..cols {
            if map[r][c] != '@' {continue;}
            solution += if find_paperrolls(map, r, c) < 4 {1} else {0} ;
        }
    }

    solution
}


fn part2_replace(prev_map: &Vec<Vec<char>>, curr_map: &mut Vec<Vec<char>>) -> u32 {
    let rows = prev_map.len();
    let cols = prev_map[0].len();
    let mut solution: u32 = 0;

    for r in 0..rows {
        for c in 0..cols {
            if prev_map[r][c] != '@' {continue;}
            if find_paperrolls(prev_map, r, c) >= 4 {continue;}

            curr_map[r][c] = '.';
            solution += 1;
        }
    }
    solution
}

fn part2_bruteforce(map: &Vec<Vec<char>>) -> u32 {
    let mut solution: u32 = 0;
    let mut curr_map = map.clone();
    let mut prev_map = map.clone();

    // do while `` {
        
    // }

    loop {
        let removed = part2_replace(&prev_map, &mut curr_map);
        if removed == 0 {break;}
        solution += removed;
        // mem::swap(&mut prev_map,&mut curr_map);
        prev_map = curr_map.clone();
        println!("Removed {}", removed);
    }

    solution

}

fn part2(map: &Vec<Vec<char>>) -> u32 {
    part2_bruteforce(map)
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input-example.txt");

    #[test]
    fn test_part1() {
        let grid: Vec<Vec<char>> = EXAMPLE
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        assert_eq!(part1(&grid), 13);
    }

    #[test]
    fn test_part2() {
        let grid: Vec<Vec<char>> = EXAMPLE
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        assert_eq!(part2(&grid), 43);
    }
}
