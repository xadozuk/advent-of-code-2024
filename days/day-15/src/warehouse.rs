use std::{collections::HashSet, ops::Add};

use colored::Colorize;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl From<(i64, i64)> for Point {
    fn from((x, y): (i64, i64)) -> Self {
        Point { x, y }
    }
}

impl From<Direction> for Point {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => (-1, 0).into(),
            Direction::Down => (1, 0).into(),
            Direction::Left => (0, -1).into(),
            Direction::Right => (0, 1).into(),
        }
    }
}

impl From<&Direction> for Point {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Up => (-1, 0).into(),
            Direction::Down => (1, 0).into(),
            Direction::Left => (0, -1).into(),
            Direction::Right => (0, 1).into(),
        }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '<' => Direction::Left,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '^' => Direction::Up,
            _ => panic!("Unsupported move"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Wall,
    Free,
    Box,
    WideBox(Point),
}

#[derive(Clone)]
pub struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    pub fn at(&self, pos: &Point) -> Option<Tile> {
        if pos.x < 0
            || pos.y < 0
            || pos.x as usize >= self.tiles.len()
            || pos.y as usize >= self.tiles[0].len()
        {
            return None;
        }

        Some(self.tiles[pos.x as usize][pos.y as usize])
    }

    pub fn at_mut(&mut self, pos: &Point) -> Option<&mut Tile> {
        if pos.x < 0
            || pos.y < 0
            || pos.x as usize >= self.tiles.len()
            || pos.y as usize >= self.tiles[0].len()
        {
            return None;
        }

        Some(&mut self.tiles[pos.x as usize][pos.y as usize])
    }
}

#[derive(Clone)]
pub struct Warehouse {
    grid: Grid,
    movements: Vec<Direction>,
    robot_position: Point,
}

impl Warehouse {
    pub fn run_robot(&mut self) {
        let movements = self.movements.clone();

        for m in movements {
            self.move_robot(m);
        }
    }

    fn move_robot(&mut self, m: Direction) {
        let new_pos: Point = self.robot_position + m.into();
        match self.grid.at(&new_pos) {
            Some(Tile::Free) => self.robot_position = new_pos,
            Some(Tile::Box) | Some(Tile::WideBox(_)) => {
                if self.push_box(&new_pos, &m) {
                    self.robot_position = new_pos;
                }
            }
            _ => {}
        }
    }

    fn push_box(&mut self, pos: &Point, direction: &Direction) -> bool {
        let next_pos = *pos + direction.into();

        let current_tile = self.grid.at(pos).unwrap();

        // No-op, used to simplify wide box computation
        if current_tile == Tile::Free {
            return true;
        } else if current_tile == Tile::Box {
            return match self.grid.at(&next_pos) {
                Some(Tile::Free) => {
                    *self.grid.at_mut(pos).unwrap() = Tile::Free;
                    *self.grid.at_mut(&next_pos).unwrap() = Tile::Box;

                    true
                }
                Some(Tile::Box) => {
                    if self.can_push_box(&next_pos, direction) {
                        self.push_box(&next_pos, direction);

                        *self.grid.at_mut(pos).unwrap() = Tile::Free;
                        *self.grid.at_mut(&next_pos).unwrap() = Tile::Box;

                        true
                    } else {
                        false
                    }
                }
                _ => false,
            };
        } else if let Tile::WideBox(box_opp_pos) = current_tile {
            let box_pos = pos;

            let box_next_pos = next_pos;
            let box_opp_next_pos = box_opp_pos + direction.into();

            if self.can_push_box(box_pos, direction) && self.can_push_box(&box_opp_pos, direction) {
                // If we can push both side of box:
                // - Push next tiles (both)
                // - Changes current tiles

                // We only need to move if its another box
                if box_next_pos != box_opp_pos {
                    self.push_box(&box_next_pos, direction);
                }

                if box_opp_next_pos != *box_pos {
                    self.push_box(&box_opp_next_pos, direction);
                }

                // Free first and re-assign to avoid having to handle horizontal pushes (a box part
                // move into its opposite part)
                *self.grid.at_mut(box_pos).unwrap() = Tile::Free;
                *self.grid.at_mut(&box_opp_pos).unwrap() = Tile::Free;
                *self.grid.at_mut(&box_next_pos).unwrap() = Tile::WideBox(box_opp_next_pos);
                *self.grid.at_mut(&box_opp_next_pos).unwrap() = Tile::WideBox(box_next_pos);

                return true;
            }
        }

        false
    }

    fn can_push_box(&self, pos: &Point, direction: &Direction) -> bool {
        let next_pos = *pos + direction.into();

        match self.grid.at(&next_pos) {
            Some(Tile::Free) => true,
            Some(Tile::Box) => self.can_push_box(&next_pos, direction),
            Some(Tile::WideBox(opp_next_pos)) => {
                self.can_push_box(&next_pos, direction)
                    && (opp_next_pos == *pos || self.can_push_box(&opp_next_pos, direction))
            }

            _ => false,
        }
    }

    pub fn gps_sum(&self) -> u64 {
        self.grid
            .tiles
            .iter()
            .enumerate()
            .flat_map(|(i, l)| {
                l.iter()
                    .enumerate()
                    .filter(|(_, t)| **t == Tile::Box)
                    .map(move |(j, _)| 100 * i + j)
            })
            .sum::<usize>() as u64
    }

    pub fn wide_gps_sum(&self) -> u64 {
        let mut boxes: HashSet<(Point, Point)> = HashSet::new();

        for (i, l) in self.grid.tiles.iter().enumerate() {
            for (j, t) in l.iter().enumerate() {
                let pos = (i as i64, j as i64).into();
                if let Tile::WideBox(opp_pos) = t {
                    if !boxes.contains(&(pos, *opp_pos)) && !boxes.contains(&(*opp_pos, pos)) {
                        boxes.insert((pos, *opp_pos));
                    }
                }
            }
        }

        let middle = self.grid.tiles[0].len() as i64;

        boxes
            .iter()
            .map(|(a, b)| {
                if a.y <= middle {
                    100 * a.x + a.y
                } else {
                    100 * b.x + b.y
                }
            })
            .sum::<i64>() as u64
    }

    pub fn from(input: &str, wide: bool) -> Self {
        let (grid, moves) = input.split_once("\n\n").unwrap();
        let mut robot_starting_pos = (0, 0).into();

        let tiles = grid
            .lines()
            .enumerate()
            .map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .flat_map(|(j, c)| match c {
                        '#' => {
                            if wide {
                                vec![Tile::Wall, Tile::Wall]
                            } else {
                                vec![Tile::Wall]
                            }
                        }
                        'O' => {
                            if wide {
                                vec![
                                    Tile::WideBox((i as i64, j as i64 * 2 + 1).into()),
                                    Tile::WideBox((i as i64, j as i64 * 2).into()),
                                ]
                            } else {
                                vec![Tile::Box]
                            }
                        }
                        '@' => {
                            if wide {
                                robot_starting_pos = (i as i64, j as i64 * 2).into();
                                vec![Tile::Free, Tile::Free]
                            } else {
                                robot_starting_pos = (i as i64, j as i64).into();
                                vec![Tile::Free]
                            }
                        }
                        _ => {
                            if wide {
                                vec![Tile::Free, Tile::Free]
                            } else {
                                vec![Tile::Free]
                            }
                        }
                    })
                    .collect()
            })
            .collect();

        Warehouse {
            grid: Grid { tiles },
            movements: moves
                .chars()
                .filter(|c| *c != '\n')
                .map(Direction::from)
                .collect(),
            robot_position: robot_starting_pos,
        }
    }
}

impl From<&str> for Warehouse {
    fn from(input: &str) -> Self {
        Self::from(input, false)
    }
}

impl std::fmt::Debug for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, l) in self.grid.tiles.iter().enumerate() {
            write!(f, "{:02}| ", i)?;

            for (j, t) in l.iter().enumerate() {
                let char = if self.robot_position == (i as i64, j as i64).into() {
                    "@".bold().red()
                } else {
                    match t {
                        Tile::Free => ".".into(),
                        Tile::Box => "O".into(),
                        Tile::Wall => "#".into(),
                        Tile::WideBox(opposite_box) => {
                            if j < opposite_box.y as usize {
                                "[".into()
                            } else {
                                "]".into()
                            }
                        }
                    }
                };

                write!(f, "{}", char)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
