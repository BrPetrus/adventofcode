fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    todo!()
}

fn part2(input: &str) -> usize {
    todo!()
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
        assert_eq!(part2(EXAMPLE), 0);
    }
}