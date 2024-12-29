use std::ops::{Add, AddAssign, Sub};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn parse((x, y): (&str, &str)) -> Result<Point, std::num::ParseIntError> {
        Ok(Point {
            x: x.parse::<i64>()?,
            y: y.parse::<i64>()?,
        })
    }
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
            Direction::UpLeft => (-1, -1).into(),
            Direction::UpRight => (-1, 1).into(),
            Direction::DownLeft => (1, -1).into(),
            Direction::DownRight => (1, 1).into(),
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
            Direction::UpLeft => (-1, -1).into(),
            Direction::UpRight => (-1, 1).into(),
            Direction::DownLeft => (1, -1).into(),
            Direction::DownRight => (1, 1).into(),
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

impl Add<Point> for &Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn turns_from(&self, other_dir: &Direction) -> u8 {
        if self == other_dir {
            0
        } else if self.turn_around() == *other_dir {
            2
        } else {
            1
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            _ => panic!("Diagonal not supported"),
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Self::Down => Self::Right,
            Self::Right => Self::Up,
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            _ => panic!("Diagonal not supported"),
        }
    }

    pub fn turn_around(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            _ => panic!("Diagonal not supported"),
        }
    }

    pub fn parse(point: &Point) -> Option<Self> {
        match (point.x, point.y) {
            (-1, 0) => Some(Direction::Up),
            (1, 0) => Some(Direction::Down),
            (0, -1) => Some(Direction::Left),
            (0, 1) => Some(Direction::Right),
            _ => None,
        }
    }
}

pub const ALL_DIRECTIONS: [Direction; 8] = [
    Direction::UpLeft,
    Direction::Up,
    Direction::UpRight,
    Direction::Left,
    Direction::Right,
    Direction::DownLeft,
    Direction::Down,
    Direction::DownRight,
];

pub const CARDINAL_DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

impl From<Point> for Direction {
    fn from(value: Point) -> Self {
        Self::parse(&value).unwrap()
    }
}

impl From<&Point> for Direction {
    fn from(value: &Point) -> Self {
        Self::parse(value).unwrap()
    }
}
