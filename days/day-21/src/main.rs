use std::{fs, time::Instant};

use keypads::{DirectionalKeyPad, GlobalSequenceCache, Keypad, NumericKeypad, SequenceCache};
use lib::{debugln, Direction, Point};

mod keypads;

type ParsedInput = Vec<(String, u32)>;
type Output = u32;

fn main() {
    let input = input();

    let start = Instant::now();
    println!("Result (part 1): {} [{:?}]", part1(&input), start.elapsed());

    let start = Instant::now();
    println!("Result (part 2): {} [{:?}]", part2(&input), start.elapsed());
}

fn input() -> ParsedInput {
    let input = fs::read_to_string("inputs/day-21.txt").unwrap();
    parse_input(&input)
}

fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|code| {
            let numeric_part: u32 = code
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap();

            (code.to_string(), numeric_part)
        })
        .collect()
}

fn part1(input: &ParsedInput) -> Output {
    input
        .iter()
        .map(|(code, n)| find_shortest_sequence(code, 2).len() as u32 * n)
        .sum()
}

fn part2(input: &ParsedInput) -> Output {
    input
        .iter()
        .map(|(code, n)| find_shortest_sequence(code, 25).len() as u32 * n)
        .sum()
}

fn find_shortest_sequence(code: &str, n_directional_robots: u32) -> String {
    /*
        - One directional keypad that you are using.
        - Two directional keypads that robots are using.
        - One numeric keypad (on a door) that a robot is using.
    */
    let mut first_keypad = NumericKeypad::new();

    let mut seq = first_keypad.sequence_for(
        code,
        &mut SequenceCache::new(),
        &mut GlobalSequenceCache::new(),
    );

    if cfg!(debug_assertions) {
        assert_numeric_sequence(&seq);
    }

    debugln!("Code: {}", code);
    debugln!("Numeric: {}", seq);

    let mut cache = SequenceCache::new();
    let mut global_cache = GlobalSequenceCache::new();

    for i in 0..n_directional_robots {
        let mut keypad = DirectionalKeyPad::new();
        seq = keypad.sequence_for(&seq, &mut cache, &mut global_cache);

        if cfg!(debug_assertions) {
            assert_directional_sequence(&seq);
        }

        println!("Directional {}: Done", i + 1);
    }

    seq
}

fn assert_numeric_sequence(seq: &str) {
    let mut pos: Point = (3, 2).into();
    let impossible: Point = (3, 0).into();

    for c in seq.chars() {
        match c {
            '<' => {
                pos += Direction::Left.into();
            }
            '>' => {
                pos += Direction::Right.into();
            }
            '^' => {
                pos += Direction::Up.into();
            }
            'v' => {
                pos += Direction::Down.into();
            }
            _ => {}
        };

        if pos == impossible {
            panic!("Invalid sequence: {}", seq);
        }
    }
}

fn assert_directional_sequence(seq: &str) {
    let mut pos: Point = (0, 2).into();
    let impossible: Point = (0, 0).into();

    for c in seq.chars() {
        match c {
            '<' => {
                pos += Direction::Left.into();
            }
            '>' => {
                pos += Direction::Right.into();
            }
            '^' => {
                pos += Direction::Up.into();
            }
            'v' => {
                pos += Direction::Down.into();
            }
            _ => {}
        };

        if pos == impossible {
            panic!("Invalid sequence");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        parse_input(
            r#"
            029A
980A
179A
456A
379A
            "#
            .trim(),
        )
    }

    #[test]
    fn test_find_sequence() {
        assert_eq!(
            find_shortest_sequence("029A", 2).len(),
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
        );

        assert_eq!(
            find_shortest_sequence("980A", 2).len(),
            "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len()
        );
        assert_eq!(
            find_shortest_sequence("179A", 2).len(),
            "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
        );
        assert_eq!(
            find_shortest_sequence("456A", 2).len(),
            "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".len()
        );
        assert_eq!(
            find_shortest_sequence("379A", 2).len(),
            "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 126384);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 0);
    }
}
