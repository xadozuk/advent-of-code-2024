use std::{collections::HashMap, fmt::format};

use lib::{debugln, Point};

type Keypad = HashMap<char, Point>;

fn new_numeric_keypad() -> Keypad {
    [
        ['7', '8', '9'],
        ['4', '5', '6'],
        ['1', '2', '3'],
        [' ', '0', 'A'],
    ]
    .iter()
    .enumerate()
    .flat_map(|(i, row)| {
        row.iter()
            .enumerate()
            .map(move |(j, c)| (*c, (i as i64, j as i64).into()))
    })
    .collect()
}

fn new_directional_keypad() -> Keypad {
    [[' ', '^', 'A'], ['<', 'v', '>']]
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, c)| (*c, (i as i64, j as i64).into()))
        })
        .collect()
}

pub struct KeypadChain {
    numeric_keypad: Keypad,
    directional_keypad: Keypad,
    n_directional_keypads: usize,

    path_cost_cache: HashMap<(usize, char, char), usize>,
}

impl KeypadChain {
    pub fn new(n_directional_keypads: usize) -> Self {
        Self {
            numeric_keypad: new_numeric_keypad(),
            directional_keypad: new_directional_keypad(),
            n_directional_keypads,

            path_cost_cache: HashMap::new(),
        }
    }

    pub fn find_shortest_sequence_length(&mut self, code: &str) -> usize {
        self.keypresses_cost(code, self.n_directional_keypads + 1)
    }

    fn keypresses_cost(&mut self, code: &str, n_robots: usize) -> usize {
        // We start on A, we need to sum the cost of each code part A -> 1, 1 -> 2
        let code = "A".to_string() + code;

        let left = code.chars().take(code.len() - 1);
        let right = code.chars().skip(1);

        left.zip(right)
            .map(|(from, to)| self.path_cost(from, to, n_robots))
            .sum()
    }

    fn path_cost(&mut self, from: char, to: char, n_robots: usize) -> usize {
        // On the last keypad we can input directly
        if n_robots == 0 {
            return 1;
        }

        let indent_size = self.n_directional_keypads + 1 - n_robots;
        let indent = "\t".repeat(indent_size);

        debugln!(
            "{}({}) Computing path cost {} -> {}",
            indent,
            n_robots,
            from,
            to
        );

        if let Some(&result) = self.path_cost_cache.get(&(n_robots, from, to)) {
            debugln!(
                "\t{}[CACHE] Path cost hit ({:?}) = {}",
                indent,
                (n_robots, from, to),
                result
            );
            return result;
        }

        let keypad = if n_robots > self.n_directional_keypads {
            &self.numeric_keypad
        } else {
            &self.directional_keypad
        };

        let from_coord = keypad[&from];
        let to_coord = keypad[&to];
        let impossible_coord: Point = keypad[&' '];
        let vector = to_coord - from_coord;

        let vertical_steps =
            if vector.x > 0 { "v" } else { "^" }.repeat(vector.x.unsigned_abs() as usize);
        let horizontal_steps =
            if vector.y > 0 { ">" } else { "<" }.repeat(vector.y.unsigned_abs() as usize);

        // It is generally better to have continous sequence of move >>^^ instance of >^>^ to
        // reduce move count
        let vertical_first = format!("{vertical_steps}{horizontal_steps}A");
        let horizontal_first = format!("{horizontal_steps}{vertical_steps}A");

        debugln!(
            "{}\t{:?} -> {:?} = {:?}",
            indent,
            from_coord,
            to_coord,
            vector
        );
        debugln!("{}\tVF: {}", indent, vertical_first);
        debugln!("{}\tHF: {}", indent, horizontal_first);

        // We need to check that on the corner we don't go through the empty cell
        let vertical_first_cost = if impossible_coord == (to_coord.x, from_coord.y).into() {
            usize::MAX
        } else {
            self.keypresses_cost(&vertical_first, n_robots - 1)
        };

        let horizontal_first_cost = if vertical_first == horizontal_first
            || impossible_coord == (from_coord.x, to_coord.y).into()
        {
            usize::MAX
        } else {
            self.keypresses_cost(&horizontal_first, n_robots - 1)
        };

        debugln!(
            "{}\tVF = {}, HF = {}",
            indent,
            vertical_first_cost,
            horizontal_first_cost
        );

        let result = vertical_first_cost.min(horizontal_first_cost);
        self.path_cost_cache.insert((n_robots, from, to), result);

        debugln!("{}<==", indent);
        result
    }
}
