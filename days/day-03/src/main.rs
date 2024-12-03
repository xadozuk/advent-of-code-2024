use core::panic;
use std::{fs, time::Instant};

use regex::Regex;

type ParsedInput = String;
type Output = u32;

fn main() {
    let input = input();

    let start = Instant::now();
    println!("Result (part 1): {} [{:?}]", part1(&input), start.elapsed());

    let start = Instant::now();
    println!("Result (part 2): {} [{:?}]", part2(&input), start.elapsed());
}

fn input() -> ParsedInput {
    let input = fs::read_to_string("inputs/day-03.txt").unwrap();
    input
}

fn part1(input: &ParsedInput) -> Output {
    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    regex.captures_iter(input).fold(0, |result, caps| {
        let left = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let right = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();

        result + left * right
    })
}

fn part2(input: &ParsedInput) -> Output {
    let regex = Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]+),([0-9]+)\)").unwrap();

    let result = regex.captures_iter(input).fold((0, true), |result, caps| {
        let func = caps.get(0).unwrap().as_str().split('(').next().unwrap();

        match func {
            "do" => (result.0, true),
            "don't" => (result.0, false),
            "mul" => {
                let mut sum = result.0;
                if result.1 {
                    let left = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    let right = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
                    sum += left * right;
                }

                (sum, result.1)
            }
            _ => panic!("Unsupported function"),
        }
    });

    result.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_owned()
    }

    fn input2() -> ParsedInput {
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_owned()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input2()), 48);
    }
}
