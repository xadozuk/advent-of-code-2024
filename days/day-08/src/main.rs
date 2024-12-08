use std::{collections::HashMap, fs, time::Instant};

use utils::{Frequency, Grid, Point};

mod utils;

type ParsedInput = Grid;
type Output = u32;

fn main() {
    let input = input();

    let start = Instant::now();
    println!("Result (part 1): {} [{:?}]", part1(&input), start.elapsed());

    let start = Instant::now();
    println!("Result (part 2): {} [{:?}]", part2(&input), start.elapsed());
}

fn input() -> ParsedInput {
    let input = fs::read_to_string("inputs/day-08.txt").unwrap();
    parse_input(&input)
}

fn parse_input(input: &str) -> ParsedInput {
    let mut antennas = HashMap::new();

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for (i, line) in grid.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char != '.' {
                antennas.insert(Point(i as i32, j as i32), Frequency(*char));
            }
        }
    }

    Grid::new(grid[0].len(), grid.len(), antennas)
}

fn part1(input: &ParsedInput) -> Output {
    input.antinodes().len() as u32
}

fn part2(input: &ParsedInput) -> Output {
    input.real_antinodes().len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        parse_input(
            r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#
            .trim(),
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 34);
    }
}
