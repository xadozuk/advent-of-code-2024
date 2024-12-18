use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
    time::Instant,
};

use colored::Colorize;
use lib::{debug, debugln, Direction, Grid2d, Point, CARDINAL_DIRECTIONS};

type ParsedInput = Maze;
type Output = u64;

#[derive(PartialEq, Eq)]
enum Tile {
    Wall,
    Free,
}

struct Maze {
    grid: Grid2d<Tile>,
    start_position: Point,
    end_position: Point,
}

impl Maze {
    pub fn from(input: &str) -> Self {
        let mut start_position = (-1, -1).into();
        let mut end_position = (-1, -1).into();

        let grid = input
            .lines()
            .enumerate()
            .map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '#' => Tile::Wall,
                        '.' => Tile::Free,
                        'S' => {
                            start_position = (i as i64, j as i64).into();
                            Tile::Free
                        }
                        'E' => {
                            end_position = (i as i64, j as i64).into();
                            Tile::Free
                        }
                        _ => panic!("Invalid tile '{}'", c),
                    })
                    .collect()
            })
            .collect();

        Self {
            grid: Grid2d::new(grid),
            start_position,
            end_position,
        }
    }

    pub fn find_shortest_paths(&self) -> Option<Vec<Vec<Point>>> {
        let shortest_path = self.find_shortest_path().unwrap();

        let max_distance = shortest_path.len();
        let max_cost = shortest_path.last().unwrap().2;

        display_maze_with_path(self, &[]);

        self._find_shortest_paths(
            &self.start_position,
            Direction::Right,
            0,
            vec![],
            &mut HashMap::new(),
            max_distance,
            max_cost,
        )
    }

    fn _find_shortest_paths(
        &self,
        pos: &Point,
        current_direction: Direction,
        cost: u64,
        path: Vec<Point>,
        visited: &mut HashMap<Point, u64>,
        max_distance: usize,
        max_cost: u64,
    ) -> Option<Vec<Vec<Point>>> {
        // let mut visited = visited;

        if cost > max_cost || path.len() > max_distance {
            debugln!(
                "! Nope (cost: {}, distance: {})",
                cost > max_cost,
                path.len() > max_distance,
            );
            return None;
        }

        if visited.contains_key(pos) && visited[pos] + 1001 < cost {
            return None;
        }

        visited.insert(*pos, cost);

        let mut path = path;
        path.push(*pos);

        if *pos == self.end_position {
            debugln!("Found the end with d = {}", cost);
            return Some(vec![path]);
        };

        // debugln!("At {:?}/{} (path: {:?})", pos, cost, path);

        debugln!("{:?}", pos);

        let mut new_paths = vec![];

        for (n_pos, n_dir) in self.neighboors_at(pos) {
            let n_cost = if n_dir == current_direction { 1 } else { 1001 };

            debugln!(
                "{:?} checking neighoor ({:?}) {:?} -> {}, {}",
                pos,
                n_dir,
                n_pos,
                cost + n_cost,
                path.len()
            );

            let paths: Vec<Vec<Point>> = self
                ._find_shortest_paths(
                    &n_pos,
                    n_dir,
                    cost + n_cost,
                    path.clone(),
                    visited,
                    max_distance,
                    max_cost,
                )
                .into_iter()
                .flatten()
                .collect();

            if paths.is_empty() {
                debugln!("No valid subpaths");
                continue;
            }

            for p in paths {
                new_paths.push(p);
            }
        }

        debugln!("<-");
        if new_paths.is_empty() {
            None
        } else {
            Some(new_paths)
        }
    }

    fn dijkstra(&self) -> Option<(HashMap<Point, u64>, HashMap<Point, Point>)> {
        let start = (self.start_position, Direction::Right);

        let mut visited: HashSet<Point> = HashSet::new();
        let mut to_visit: BinaryHeap<Reverse<(u64, (Point, Direction))>> = BinaryHeap::new();
        let mut distances: HashMap<Point, u64> = HashMap::new();
        let mut reverse_path: HashMap<Point, Point> = HashMap::new();

        to_visit.push(Reverse((0, start)));
        distances.insert(self.start_position, 0);

        while let Some(Reverse((_, (pos, direction)))) = to_visit.pop() {
            debugln!("Visiting: {:?} ({:?})", pos, direction);

            if visited.contains(&pos) {
                debugln!("{}", "\t! Already visited".red());
                continue;
            }

            visited.insert(pos);

            let current_distance = *distances.get(&pos).unwrap_or(&u64::MAX);

            debugln!("\tDistance from start: {}", current_distance);

            for (neighboor_pos, neighboor_dir) in self.neighboors_at(&pos) {
                let existing_neighboor_distance =
                    *distances.get(&neighboor_pos).unwrap_or(&u64::MAX);

                let turn_cost = if direction == neighboor_dir {
                    0
                } else {
                    direction.turns_from(&neighboor_dir) as u64 * 1000
                };

                let new_neighboor_distance = current_distance.saturating_add(1 + turn_cost);

                debugln!(
                    "\tNeighboor {:?} ({:?}): ({}, {})",
                    neighboor_pos,
                    neighboor_dir,
                    existing_neighboor_distance,
                    new_neighboor_distance
                );

                if new_neighboor_distance <= existing_neighboor_distance {
                    debugln!(
                        "{}",
                        "\t\tFound shorter distance for this neighboor".green()
                    );

                    *distances.entry(neighboor_pos).or_insert(u64::MAX) = new_neighboor_distance;
                    reverse_path.insert(neighboor_pos, pos);

                    to_visit.push(Reverse((
                        new_neighboor_distance,
                        (neighboor_pos, neighboor_dir),
                    )));
                } else {
                    debugln!(
                        "{}",
                        "\t\t! Already have a shorter path for this neighboor".yellow()
                    );
                }
            }
        }

        if !visited.contains(&self.end_position) {
            return None;
        }

        Some((distances, reverse_path))
    }

    pub fn find_shortest_path(&self) -> Option<Vec<(Point, Direction, u64)>> {
        let (distances, reverse_path) = self.dijkstra()?;

        let mut current = self.end_position;
        let mut path: Vec<(Point, Direction, u64)> = Vec::with_capacity(reverse_path.len());

        while current != self.start_position {
            let previous = reverse_path[&current];

            path.push((current, (current - previous).into(), distances[&current]));

            current = previous;
        }

        path.push((self.start_position, Direction::Right, 0));

        Some(path.into_iter().rev().collect())
    }

    fn neighboors_at(&self, pos: &Point) -> Vec<(Point, Direction)> {
        CARDINAL_DIRECTIONS
            .iter()
            .map(|dir| {
                let new_pos: Point = *pos + dir.into();
                if let Some(Tile::Free) = self.grid.at(&new_pos) {
                    Some((new_pos, *dir))
                } else {
                    None
                }
            })
            .filter(Option::is_some)
            .flatten()
            .collect()
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
    let input = fs::read_to_string("inputs/day-16.txt").unwrap();
    parse_input(&input)
}

fn parse_input(input: &str) -> ParsedInput {
    Maze::from(input)
}

fn display_maze_with_path(maze: &Maze, path: &[(Point, Direction, u64)]) {
    for (i, l) in maze.grid.iter() {
        for (j, t) in l.iter() {
            let pos: Point = (i as i64, j as i64).into();

            if *t == Tile::Wall {
                debug!("#");
            } else if pos == maze.start_position {
                debug!("{}", "S".yellow());
            } else if pos == maze.end_position {
                debug!("{}", "E".yellow());
            } else if let Some((_, dir, _)) = path.iter().find(|(p, _, _)| *p == pos) {
                let dir_char = match dir {
                    Direction::Up => "^",
                    Direction::Down => "v",
                    Direction::Left => "<",
                    Direction::Right => ">",
                };

                debug!("{}", dir_char.green());
            } else {
                debug!(".")
            }
        }

        debugln!();
    }
}

fn part1(input: &ParsedInput) -> Output {
    let path = input.find_shortest_path().unwrap();

    display_maze_with_path(input, &path);

    path.last().unwrap().2
}

fn part2(input: &ParsedInput) -> Output {
    let paths = input.find_shortest_paths().unwrap();

    for p in paths.iter() {
        debugln!("{:?}", p);
    }

    let unique_pos: HashSet<Point> = paths.into_iter().flatten().collect();

    unique_pos.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        parse_input(
            r#"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
            "#
            .trim(),
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 7036);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 45);
    }

    #[test]
    fn custom() {
        part2(&parse_input(
            r#"
#####
#E..#
#.###
#...#
#.#.#
#...#
###.#
#..S#
#####
            "#
            .trim(),
        ));
    }
}
