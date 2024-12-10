use std::{fs, time::Instant};

use utils::Drive;

mod utils;

type ParsedInput = Drive;
type Output = u64;

fn main() {
    let input = input();

    let start = Instant::now();
    println!("Result (part 1): {} [{:?}]", part1(&input), start.elapsed());

    let start = Instant::now();
    println!("Result (part 2): {} [{:?}]", part2(&input), start.elapsed());
}

fn input() -> ParsedInput {
    let input = fs::read_to_string("inputs/day-09.txt").unwrap();
    parse_input(&input)
}

fn parse_input(input: &str) -> ParsedInput {
    Drive::new(input.trim())
}

fn part1(input: &ParsedInput) -> Output {
    let mut compacted_drive = input.clone();

    compacted_drive.dumb_compact();

    compacted_drive.checksum()
}

fn part2(input: &ParsedInput) -> Output {
    let mut compacted_drive = input.clone();

    compacted_drive.smart_compact();

    compacted_drive.checksum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        parse_input("2333133121414131402")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 2858);
    }

    #[test]
    fn test_part2_other() {
        assert_eq!(part2(&parse_input("12345")), 132);
    }
}
