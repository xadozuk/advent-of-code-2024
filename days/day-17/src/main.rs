use std::{fs, time::Instant};

use cpu::Cpu;

type ParsedInput = Cpu;
type Output = String;

mod cpu;
mod debug;

fn main() {
    let input = input();

    let start = Instant::now();
    println!("Result (part 1): {} [{:?}]", part1(&input), start.elapsed());

    let start = Instant::now();
    println!("Result (part 2): {} [{:?}]", part2(&input), start.elapsed());
}

fn input() -> ParsedInput {
    let input = fs::read_to_string("inputs/day-17.txt").unwrap();
    parse_input(&input)
}

fn parse_input(input: &str) -> ParsedInput {
    Cpu::from(input)
}

fn part1(input: &ParsedInput) -> Output {
    let mut cpu = input.clone();

    let stdout = cpu.run();

    stdout
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn part2(input: &ParsedInput) -> usize {
    let mut cpu = input.clone();

    cpu.find_program_as_stdout() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        parse_input(
            r#"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
            "#
            .trim(),
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        let cpu = parse_input(
            r#"
            Register A: 729
            Register B: 0
            Register C: 0

            Program: 0,3,5,4,3,0 
                        "#
            .trim(),
        );

        assert_eq!(part2(&cpu), 117440);
    }
}
