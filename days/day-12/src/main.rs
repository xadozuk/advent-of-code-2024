use std::{fs, time::Instant};

use farm::Farm;

type ParsedInput = Farm;
type Output = u64;

mod farm;

fn main() {
    let input = input();

    let start = Instant::now();
    println!("Result (part 1): {} [{:?}]", part1(&input), start.elapsed());

    let start = Instant::now();
    println!("Result (part 2): {} [{:?}]", part2(&input), start.elapsed());
}

fn input() -> ParsedInput {
    let input = fs::read_to_string("inputs/day-12.txt").unwrap();
    parse_input(&input)
}

fn parse_input(input: &str) -> ParsedInput {
    Farm::from(
        input
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>(),
    )
}

fn part1(input: &ParsedInput) -> Output {
    input.fence_total_price()
}

fn part2(input: &ParsedInput) -> Output {
    input.fence_discount_price()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        parse_input(
            r#"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
            "#
            .trim(),
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 1206);
    }

    #[test]
    fn test_custom_p2() {
        assert_eq!(
            part2(&parse_input(
                r#"
AAA
AAA
BBB
            "#
                .trim()
            )),
            6 * 4 + 3 * 4
        );

        assert_eq!(
            part2(&parse_input(
                r#"
AAA
ABA
AAA
            "#
                .trim()
            )),
            8 * 8 + 1 * 4
        );

        assert_eq!(
            part2(&parse_input(
                r#"
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
            "#
                .trim()
            )),
            2 * 4 * 4 + 28 * 12
        );

        assert_eq!(
            part2(&parse_input(
                r#"
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
            "#
                .trim()
            )),
            236
        );

        assert_eq!(
            part2(&parse_input(
                r#"
EEEEE
EXXXE
EXAXE
EXXXE
EEEEE
            "#
                .trim()
            )),
            16 * 8 + 8 * 8 + 1 * 4
        );

        assert_eq!(
            part2(&parse_input(
                r#"
EEEEE
EAEAE
EEEEE
EAEAE
EEEEE
            "#
                .trim()
            )),
            (4 * 1) * 4 + 21 * 20
        );

        assert_eq!(
            part2(&parse_input(
                r#"
AEEEA
EEEEE
EEEEE
EEEEE
AEEEA
            "#
                .trim()
            )),
            (4 * 1) * 4 + 21 * 12
        );
        assert_eq!(
            part2(&parse_input(
                r#"
AAAAAAAA
AACBBDDA
AACBBAAA
ABBAAAAA
ABBADDDA
AAAADADA
AAAAAAAA
            "#
                .trim()
            )),
            946
        );
    }
}
