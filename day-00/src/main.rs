use std::{fs, time::Instant};

mod debug;

type ParsedInput = Vec<Vec<char>>;
type Output = u32;

fn main() {
    let input = input();

    let start = Instant::now();
    println!("Result (part 1): {} [{:?}]", part1(&input), start.elapsed());

    let start = Instant::now();
    println!("Result (part 2): {} [{:?}]", part2(&input), start.elapsed());
}

fn input() -> ParsedInput {
    let input = fs::read_to_string("inputs/day-00.txt").unwrap();
    parse_input(&input)
}

fn parse_input(input: &str) -> ParsedInput {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &ParsedInput) -> Output {
    0
}

fn part2(input: &ParsedInput) -> Output {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        ParsedInput::default()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 0);
    }
}
