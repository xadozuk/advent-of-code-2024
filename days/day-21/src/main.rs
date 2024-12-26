use std::{fs, time::Instant};

use keypads::KeypadChain;
use lib::{Direction, Point};

mod keypads;

type ParsedInput = Vec<(String, u64)>;
type Output = u64;

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
            let numeric_part: u64 = code
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
    let mut keypad_chain = KeypadChain::new(2);

    input
        .iter()
        .map(|(code, n)| keypad_chain.find_shortest_sequence_length(code) as u64 * n)
        .sum()
}

fn part2(input: &ParsedInput) -> Output {
    let mut keypad_chain = KeypadChain::new(25);

    input
        .iter()
        .map(|(code, n)| keypad_chain.find_shortest_sequence_length(code) as u64 * n)
        .sum()
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
            KeypadChain::new(2).find_shortest_sequence_length("029A"),
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
        );

        assert_eq!(
            KeypadChain::new(2).find_shortest_sequence_length("980A"),
            "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len()
        );
        assert_eq!(
            KeypadChain::new(2).find_shortest_sequence_length("179A"),
            "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
        );
        assert_eq!(
            KeypadChain::new(2).find_shortest_sequence_length("456A"),
            "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".len()
        );
        assert_eq!(
            KeypadChain::new(2).find_shortest_sequence_length("379A"),
            "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 126384);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            KeypadChain::new(25).find_shortest_sequence_length("029A"),
            82050061710
        );

        assert_eq!(
            KeypadChain::new(25).find_shortest_sequence_length("980A"),
            72242026390
        );
        assert_eq!(
            KeypadChain::new(25).find_shortest_sequence_length("179A"),
            81251039228
        );
        assert_eq!(
            KeypadChain::new(25).find_shortest_sequence_length("456A"),
            80786362258
        );
        assert_eq!(
            KeypadChain::new(25).find_shortest_sequence_length("379A"),
            77985628636
        );

        assert_eq!(part2(&input()), 154115708116294);
    }

    #[test]
    fn test_4_p2() {
        let expected = &[
            12, 26,
            64,
            // 162,
            // 394,
            // 988,
            // 2434,
            // 6082,
            // 15090,
            // 37576,
            // 93444,
            // 232450,
            // 578314,
            // 1438450,
            // 3578646,
            // 8901822,
            // 22145084,
            // 55087898,
            // 137038728,
            // 340900864,
            // 848032810,
            // 2109590876,
            // 5247866716,
            // 13054736520,
            // 32475283854,
            // 80786362258,
        ];

        let code = "456A";

        for (i, exp) in expected.iter().enumerate() {
            println!("Code: {}, n robots: {}", code, i);
            assert_eq!(
                KeypadChain::new(i).find_shortest_sequence_length(code),
                *exp
            );
        }
    }
}
