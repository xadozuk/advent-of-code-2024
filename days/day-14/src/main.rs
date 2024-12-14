use std::{fs, time::Instant};

use grid::{Grid, Point, Robot};

type ParsedInput = Vec<Robot>;
type Output = u32;

mod grid;

const GRID_SIZE: Point = Point { x: 101, y: 103 };

fn main() {
    let input = input();

    let start = Instant::now();
    println!("Result (part 1): {} [{:?}]", part1(&input), start.elapsed());

    let start = Instant::now();
    println!("Result (part 2): {} [{:?}]", part2(&input), start.elapsed());
}

fn input() -> ParsedInput {
    let input = fs::read_to_string("inputs/day-14.txt").unwrap();
    parse_input(&input)
}

fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|line| Robot::parse(line).unwrap())
        .collect()
}

fn part1(input: &ParsedInput) -> Output {
    safety_factor(input, &GRID_SIZE)
}

fn part2(input: &ParsedInput) -> Output {
    let grid = Grid::new(GRID_SIZE, input.clone());

    grid.find_christmas_tree()
}

fn safety_factor(input: &ParsedInput, grid_size: &Point) -> Output {
    let grid = Grid::new(*grid_size, input.clone());

    grid.safety_factor_after(100)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        parse_input(
            r#"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
        "#
            .trim(),
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(safety_factor(&input(), &(11, 7).into()), 12);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 0);
    }
}
