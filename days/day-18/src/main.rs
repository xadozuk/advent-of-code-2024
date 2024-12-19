use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
    time::Instant,
};

use lib::{debugln, grid, Grid2d, Point};

type ParsedInput = Vec<Point>;
type Output = u64;

fn main() {
    let input = input();

    let start = Instant::now();
    println!("Result (part 1): {} [{:?}]", part1(&input), start.elapsed());

    let start = Instant::now();
    println!(
        "Result (part 2): {:?} [{:?}]",
        part2(&input),
        start.elapsed()
    );
}

fn input() -> ParsedInput {
    let input = fs::read_to_string("inputs/day-18.txt").unwrap();
    parse_input(&input)
}

fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|l| {
            let coord = l.split_once(",").unwrap();
            Point::parse((coord.1, coord.0)).unwrap()
        })
        .collect()
}

fn find_shortest_path(
    (start, end): (Point, Point),
    size: (usize, usize),
    obstacles: &HashSet<Point>,
) -> Option<Vec<Point>> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut to_visit: BinaryHeap<Reverse<(u64, Point)>> = BinaryHeap::new();
    let mut distances: HashMap<Point, u64> = HashMap::new();
    let mut reverse_path: HashMap<Point, Point> = HashMap::new();

    let grid = Grid2d::new(vec![vec![(); size.1]; size.0]);

    to_visit.push(Reverse((0, start)));
    distances.insert(start, 0);

    while let Some(Reverse((_, pos))) = to_visit.pop() {
        if visited.contains(&pos) {
            continue;
        }

        visited.insert(pos);

        let current_distance = *distances.get(&pos).unwrap_or(&u64::MAX);

        for (_, neighboor_pos, neighboor_dir) in grid
            .cardinal_neighboors(pos)
            .filter(|(_, n_pos, _)| !obstacles.contains(n_pos))
        {
            let existing_neighboor_distance = *distances.get(&neighboor_pos).unwrap_or(&u64::MAX);
            let new_neighboor_distance = current_distance.saturating_add(1);

            if new_neighboor_distance <= existing_neighboor_distance {
                *distances.entry(neighboor_pos).or_insert(u64::MAX) = new_neighboor_distance;
                reverse_path.insert(neighboor_pos, pos);

                to_visit.push(Reverse((new_neighboor_distance, neighboor_pos)));
            }
        }
    }

    if !visited.contains(&end) {
        return None;
    }

    let mut current = end;
    let mut path: Vec<Point> = Vec::with_capacity(reverse_path.len());

    while current != start {
        let previous = reverse_path[&current];

        path.push(current);

        current = previous;
    }

    path.push(start);

    Some(path.into_iter().rev().collect())
}

fn part1(input: &ParsedInput) -> Output {
    let grid_size = (71, 71);
    let start = (0, 0).into();
    let end = (70, 70).into();

    let bytes: HashSet<Point> = input.iter().take(1024).copied().collect();

    for i in 0..grid_size.0 {
        for j in 0..grid_size.1 {
            let pos = (i as i64, j as i64).into();

            if bytes.contains(&pos) {
                print!("#");
            } else if pos == start {
                print!("S");
            } else if pos == end {
                print!("E");
            } else {
                print!(".");
            }
        }

        println!();
    }

    let path = find_shortest_path((start, end), grid_size, &bytes).unwrap();

    (path.len() - 1) as u64
}

fn part2(input: &ParsedInput) -> Point {
    let grid_size = (71, 71);

    solve_part2(grid_size.into(), input, 1024).unwrap()
}

fn solve_part2(grid_size: Point, bytes: &[Point], start_at_bytes: usize) -> Option<Point> {
    let start: Point = (0, 0).into();
    let end: Point = grid_size - (1, 1).into();

    let mut path = find_shortest_path(
        (start, end),
        (grid_size.x as usize, grid_size.y as usize),
        &bytes.iter().take(start_at_bytes).copied().collect(),
    )
    .unwrap();

    for i in start_at_bytes..bytes.len() {
        // If new bytes are not part of the shortest path, we skip
        if !path.contains(&bytes[i]) {
            debugln!("Short-circuit");
            continue;
        }

        let bytes_set: HashSet<Point> = bytes.iter().take(i + 1).copied().collect();

        let new_path = find_shortest_path(
            (start, end),
            (grid_size.x as usize, grid_size.y as usize),
            &bytes_set,
        );

        if new_path.is_none() {
            debugln!("No solution found at i = {} ({:?})", i, bytes[i]);
            return Some(bytes[i]);
        }

        debugln!("Solution exists at i = {} ({:?})", i, bytes[i]);
        path = new_path.unwrap();
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        parse_input(
            r#"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"#
            .trim(),
        )
    }

    #[test]
    fn test_part1() {
        let bytes = input().iter().take(12).copied().collect();

        assert_eq!(
            find_shortest_path(((0, 0).into(), (6, 6).into()), (7, 7), &bytes)
                .unwrap()
                .len()
                - 1,
            22
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2((7, 7).into(), &input(), 0), Some((1, 6).into()));
    }
}
