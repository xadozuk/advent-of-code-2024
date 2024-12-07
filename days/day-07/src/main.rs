use std::{fmt::Debug, fs, time::Instant};

use itertools::Itertools;

type ParsedInput = Vec<Equation>;
type Output = u64;

#[derive(Debug)]
struct Equation {
    result: u64,
    operands: Vec<u64>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

impl Operator {
    const PART1: [Self; 2] = [Self::Add, Self::Multiply];
    const PART2: [Self; 3] = [Self::Add, Self::Multiply, Self::Concat];
}

impl Debug for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Multiply => write!(f, "*"),
            Self::Concat => write!(f, "||"),
        }
    }
}

fn main() {
    let input = input();

    let start = Instant::now();
    println!("Result (part 1): {} [{:?}]", part1(&input), start.elapsed());

    let start = Instant::now();
    println!("Result (part 2): {} [{:?}]", part2(&input), start.elapsed());
}

fn input() -> ParsedInput {
    let input = fs::read_to_string("inputs/day-07.txt").unwrap();
    parse_input(&input)
}

fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|line| {
            let (result_str, operands_str) = line.split_once(':').unwrap();

            Equation {
                result: result_str.parse().unwrap(),
                operands: operands_str
                    .trim()
                    .split(' ')
                    .map(|op| op.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

fn part1(input: &ParsedInput) -> Output {
    input
        .iter()
        .filter(|eq| solve_equation(eq, &Operator::PART1).is_some())
        .map(|eq| eq.result)
        .sum()
}

fn part2(input: &ParsedInput) -> Output {
    let mut not_solvable_eqs_with_p1: Vec<&Equation> = Vec::with_capacity(input.len() / 2);
    let mut result = 0;

    for equation in input {
        if solve_equation(equation, &Operator::PART1).is_some() {
            result += equation.result;
        } else {
            not_solvable_eqs_with_p1.push(equation);
        }
    }

    for equation in not_solvable_eqs_with_p1 {
        if solve_equation(equation, &Operator::PART2).is_some() {
            result += equation.result;
        }
    }

    result
}

fn solve_equation(equation: &Equation, operator_list: &[Operator]) -> Option<Vec<Operator>> {
    (0..(equation.operands.len()))
        .map(|_| operator_list.to_vec())
        .multi_cartesian_product()
        .find(|operators| equation.result == compute(equation, operators))
}

fn compute(equation: &Equation, operators: &[Operator]) -> u64 {
    let first_operand = equation.operands[0];
    equation.operands.iter().skip(1).zip(operators.iter()).fold(
        first_operand,
        |acc, (operand, operator)| match operator {
            Operator::Add => acc + operand,
            Operator::Multiply => acc * operand,
            Operator::Concat => acc * (10u64.pow(operand.ilog10() + 1)) + operand,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        parse_input(
            r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#
            .trim(),
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 11387);
    }
}
