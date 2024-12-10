use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    ops::Add,
    time::Instant,
    usize,
};

#[derive(Hash, PartialEq, Eq, Clone)]
struct Point(i32, i32);

type ParsedInput = Vec<Vec<u32>>;
type Output = u32;

const UP: Point = Point(-1, 0);
const DOWN: Point = Point(1, 0);
const LEFT: Point = Point(0, -1);
const RIGHT: Point = Point(0, 1);

const DIRECTIONS: [Point; 4] = [UP, RIGHT, DOWN, LEFT];

fn main() {
    let input = input();
    let start = Instant::now();

    let result = trailheads(&input)
        .iter()
        .map(|t| score_and_rate_trailhead(&input, t))
        .reduce(|(acc_score, acc_rate), (score, rate)| (acc_score + score, acc_rate + rate))
        .unwrap();

    println!("Processing [{:?}]", start.elapsed());

    println!("Result (part 1): {}", result.0);
    println!("Result (part 2): {}", result.1);
}

fn input() -> ParsedInput {
    let input = fs::read_to_string("inputs/day-10.txt").unwrap();
    parse_input(&input)
}

fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn trailheads(input: &ParsedInput) -> Vec<Point> {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().map(move |(j, &cell)| {
                if cell == 0 {
                    Some(Point(i as i32, j as i32))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect()
}

fn score_and_rate_trailhead(grid: &ParsedInput, start: &Point) -> (u32, u32) {
    let mut result: HashMap<Point, u32> = HashMap::new();

    let mut queue: VecDeque<(Point, u32)> = VecDeque::new();
    queue.push_back((start.clone(), 0));

    while let Some((pos, height)) = queue.pop_back() {
        if height == 9 {
            result.entry(pos).and_modify(|e| *e += 1).or_insert(1);
            continue;
        }

        for dir in DIRECTIONS.iter() {
            let next_pos = &pos + dir;

            if let Some(next_height) = at(grid, &next_pos) {
                if next_height == height + 1 {
                    queue.push_back((next_pos, next_height))
                }
            }
        }
    }

    (result.len() as u32, result.values().sum())
}

fn at(input: &ParsedInput, pos: &Point) -> Option<u32> {
    if pos.0 < 0 || pos.1 < 0 || pos.0 as usize >= input.len() || pos.1 as usize >= input[0].len() {
        None
    } else {
        Some(input[pos.0 as usize][pos.1 as usize])
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        parse_input(
            r#"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
            "#
            .trim(),
        )
    }

    #[test]
    fn test_parts() {
        let input = input();
        let result = trailheads(&input)
            .iter()
            .map(|t| score_and_rate_trailhead(&input, t))
            .reduce(|(acc_score, acc_rate), (score, rate)| (acc_score + score, acc_rate + rate))
            .unwrap();

        assert_eq!(result.0, 36);
        assert_eq!(result.1, 81);
    }
}
