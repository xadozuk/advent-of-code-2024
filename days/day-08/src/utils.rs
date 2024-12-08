use std::{
    cmp,
    collections::{HashMap, HashSet},
    ops::{Add, Div, Mul, Sub},
};

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Frequency(pub char);

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Point(pub i32, pub i32);

pub struct Grid {
    width: usize,
    height: usize,
    // antennas: HashMap<Point, Frequency>,
    antennas_index: HashMap<Frequency, Vec<Point>>,
}

impl Grid {
    pub fn new(width: usize, height: usize, antennas: HashMap<Point, Frequency>) -> Self {
        let mut antennas_index = HashMap::<Frequency, Vec<Point>>::new();

        for (point, frequency) in antennas.iter() {
            antennas_index
                .entry(*frequency)
                .and_modify(|e| e.push(*point))
                .or_insert(vec![*point]);
        }

        Self {
            width,
            height,
            // antennas,
            antennas_index,
        }
    }

    pub fn antinodes(&self) -> HashSet<Point> {
        self.antennas_index
            .iter()
            .flat_map(|(_, points)| {
                points.iter().permutations(2).flat_map(|pair| {
                    let vector = *pair[1] - *pair[0];
                    vec![*pair[0] - vector, *pair[1] + vector]
                })
            })
            .filter(|p| self.is_in_bound(p))
            .collect()
    }

    pub fn real_antinodes(&self) -> HashSet<Point> {
        let width = self.width as i32;
        let height = self.height as i32;

        self.antennas_index
            .iter()
            .flat_map(|(_, points)| {
                points.iter().permutations(2).flat_map(|pair| {
                    let vector = *pair[1] - *pair[0];
                    let size = cmp::max(width.div(vector.0), height.div(vector.1)) + 1;

                    (0..size)
                        .flat_map(|n| vec![*pair[0] - vector * n, *pair[1] + vector * n])
                        .collect::<Vec<Point>>()
                })
            })
            .filter(|p| self.is_in_bound(p))
            .collect()
    }

    pub fn is_in_bound(&self, point: &Point) -> bool {
        point.0 >= 0
            && point.1 >= 0
            && (point.0 as usize) < self.width
            && (point.1 as usize) < self.height
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}
