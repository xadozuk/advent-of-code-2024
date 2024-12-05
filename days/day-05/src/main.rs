use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    time::Instant,
};

type ParsedInput = (Vec<(u32, u32)>, Vec<Vec<u32>>);
type Output = u32;

fn main() {
    let input = input();

    let start = Instant::now();
    println!("Result (part 1): {} [{:?}]", part1(&input), start.elapsed());

    let start = Instant::now();
    println!("Result (part 2): {} [{:?}]", part2(&input), start.elapsed());
}

fn input() -> ParsedInput {
    let input = fs::read_to_string("inputs/day-05.txt").unwrap();

    parse_input(&input)
}

fn parse_input(input: &str) -> ParsedInput {
    let (rules_input, updates_input) = input.split_once("\n\n").unwrap();

    let rules = rules_input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("|").unwrap();

            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect();

    let updates = updates_input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|page| page.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    (rules, updates)
}

fn part1(input: &ParsedInput) -> Output {
    let rule_index = build_rule_index(&input.0);

    input
        .1
        .iter()
        .filter(|update| is_in_correct_order(update, &rule_index))
        .map(|update| {
            let middle = update.len() / 2;
            update[middle]
        })
        .sum()
}

fn part2(input: &ParsedInput) -> Output {
    let rule_index = build_rule_index(&input.0);

    let incorrect_updates = input
        .1
        .iter()
        .filter(|update| !is_in_correct_order(update, &rule_index));

    incorrect_updates
        .map(|update| fix_update_order(update, &rule_index))
        .map(|update| {
            let middle = update.len() / 2;
            update[middle]
        })
        .sum()
}

fn fix_update_order(update: &[u32], rule_index: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let all_pages = update.iter().copied().collect::<HashSet<u32>>();
    let mut processed_pages = HashSet::<u32>::with_capacity(update.len());
    let mut fixed_update = Vec::<u32>::with_capacity(update.len());

    let mut wrong_pages = VecDeque::<u32>::with_capacity(update.len());
    let mut wrong_pages_swap = Vec::<u32>::with_capacity(update.len());

    for page in update {
        if !is_page_correct(page, &all_pages, &processed_pages, rule_index) {
            wrong_pages.push_back(*page);
        } else {
            processed_pages.insert(*page);
            fixed_update.push(*page);
        }

        wrong_pages_swap.clear();

        while let Some(wrong_page) = wrong_pages.pop_front() {
            if is_page_correct(&wrong_page, &all_pages, &processed_pages, rule_index) {
                processed_pages.insert(wrong_page);
                fixed_update.push(wrong_page);
            } else {
                wrong_pages_swap.push(wrong_page);
            }
        }

        wrong_pages_swap
            .iter()
            .for_each(|p| wrong_pages.push_back(*p));
    }

    while !wrong_pages.is_empty() {
        wrong_pages_swap.clear();

        while let Some(wrong_page) = wrong_pages.pop_front() {
            if is_page_correct(&wrong_page, &all_pages, &processed_pages, rule_index) {
                processed_pages.insert(wrong_page);
                fixed_update.push(wrong_page);
            } else {
                wrong_pages_swap.push(wrong_page);
            }
        }

        wrong_pages_swap
            .iter()
            .for_each(|p| wrong_pages.push_back(*p));
    }

    fixed_update
}

fn is_page_correct(
    page: &u32,
    all_pages: &HashSet<u32>,
    previous_pages: &HashSet<u32>,
    rule_index: &HashMap<u32, Vec<u32>>,
) -> bool {
    let expected_previous_page = rule_index.get(page).and_then(|previous_pages| {
        let existing_previous_pages: Vec<u32> = previous_pages
            .iter()
            .filter(|p| all_pages.contains(p))
            .copied()
            .collect();

        if existing_previous_pages.is_empty() {
            None
        } else {
            Some(existing_previous_pages)
        }
    });

    if let Some(expected_previous_page) = expected_previous_page {
        if expected_previous_page
            .iter()
            .any(|p| !previous_pages.contains(p))
        {
            return false;
        }
    }

    true
}

fn build_rule_index(rules: &Vec<(u32, u32)>) -> HashMap<u32, Vec<u32>> {
    let mut index = HashMap::<u32, Vec<u32>>::new();

    for &(left, right) in rules {
        index
            .entry(right)
            .and_modify(|e| e.push(left))
            .or_insert(vec![left]);
    }

    index
}

fn is_in_correct_order(update: &Vec<u32>, rule_index: &HashMap<u32, Vec<u32>>) -> bool {
    let all_pages = update.iter().copied().collect::<HashSet<u32>>();
    let mut processed_pages = HashSet::<u32>::with_capacity(update.len());

    for page in update {
        if !is_page_correct(page, &all_pages, &processed_pages, rule_index) {
            return false;
        }

        processed_pages.insert(*page);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

        parse_input(input)
    }

    #[test]
    fn test_part1() {
        println!("{:?}", input());

        assert_eq!(part1(&input()), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 123);
    }
}
