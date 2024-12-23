#[allow(unused_imports)]
use std::{collections::BTreeMap, fs, time::Instant};

#[allow(unused_imports)]
use lib::debugln;
use maze::Maze;

type ParsedInput = Maze;
type Output = u32;

mod maze;

fn main() {
    let input = input();

    let start = Instant::now();
    println!("Result (part 1): {} [{:?}]", part1(&input), start.elapsed());

    let start = Instant::now();
    println!("Result (part 2): {} [{:?}]", part2(&input), start.elapsed());
}

fn input() -> ParsedInput {
    let input = fs::read_to_string("inputs/day-20.txt").unwrap();
    parse_input(&input)
}

fn parse_input(input: &str) -> ParsedInput {
    Maze::new(input)
}

fn part1(input: &ParsedInput) -> Output {
    // let mut cheats = input.find_all_2ps_cheats();
    let cheats = input.find_all_cheats(2);

    cheats.iter().filter(|c| c.time_saved >= 100).count() as u32
}

fn part2(input: &ParsedInput) -> Output {
    let cheats = input.find_all_cheats(20);

    // let mut cheats_by_time: BTreeMap<u64, u64> = BTreeMap::new();
    //
    // for c in cheats.iter() {
    //     *cheats_by_time.entry(c.time_saved).or_insert(0) += 1;
    // }
    //
    // for g in cheats_by_time {
    //     debugln!("{} saved: {}", g.0, g.1);
    // }

    cheats.iter().filter(|c| c.time_saved >= 100).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        parse_input(
            r#"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
            "#
            .trim(),
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 0);
    }
}
