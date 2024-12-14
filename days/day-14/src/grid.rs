use core::f64;
use std::{collections::HashMap, io::stdin, num::ParseIntError};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl From<(i64, i64)> for Point {
    fn from((x, y): (i64, i64)) -> Self {
        Point { x, y }
    }
}

impl Point {
    pub fn parse((x, y): (&str, &str)) -> Result<Self, ParseIntError> {
        Ok(Point {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

#[derive(Copy, Clone)]
pub struct Robot {
    starting_position: Point,
    velocity: Point,
}

impl Robot {
    pub fn parse(input: &str) -> Result<Self, String> {
        let (pos_str, veolicity_str) = input.split_once(" ").ok_or("Invalid input")?;

        let pos = pos_str.split_once("=").ok_or("Invalid position")?.1;
        let velocity = veolicity_str.split_once("=").ok_or("Invalid velocity")?.1;

        Ok(Robot {
            starting_position: Point::parse(pos.split_once(",").ok_or("Invalid position data")?)
                .map_err(|_| "Invalid position value".to_owned())?,
            velocity: Point::parse(velocity.split_once(",").ok_or("Invalid velocity data")?)
                .map_err(|_| "Invalid velocity value".to_owned())?,
        })
    }
}

pub struct Grid {
    size: Point,
    robots: Vec<Robot>,
}

impl Grid {
    pub fn new(size: Point, robots: Vec<Robot>) -> Self {
        Grid { size, robots }
    }

    pub fn safety_factor_after(&self, seconds: i64) -> u32 {
        let robot_positions = self.robot_positions_after(seconds);

        let robots_in_quadrant = robot_positions.iter().filter_map(|pos| self.quadrant(pos));

        let mut quadrants = HashMap::new();

        for q in robots_in_quadrant {
            *quadrants.entry(q).or_insert(0) += 1;
        }

        quadrants
            .values()
            .copied()
            .reduce(|acc, v| acc * v)
            .unwrap()
    }

    fn robot_positions_after(&self, seconds: i64) -> Vec<Point> {
        self.robots
            .iter()
            .map(|robot| {
                let mut x = (robot.starting_position.x
                    + robot.velocity.x * (seconds % self.size.x))
                    % self.size.x;
                let mut y = (robot.starting_position.y
                    + robot.velocity.y * (seconds % self.size.y))
                    % self.size.y;

                if x < 0 {
                    x += self.size.x
                }
                if y < 0 {
                    y += self.size.y
                }

                (x, y).into()
            })
            .collect()
    }

    fn quadrant(&self, pos: &Point) -> Option<u32> {
        let middle_x = self.size.x / 2;
        let middle_y = self.size.y / 2;

        if pos.x == middle_x || pos.y == middle_y {
            return None;
        }

        let left = (0..middle_x).contains(&pos.x);
        let top = (0..middle_y).contains(&pos.y);

        match (top, left) {
            (true, true) => Some(1),
            (true, false) => Some(2),
            (false, true) => Some(3),
            (false, false) => Some(4),
        }
    }

    fn debug(&self, robot_positions: &[Point]) {
        // println!("{:?}", robot_positions);

        println!();

        for j in 0..self.size.y {
            for i in 0..self.size.x {
                let point = (i, j).into();
                let count = robot_positions.iter().filter(|p| **p == point).count();

                if count == 0 {
                    print!(".");
                } else {
                    print!("{}", count);
                }
            }

            println!();
        }

        println!();
    }

    pub fn find_christmas_tree(&self) -> u32 {
        for n in 0..1_000_000 {
            let robot_positions = self.robot_positions_after(n);

            let all_x: Vec<f64> = robot_positions.iter().map(|p| p.x as f64).collect();
            let all_y: Vec<f64> = robot_positions.iter().map(|p| p.y as f64).collect();

            let mean_x: f64 = all_x.iter().sum::<f64>() / all_x.len() as f64;
            let mean_y: f64 = all_y.iter().sum::<f64>() / all_y.len() as f64;

            let v_x = all_x.iter().map(|x| (x - mean_x).powi(2)).sum::<f64>() / all_x.len() as f64;
            let v_y = all_y.iter().map(|y| (y - mean_y).powi(2)).sum::<f64>() / all_y.len() as f64;

            if v_x < 400f64 && v_y < 400f64 {
                // self.debug(&robot_positions);

                // println!("Grid: {}", n);
                // println!("Variance: X={}, Y={}", v_x, v_y);

                return n as u32;
            }
        }

        0
    }
}
