use std::{
    collections::{HashMap, HashSet},
    fs,
    time::Instant,
};

use lib::debugln;

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
    let input = fs::read_to_string("inputs/day-22.txt").unwrap();
    parse_input(&input)
}

fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect()
}

fn part1(input: &ParsedInput) -> Output {
    input
        .iter()
        .map(|initial| secret_number(*initial, 2000))
        .sum()
}

type Seq = Vec<i64>;
fn part2(input: &ParsedInput) -> Output {
    let start = Instant::now();
    println!("Building data structures...");
    let vendor_prices: Vec<Vec<(u64, i64)>> = price_and_delta_per_vendors(input);

    let vendor_sequences_prices = build_vendor_sequences(&vendor_prices);
    let all_sequences: HashSet<Seq> = vendor_sequences_prices
        .iter()
        .flat_map(|map| map.keys().cloned())
        .collect();

    println!("Data structure built in {:?}", start.elapsed());

    debugln!("Scanning sequences...");

    let mut max: (u64, Seq) = (0, vec![]);

    for (i, seq) in all_sequences.iter().enumerate() {
        debugln!(
            "[{}/{}] Checking sequence: {:?}",
            i,
            all_sequences.len(),
            seq
        );

        let mut price = 0;

        for sequences in vendor_sequences_prices.iter() {
            if let Some(p) = sequences.get(seq) {
                price += p;
            }
        }

        if price > max.0 {
            debugln!("Found new max price for {:?} = {}", seq, price);
            max = (price, seq.clone());
        }
    }

    max.0
}

fn price_and_delta_per_vendors(input: &[u64]) -> Vec<Vec<(u64, i64)>> {
    input
        .iter()
        .map(|initial| {
            let mut secret = *initial;
            let mut previous_price = 0;

            (0..2000)
                .map(|_| {
                    let price = secret % 10;
                    let delta = price as i64 - previous_price;

                    secret = next_secret_number(secret);
                    previous_price = price as i64;

                    (price, delta)
                })
                .collect()
        })
        .collect()
}

fn build_vendor_sequences(vendor_prices: &Vec<Vec<(u64, i64)>>) -> Vec<HashMap<Seq, u64>> {
    let mut result = Vec::with_capacity(vendor_prices.len());

    for prices in vendor_prices {
        let mut map = HashMap::new();
        // Skip the first one as it has no valid sequence
        for seq in prices.windows(4).skip(1) {
            let delta_set: Vec<i64> = seq.iter().map(|v| v.1).collect();

            map.entry(delta_set)
                .or_insert_with(|| seq.last().unwrap().0);
        }

        result.push(map);
    }

    result
}

fn secret_number(initial: u64, iteration: u64) -> u64 {
    (0..iteration).fold(initial, |secret, _| next_secret_number(secret))
}

fn next_secret_number(secret: u64) -> u64 {
    let a = ((secret * 64) ^ secret) % 16777216;
    let b = ((a / 32) ^ a) % 16777216;
    ((b * 2048) ^ b) % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        parse_input(
            r#"
1
10
100
2024
"#
            .trim(),
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 37327623);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse_input(
                r#"
1
2
3
2024
            "#
                .trim()
            )),
            23
        );
    }
}
