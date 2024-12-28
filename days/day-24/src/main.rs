use std::{fs, time::Instant};

use device::Device;

type ParsedInput = Device;
type Output = u64;

mod device;

fn main() {
    let input = input();

    let start = Instant::now();
    println!("Result (part 1): {} [{:?}]", part1(&input), start.elapsed());

    let start = Instant::now();
    println!("Result (part 2): {} [{:?}]", part2(&input), start.elapsed());
}

fn input() -> ParsedInput {
    let input = fs::read_to_string("inputs/day-24.txt").unwrap();
    parse_input(&input)
}

fn parse_input(input: &str) -> ParsedInput {
    Device::parse(input)
}

fn part1(input: &ParsedInput) -> Output {
    let mut input = input.clone();
    let _ = input.converge();

    input.debug();

    input.wires_number('z')
}

fn part2(input: &ParsedInput) -> String {
    let input = input.clone();

    let expected_value: u64 = input.wires_number('x') + input.wires_number('y');

    let result = input.solve_switch(4, expected_value);

    let mut result: Vec<String> = result.into_iter().map(|v| v.to_string()).collect();
    result.sort();

    result.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ParsedInput {
        parse_input(
            r#"
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
        "#
            .trim(),
        )
    }

    #[test]
    fn test_example() {
        assert_eq!(
            part1(&parse_input(
                r#"
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
"#
                .trim()
            )),
            4
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 2024);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(
            r#"
x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00
        "#
            .trim(),
        );

        let mut result: Vec<String> = input
            .solve_switch(2, input.wires_number('x') & input.wires_number('y'))
            .into_iter()
            .map(|v| v.to_string())
            .collect();
        result.sort();

        assert_eq!(result.join(","), "z00,z01,z02,z05");
    }
}
