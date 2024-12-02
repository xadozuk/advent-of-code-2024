use std::{collections::HashMap, fs};

type ParsedInput = Vec<Vec<u32>>;
type Output = u32;

enum Direction {
    Increase,
    Decrease,
    Unknown,
}

fn main() {
    let input = input();

    println!("Result (part 1): {}", part1(&input));
    println!("Result (part 2): {}", part2(&input));
}

fn input() -> ParsedInput {
    let input = fs::read_to_string("inputs/day-02.txt").unwrap();

    input
        .lines()
        .map(|line| line.split(" ").map(|str| str.parse().unwrap()).collect())
        .collect()
}

fn part1(input: &ParsedInput) -> Output {
    input.iter().filter(|report| is_safe(report, 0)).count() as u32
}

fn part2(input: &ParsedInput) -> Output {
    input.iter().filter(|report| is_safe(report, 1)).count() as u32
}

fn is_safe(report: &[u32], tolerance: u32) -> bool {
    if tolerance > 0 {
        if is_safe(report, 0) {
            return true;
        }

        for skip_index in 0..report.len() {
            let sub_report: Vec<u32> = report
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != skip_index)
                .map(|(_, &level)| level)
                .collect();

            if is_safe(&sub_report, tolerance - 1) {
                return true;
            }
        }

        return false;
    }

    let mut direction = Direction::Unknown;
    let mut prev = report[0];

    for level in report.iter().skip(1) {
        let mut error = false;
        let diff = *level as i32 - prev as i32;
        if !(1..=3).contains(&diff.abs()) {
            return false;
        }

        match direction {
            Direction::Unknown => {
                direction = if diff > 0 {
                    Direction::Increase
                } else {
                    Direction::Decrease
                };
            }
            Direction::Increase => {
                if diff <= 0 {
                    return false;
                }
            }
            Direction::Decrease => {
                if diff >= 0 {
                    return false;
                }
            }
        }
        prev = *level;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 4);
    }
}
