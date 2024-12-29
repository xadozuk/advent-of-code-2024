use std::{fs, time::Instant};

use itertools::iproduct;
use lib::debugln;

type ParsedInput = System;
type Output = usize;

#[derive(Debug)]
struct System {
    pub locks: Vec<Vec<u8>>,
    pub keys: Vec<Vec<u8>>,
}

fn main() {
    let input = input();

    let start = Instant::now();
    println!("Result (part 1): {} [{:?}]", part1(&input), start.elapsed());

    let start = Instant::now();
    println!("Result (part 2): {} [{:?}]", part2(&input), start.elapsed());
}

fn input() -> ParsedInput {
    let input = fs::read_to_string("inputs/day-25.txt").unwrap();
    parse_input(&input)
}

fn parse_input(input: &str) -> ParsedInput {
    let mut keys = vec![];
    let mut locks = vec![];

    input.split("\n\n").for_each(|block| {
        let (header, block) = block.split_once("\n").unwrap();

        if header.starts_with("#") {
            locks.push(parse_block(block));
        } else {
            keys.push(parse_block(block));
        }
    });

    System { locks, keys }
}
fn parse_block(block: &str) -> Vec<u8> {
    let lines: Vec<String> = block.lines().map(str::to_string).collect();

    let chars: Vec<Vec<char>> = lines
        .iter()
        .take(lines.len() - 1)
        .map(|l| l.chars().collect())
        .collect();
    let width = chars[0].len();

    let mut result = vec![];

    for tumbler_idx in 0..width {
        result.push(
            (0..chars.len())
                .map(|i| chars[i][tumbler_idx])
                .filter(|c| *c == '#')
                .count()
                .try_into()
                .unwrap(),
        );
    }

    result
}

fn fit(lock: &[u8], key: &[u8]) -> bool {
    if lock.len() != key.len() {
        panic!("Lock and key are not the same size");
    }

    for i in 0..lock.len() {
        if lock[i] + key[i] > 5 {
            return false;
        }
    }

    true
}

fn part1(input: &ParsedInput) -> Output {
    debugln!("{:?}", input);

    iproduct!(input.locks.iter(), input.keys.iter())
        .filter(|(lock, key)| fit(lock, key))
        .count()
}

fn part2(input: &ParsedInput) -> Output {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        parse_input(
            r#"
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
        "#
            .trim(),
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 0);
    }
}
