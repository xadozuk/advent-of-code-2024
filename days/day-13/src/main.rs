use std::{fs, time::Instant};

use game::ClawMachine;
use regex::Regex;

mod game;

type ParsedInput = Vec<ClawMachine>;
type Output = u64;

fn main() {
    let input = input();

    let start = Instant::now();
    println!("Result (part 1): {} [{:?}]", part1(&input), start.elapsed());

    let start = Instant::now();
    println!("Result (part 2): {} [{:?}]", part2(&input), start.elapsed());
}

fn input() -> ParsedInput {
    let input = fs::read_to_string("inputs/day-13.txt").unwrap();
    parse_input(&input)
}

fn parse_input(input: &str) -> ParsedInput {
    let button_regex = Regex::new(r"^Button .: X(?<x>(\-|\+\d+)), Y(?<y>(\-|\+)\d+)$").unwrap();
    let prize_regex = Regex::new(r"^Prize: X=(?<x>\d+), Y=(?<y>\d+)$").unwrap();

    input
        .split("\n\n")
        .map(|block| {
            let lines: Vec<&str> = block.split("\n").collect();

            let Some(button_a) = button_regex.captures(lines[0]) else {
                panic!("Invalid input")
            };

            let Some(button_b) = button_regex.captures(lines[1]) else {
                panic!("Invalid input")
            };

            let Some(prize) = prize_regex.captures(lines[2]) else {
                panic!("Invalid input")
            };

            ClawMachine::new(
                (prize["x"].parse().unwrap(), prize["y"].parse().unwrap()).into(),
                (
                    button_a["x"].parse().unwrap(),
                    button_a["y"].parse().unwrap(),
                )
                    .into(),
                (
                    button_b["x"].parse().unwrap(),
                    button_b["y"].parse().unwrap(),
                )
                    .into(),
            )
        })
        .collect()
}

fn part1(input: &ParsedInput) -> Output {
    input
        .iter()
        .filter_map(|g| g.solve())
        .map(|s| (s.x * ClawMachine::A_TOKEN + s.y * ClawMachine::B_TOKEN) as u64)
        .sum()
}

fn part2(input: &ParsedInput) -> Output {
    input
        .iter()
        .filter_map(|g| g.solve_with_converstion())
        .map(|s| (s.x * ClawMachine::A_TOKEN + s.y * ClawMachine::B_TOKEN) as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        parse_input(
            r#"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
            "#
            .trim(),
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 480);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 0);
    }
}
