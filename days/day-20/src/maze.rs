use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use lib::{Grid2d, Point};

#[derive(PartialEq, Eq)]
pub enum Tile {
    Wall,
    Free,
}

pub struct Maze {
    grid: Grid2d<Tile>,
    starting_pos: Point,
    end_pos: Point,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Cheat {
    pub start: Point,
    pub end: Point,
    pub time_saved: u64,
}

impl Maze {
    pub fn new(input: &str) -> Self {
        let mut start_pos = Point::default();
        let mut end_pos = Point::default();
        let grid = input
            .lines()
            .enumerate()
            .map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '#' => Tile::Wall,
                        'S' => {
                            start_pos = (i as i64, j as i64).into();
                            Tile::Free
                        }
                        'E' => {
                            end_pos = (i as i64, j as i64).into();
                            Tile::Free
                        }
                        '.' => Tile::Free,
                        _ => panic!("Unsupported tile"),
                    })
                    .collect()
            })
            .collect();

        Maze {
            grid: Grid2d::new(grid),
            starting_pos: start_pos,
            end_pos,
        }
    }

    pub fn find_all_cheats(&self, cheat_duration: u64) -> Vec<Cheat> {
        let distances_from_start = self.dijkstra(&self.starting_pos);
        let distances_from_end = self.dijkstra(&self.end_pos);
        let time_to_beat = distances_from_start[&self.end_pos];

        let all_points = self.all_points();
        let mut cheats = vec![];

        for a in all_points.iter() {
            for b in all_points.iter() {
                if a == b {
                    continue;
                }

                match (self.grid.at(a), self.grid.at(b)) {
                    (Some(Tile::Wall), _) | (_, Some(Tile::Wall)) => continue,
                    _ => {}
                }

                let delta = b - a;
                let steps = delta.x.unsigned_abs() + delta.y.unsigned_abs();

                if steps > cheat_duration {
                    continue;
                }

                let time_with_cheat = distances_from_start[a] + distances_from_end[b] + steps;

                if time_with_cheat < time_to_beat {
                    let c = Cheat {
                        start: *a,
                        end: *b,
                        time_saved: time_to_beat - time_with_cheat,
                    };

                    // debugln!("Found cheat: {:?}", c);

                    cheats.push(c);
                }
            }
        }

        cheats
    }

    fn all_points(&self) -> Vec<Point> {
        self.grid
            .iter()
            .flat_map(|(i, l)| {
                l.iter()
                    .map(move |(j, _)| Into::<Point>::into((i as i64, j as i64)))
                    .collect::<Vec<Point>>()
            })
            .collect()
    }

    fn dijkstra(&self, start: &Point) -> HashMap<Point, u64> {
        let mut visited: HashSet<Point> = HashSet::new();
        let mut to_visit: BinaryHeap<Reverse<(u64, Point)>> = BinaryHeap::new();
        let mut distances: HashMap<Point, u64> = HashMap::new();

        to_visit.push(Reverse((0, *start)));
        distances.insert(*start, 0);

        while let Some(Reverse((_, pos))) = to_visit.pop() {
            if visited.contains(&pos) {
                continue;
            }

            visited.insert(pos);

            let current_distance = *distances.get(&pos).unwrap_or(&u64::MAX);

            for (_, neighboor_pos, _) in self
                .grid
                .cardinal_neighboors(pos)
                .filter(|(t, _, _)| **t == Tile::Free)
            {
                let existing_neighboor_distance =
                    *distances.get(&neighboor_pos).unwrap_or(&u64::MAX);

                let new_neighboor_distance = current_distance.saturating_add(1);

                if new_neighboor_distance <= existing_neighboor_distance {
                    *distances.entry(neighboor_pos).or_insert(u64::MAX) = new_neighboor_distance;

                    to_visit.push(Reverse((new_neighboor_distance, neighboor_pos)));
                }
            }
        }

        distances
    }
}
