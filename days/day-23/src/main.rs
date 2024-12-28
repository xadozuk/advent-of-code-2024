use std::{collections::HashSet, fs, rc::Rc, time::Instant};

use graph::Graph;
use lib::debugln;

mod graph;

type ParsedInput = Graph;
type Output = usize;

fn main() {
    let input = input();

    let start = Instant::now();
    println!("Result (part 1): {} [{:?}]", part1(&input), start.elapsed());

    let start = Instant::now();
    println!("Result (part 2): {} [{:?}]", part2(&input), start.elapsed());
}

fn input() -> ParsedInput {
    let input = fs::read_to_string("inputs/day-23.txt").unwrap();
    parse_input(&input)
}

fn parse_input(input: &str) -> ParsedInput {
    Graph::from(input)
}

fn part1(input: &ParsedInput) -> Output {
    let all_t_nodes: HashSet<Rc<String>> = input
        .nodes
        .iter()
        .filter(|&n| n.starts_with("t"))
        .cloned()
        .collect();

    debugln!(
        "Found {} nodes with t in name: {:?}",
        all_t_nodes.len(),
        all_t_nodes
    );

    let three_tuples = find_all_three_tuples(&all_t_nodes, input);

    let mut unique_tuples = HashSet::new();

    for t in three_tuples {
        if !uniquely_contains(&t, &unique_tuples) {
            debugln!("{:?}", t);
            unique_tuples.insert(t);
        }
    }

    unique_tuples.len()
}

fn part2(input: &ParsedInput) -> String {
    let set = input.find_largest_connected_set();

    let mut connected_nodes = set
        .1
        .into_iter()
        .map(|v| (*v).clone())
        .collect::<Vec<String>>();

    connected_nodes.sort();

    connected_nodes.join(",")
}

type ThreeTuple = (Rc<String>, Rc<String>, Rc<String>);

fn find_all_three_tuples(expected_nodes: &HashSet<Rc<String>>, graph: &Graph) -> Vec<ThreeTuple> {
    let mut result = vec![];

    for node in expected_nodes {
        debugln!("Searchin 3-tuple for {}", node);
        result.append(&mut find_three_tuples(node, graph));
    }

    result
}

fn find_three_tuples(node: &Rc<String>, graph: &Graph) -> Vec<ThreeTuple> {
    let mut result = vec![];
    let links = graph.links.get(node);

    if links.is_none() {
        return vec![];
    }

    let links = links.unwrap();

    for link in links.iter() {
        for second_link in links.iter().filter(|&n| n != link) {
            if is_three_connected(graph, node, link, second_link) {
                let tuple = (node.clone(), link.clone(), second_link.clone());

                result.push((node.clone(), link.clone(), second_link.clone()));
            }
        }
    }

    result.into_iter().collect()
}

fn is_three_connected(graph: &Graph, a: &Rc<String>, b: &Rc<String>, c: &Rc<String>) -> bool {
    if let (Some(a_links), Some(b_links)) = (graph.links.get(a), graph.links.get(b)) {
        return a_links.contains(b) && a_links.contains(c) && b_links.contains(c);
    }

    false
}

fn uniquely_contains(tuple: &ThreeTuple, set: &HashSet<ThreeTuple>) -> bool {
    set.contains(tuple)
        || set.contains(&(tuple.0.clone(), tuple.2.clone(), tuple.1.clone()))
        || set.contains(&(tuple.1.clone(), tuple.0.clone(), tuple.2.clone()))
        || set.contains(&(tuple.1.clone(), tuple.2.clone(), tuple.0.clone()))
        || set.contains(&(tuple.2.clone(), tuple.0.clone(), tuple.1.clone()))
        || set.contains(&(tuple.2.clone(), tuple.1.clone(), tuple.0.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        parse_input(
            r#"
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
            "#
            .trim(),
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), "co,de,ka,ta");
    }
}
