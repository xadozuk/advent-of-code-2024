use std::{
    collections::{BTreeSet, HashMap},
    fs,
    time::Instant,
};

use lib::debugln;

#[derive(Eq, PartialEq, Clone)]
struct Towel {
    stripes: String,
}

impl Ord for Towel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.stripes.len().cmp(&self.stripes.len()) {
            std::cmp::Ordering::Equal => self.stripes.cmp(&other.stripes),
            ordering => ordering,
        }
    }
}

impl PartialOrd for Towel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Debug for Towel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.stripes)
    }
}

struct Onsen {
    towels: BTreeSet<Towel>,
    patterns: Vec<String>,
}

type ParsedInput = Onsen;
type Output = u64;

fn main() {
    let input = input();

    let start = Instant::now();
    println!("Result (part 1): {} [{:?}]", part1(&input), start.elapsed());

    let start = Instant::now();
    println!("Result (part 2): {} [{:?}]", part2(&input), start.elapsed());
}

fn input() -> ParsedInput {
    let input = fs::read_to_string("inputs/day-19.txt").unwrap();
    parse_input(&input)
}

fn parse_input(input: &str) -> ParsedInput {
    let (towels, pattern) = input.split_once("\n\n").unwrap();

    let parsed_towels = towels
        .split(", ")
        .map(|str| Towel {
            stripes: str.to_string(),
        })
        .collect();
    let parsed_patterns = pattern.lines().map(&str::to_string).collect();

    Onsen {
        towels: parsed_towels,
        patterns: parsed_patterns,
    }
}

fn part1(input: &ParsedInput) -> Output {
    let mut r = 0;
    let mut cache: HashMap<String, u64> = HashMap::new();
    for (i, pattern) in input.patterns.iter().enumerate() {
        debugln!("[{}/{}] Pattern: {}", i + 1, input.patterns.len(), pattern);
        if find_pattern(pattern, &input.towels, &mut cache) != 0 {
            debugln!("\tFound solution",);
            r += 1;
        }
    }

    r
}

fn part2(input: &ParsedInput) -> Output {
    let mut r = 0;
    let mut cache: HashMap<String, u64> = HashMap::new();
    for (i, pattern) in input.patterns.iter().enumerate() {
        debugln!("[{}/{}] Pattern: {}", i + 1, input.patterns.len(), pattern);
        r += find_pattern(pattern, &input.towels, &mut cache);
    }

    r
}

fn find_pattern(pattern: &str, towels: &BTreeSet<Towel>, cache: &mut HashMap<String, u64>) -> u64 {
    if pattern.is_empty() {
        debugln!("<-");
        return 1;
    }

    if cache.contains_key(pattern) {
        // debugln!("Cache hit");
        debugln!(
            "{: >20} Cache match: {} ({:?})",
            "",
            pattern,
            cache[pattern]
        );
        return cache[pattern];
    }

    debugln!("{: >20}", pattern);

    let mut result = 0;

    for towel in towels.iter() {
        // debugln!("[{}] Checking towel {}", pattern, towel.stripes);
        // debugln!("{: >20} Checking: {}", "", towel.stripes);
        if pattern.starts_with(&towel.stripes) {
            // debugln!("\tMatch {} with {}", pattern, towel.stripes);
            debugln!("{: >20} Match: {}", "", towel.stripes);
            let remaining = &pattern[towel.stripes.len()..];

            let r = find_pattern(remaining, towels, cache);
            cache.insert(remaining.to_string(), r);

            result += r;
        }
    }

    // debugln!("{: >20} Nope", "");
    cache.insert(pattern.to_string(), result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        parse_input(
            r#"
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
        "#
            .trim(),
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 16);
    }
}
