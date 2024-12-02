use std::{collections::HashMap, fs};

fn main() {
    let input = input();

    println!("Result (part 1): {}", part1(&input));
    println!("Result (part 2): {}", part2(&input));
}

fn input() -> Vec<(i32, i32)> {
    let input = fs::read_to_string("inputs/day-01.txt").unwrap();

    input
        .lines()
        .map(|line| {
            line.split_once("   ")
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap()
        })
        .collect()
}

fn part1(input: &Vec<(i32, i32)>) -> i32 {
    let mut left_list: Vec<i32> = input.iter().map(|(l, _)| *l).collect();
    let mut right_list: Vec<i32> = input.iter().map(|(_, r)| *r).collect();

    left_list.sort();
    right_list.sort();

    left_list
        .iter()
        .zip(right_list)
        .fold(0, |acc, (l, r)| acc + (l - r).abs())
}

fn part2(input: &Vec<(i32, i32)>) -> i32 {
    let right_list_counts = input.iter().fold(HashMap::new(), |mut acc, (_, r)| {
        acc.entry(*r).and_modify(|e| *e += 1).or_insert(1);
        acc
    });

    input.iter().fold(0, |acc, (l, _)| {
        acc + l * right_list_counts.get(&l).unwrap_or(&0)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &[(i32, i32)] = &[(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)];

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT.into()), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT.into()), 31);
    }
}
