use std::{collections::HashMap, fs, ops::Div, time::Instant};

type ParsedInput = Vec<u64>;
type Output = u64;

fn main() {
    let input = input();

    let start = Instant::now();
    println!("Result (part 1): {} [{:?}]", part1(&input), start.elapsed());

    let start = Instant::now();
    println!("Result (part 2): {} [{:?}]", part2(&input), start.elapsed());
}

fn input() -> ParsedInput {
    let input = fs::read_to_string("inputs/day-11.txt").unwrap();
    parse_input(&input)
}

fn parse_input(input: &str) -> ParsedInput {
    input
        .trim()
        .split(' ')
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

fn part1(input: &ParsedInput) -> Output {
    blink_stones(input, 25)
}

fn part2(input: &ParsedInput) -> Output {
    blink_stones(input, 75)
}

fn split_int(n: u64) -> (u64, u64) {
    let n_half_size = (n.ilog10() + 1).div(2);
    let left_part = n.div(10u64.pow(n_half_size));
    let right_part = n - left_part * 10u64.pow(n_half_size);

    (left_part, right_part)
}

fn n_digit(n: u64) -> u32 {
    n.ilog10() + 1
}

fn blink_stone(n: u64) -> (u64, Option<u64>) {
    if n == 0 {
        (1, None)
    } else if n_digit(n) % 2 == 0 {
        let (a, b) = split_int(n);

        (a, Some(b))
    } else {
        (n * 2024, None)
    }
}

fn blink_stones(stones: &[u64], count: u32) -> Output {
    let mut stones: HashMap<u64, usize> = stones.iter().map(|n| (*n, 1)).collect();

    for _ in 0..count {
        let mut new_stones = HashMap::new();

        for (stone, count) in stones {
            let (a, b) = blink_stone(stone);

            new_stones
                .entry(a)
                .and_modify(|c| *c += count)
                .or_insert(count);

            if let Some(b) = b {
                new_stones
                    .entry(b)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            }
        }

        stones = new_stones
    }

    stones.values().sum::<usize>() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        parse_input("125 17")
    }

    #[test]
    fn test_split_int() {
        assert_eq!(split_int(123456), (123, 456));
        assert_eq!(split_int(123001), (123, 1));
    }

    #[test]
    fn test_part1() {
        // Wrong example in AoC
        assert_eq!(part1(&input()), 55312);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 65601038650482);
    }
}
