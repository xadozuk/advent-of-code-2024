use std::{fs, time::Instant};

type ParsedInput = Vec<Vec<char>>;
type Output = u32;

fn main() {
    let input = input();

    let start = Instant::now();
    println!("Result (part 1): {} [{:?}]", part1(&input), start.elapsed());

    let start = Instant::now();
    println!("Result (part 2): {} [{:?}]", part2(&input), start.elapsed());
}

fn input() -> ParsedInput {
    let input = fs::read_to_string("inputs/day-04.txt").unwrap();
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &ParsedInput) -> Output {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter().enumerate().map(move |(j, char)| {
                if *char == 'X' {
                    count_xmas_words(input, i, j)
                } else {
                    0
                }
            })
        })
        .sum()
}

fn part2(input: &ParsedInput) -> Output {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter().enumerate().map(move |(j, char)| {
                if *char == 'A' && has_cross_mas_words(input, i, j) {
                    1
                } else {
                    0
                }
            })
        })
        .sum()
}

fn get_letter_around(
    input: &[Vec<char>],
    pos: (usize, usize),
    offset: (isize, isize),
) -> Option<char> {
    let (i, j) = pos;
    let (l, c) = offset;

    let letter_i = i.checked_add_signed(l);
    let letter_j = j.checked_add_signed(c);

    letter_i.and_then(|i| letter_j.and_then(|j| input.get(i).and_then(|line| line.get(j)).copied()))
}

fn count_xmas_words(input: &[Vec<char>], i: usize, j: usize) -> u32 {
    let letters = ['X', 'M', 'A', 'S'];
    let mut word_matches: [u32; 9] = [1; 9];

    for n in 1..=3 {
        let expected_letter = letters[n as usize];

        for l in -1..=1 {
            for c in -1..=1 {
                let match_i = (c + 1) + 3 * (l + 1);
                let actual_letter = get_letter_around(input, (i, j), (l * n, c * n));

                if let Some(actual_letter) = actual_letter {
                    if actual_letter != expected_letter {
                        word_matches[match_i as usize] = 0;
                    }
                } else {
                    word_matches[match_i as usize] = 0;
                }
            }
        }

        if word_matches.iter().all(|&v| v == 0) {
            break;
        }
    }

    word_matches.iter().sum::<u32>()
}

fn has_cross_mas_words(input: &[Vec<char>], i: usize, j: usize) -> bool {
    let pos = (i, j);
    let top_left = get_letter_around(input, pos, (-1, -1));
    let bottom_right = get_letter_around(input, pos, (1, 1));
    let top_right = get_letter_around(input, pos, (-1, 1));
    let bottom_left = get_letter_around(input, pos, (1, -1));

    let expected = [(Some('M'), Some('S')), (Some('S'), Some('M'))];

    let n_pair: u32 = [(top_left, bottom_right), (top_right, bottom_left)]
        .iter()
        .flat_map(|&pair| {
            expected
                .iter()
                .map(move |&expected| if pair == expected { 1 } else { 0 })
        })
        .sum();

    n_pair == 2
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#
            .lines()
            .map(|line| line.chars().collect())
            .collect()
    }

    #[test]
    fn test_part1() {
        println!("{:?}", input());

        assert_eq!(part1(&input()), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 9);
    }
}
