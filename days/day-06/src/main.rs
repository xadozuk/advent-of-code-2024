use core::panic;
use std::{collections::HashSet, fs, time::Instant};

type ParsedInput = (Point, Map);
type Output = u32;

type Map = Vec<Vec<Tile>>;
type Point = (i32, i32);

const UP: Point = (-1, 0);
const DOWN: Point = (1, 0);
const LEFT: Point = (0, -1);
const RIGHT: Point = (0, 1);

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Tile {
    Block,
    Free,
}

fn main() {
    let input = input();

    let start = Instant::now();
    println!("Result (part 1): {} [{:?}]", part1(&input), start.elapsed());

    let start = Instant::now();
    println!("Result (part 2): {} [{:?}]", part2(&input), start.elapsed());
}

fn input() -> ParsedInput {
    let input = fs::read_to_string("inputs/day-06.txt").unwrap();
    parse_input(&input)
}

fn parse_input(input: &str) -> ParsedInput {
    let mut starting_pos = Point::default();

    let map = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '#' => Tile::Block,
                    '^' => {
                        starting_pos = (i as i32, j as i32);
                        Tile::Free
                    }
                    _ => Tile::Free,
                })
                .collect()
        })
        .collect();

    (starting_pos, map)
}

fn part1(input: &ParsedInput) -> Output {
    do_patrol(input).len() as u32
}

fn do_patrol(input: &ParsedInput) -> HashSet<Point> {
    let mut direction = UP;
    let mut current_pos = input.0;
    let map = &input.1;

    let mut traversed_tiles: HashSet<Point> = HashSet::new();
    traversed_tiles.insert(current_pos);

    loop {
        let (next_pos, tile, steps) = move_to_next_obstacle(current_pos, direction, map);

        traversed_tiles.extend(steps.iter());

        if tile.is_none() {
            break;
        }

        current_pos = next_pos;

        direction = match direction {
            UP => RIGHT,
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
            _ => panic!("Unsupported"),
        };
    }

    traversed_tiles
}

fn part2(input: &ParsedInput) -> Output {
    let patrol_path = do_patrol(input);

    let starting_pos = input.0;
    let map = &input.1;
    let mut looping_config = 0;

    for tile_pos in patrol_path {
        if tile_pos == starting_pos {
            continue;
        }

        if let Some(Tile::Free) = tile_at(tile_pos, map) {
            let mut new_map = map.clone();
            new_map[tile_pos.0 as usize][tile_pos.1 as usize] = Tile::Block;

            if is_patrol_looping(starting_pos, &new_map) {
                looping_config += 1;
            }
        }
    }

    looping_config
}

fn is_patrol_looping(starting_pos: Point, map: &Map) -> bool {
    let mut direction = UP;
    let mut current_pos = starting_pos;

    let mut obstacle_hit_index: HashSet<(Point, Point)> = HashSet::new();

    loop {
        let (next_pos, tile, _) = move_to_next_obstacle(current_pos, direction, map);

        if tile.is_none() {
            return false;
        }

        if obstacle_hit_index.contains(&(next_pos, direction)) {
            return true;
        }

        obstacle_hit_index.insert((next_pos, direction));

        current_pos = next_pos;

        direction = match direction {
            UP => RIGHT,
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
            _ => panic!("Unsupported"),
        };
    }
}

fn move_to_next_obstacle(
    pos: Point,
    direction: Point,
    map: &Map,
) -> (Point, Option<Tile>, HashSet<Point>) {
    let mut current_pos = pos;
    let mut traversed_tiles: HashSet<Point> = HashSet::new();
    loop {
        let next_pos = move_to(current_pos, direction);
        let tile = tile_at(next_pos, map);

        traversed_tiles.insert(current_pos);

        match tile {
            Some(Tile::Free) => {
                current_pos = next_pos;
            }
            Some(Tile::Block) => return (current_pos, Some(Tile::Block), traversed_tiles),
            None => return (current_pos, None, traversed_tiles),
        }
    }
}

fn move_to(pos: Point, direction: Point) -> Point {
    (pos.0 + direction.0, pos.1 + direction.1)
}

fn tile_at(pos: Point, map: &Map) -> Option<Tile> {
    if pos.0 < 0
        || pos.1 < 0
        || pos.0 >= map.len() as i32
        || pos.1 >= map.first().map(|v| v.len()).unwrap_or(0) as i32
    {
        None
    } else {
        Some(map[pos.0 as usize][pos.1 as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        let input = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#
        .trim();

        parse_input(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 6);
    }
}
