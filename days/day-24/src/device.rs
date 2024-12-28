use std::{
    collections::{BTreeSet, HashMap, HashSet},
    rc::Rc,
};

use itertools::Itertools;
use lib::debugln;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum GateType {
    And,
    Or,
    Xor,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Gate {
    input: (Rc<String>, Rc<String>),
    gate_type: GateType,
    output: Rc<String>,
}

#[derive(Clone)]
pub struct Device {
    wires: BTreeSet<Rc<String>>,
    gates: HashSet<Rc<Gate>>,
    state: HashMap<Rc<String>, Option<bool>>,

    input_index: HashMap<Rc<String>, Vec<Rc<Gate>>>,
    output_index: HashMap<Rc<String>, Rc<Gate>>,
}

impl Device {
    pub fn parse(input: &str) -> Self {
        let (initial_state, gates) = input.split_once("\n\n").unwrap();

        let mut all_wires = BTreeSet::new();

        let state = initial_state
            .lines()
            .map(|line| {
                let (gate, value_str) = line.split_once(": ").unwrap();

                let gate = Rc::new(gate.to_string());

                all_wires.insert(gate.clone());

                (gate.clone(), value_str.parse::<u32>().map(|v| v == 1).ok())
            })
            .collect();

        let parsed_gates = gates
            .lines()
            .map(|line| {
                let parts: Vec<&str> = line.split(' ').collect();

                debug_assert!(parts.len() == 5);

                let a = Rc::new(parts[0].to_string());
                let b = Rc::new(parts[2].to_string());
                let output = Rc::new(parts[4].to_string());
                let gate_type = parts[1];

                if !all_wires.contains(&a) {
                    all_wires.insert(a.clone());
                }

                if !all_wires.contains(&b) {
                    all_wires.insert(b.clone());
                }

                if !all_wires.contains(&output) {
                    all_wires.insert(output.clone());
                }

                let parsed_gate_type = match gate_type {
                    "AND" => GateType::And,
                    "OR" => GateType::Or,
                    "XOR" => GateType::Xor,
                    _ => panic!("Unsupported gate type"),
                };

                Rc::new(Gate {
                    gate_type: parsed_gate_type,
                    input: (a.clone(), b.clone()),
                    output: output.clone(),
                })
            })
            .collect();

        let mut device = Device {
            wires: all_wires,
            gates: parsed_gates,
            state,
            input_index: HashMap::new(),
            output_index: HashMap::new(),
        };

        device.build_indexes();

        device
    }

    fn build_indexes(&mut self) {
        self.input_index.clear();
        self.output_index.clear();

        for gate in self.gates.iter().collect::<Vec<&Rc<Gate>>>() {
            self.input_index
                .entry(gate.input.0.clone())
                .or_default()
                .push(gate.clone());

            self.input_index
                .entry(gate.input.1.clone())
                .or_default()
                .push(gate.clone());

            self.output_index.insert(gate.output.clone(), gate.clone());
        }
    }

    pub fn converge(&mut self) -> Result<(), ()> {
        let z_wires: Vec<Rc<String>> = self
            .wires
            .iter()
            .filter(|w| w.starts_with("z"))
            .cloned()
            .collect();

        let mut visited = HashSet::new();
        for wire in z_wires {
            if self.resolve(wire, &mut visited).is_none() {
                return Err(());
            }
        }

        Ok(())
    }

    fn resolve(&mut self, wire: Rc<String>, visited: &mut HashSet<Rc<String>>) -> Option<bool> {
        // debugln!("Resolving {}", wire);

        if let Some(value) = self.state.get(&wire) {
            // debugln!("\t[CACHE] {} = {:?}", wire, value);
            return *value;
        }

        // Loop detection
        if visited.contains(&wire) {
            // debugln!("Loop detected on {}", wire);
            // self.debug_dependency_graph(&wire);

            return None;
        }

        visited.insert(wire.clone());

        if let Some(gate) = self.output_index.get(&wire) {
            let gate = gate.clone();

            // debugln!("\t* {} {:?} {}", gate.input.0, gate.gate_type, gate.input.1);

            if let (Some(a), Some(b)) = (
                self.resolve(gate.input.0.clone(), visited),
                self.resolve(gate.input.1.clone(), visited),
            ) {
                let output_value = gate.process(a, b);

                // debugln!("\t -> {}", output_value);

                self.state.insert(gate.output.clone(), Some(output_value));

                Some(output_value)
            } else {
                // This happen in case of loop, so we need to exit fast
                self.state.insert(gate.output.clone(), None);
                None
            }
        } else {
            panic!("Impossible case !");
        }
    }

    pub fn solve_switch(&self, n_pairs: usize, expected_value: u64) -> Vec<Rc<String>> {
        debugln!("Starting solve...");

        let wrong_gates = self.find_wrong_gates();

        debugln!(
            "Found {} gates breaking ripple carry adder rule:",
            wrong_gates.len()
        );

        wrong_gates.iter().for_each(|g| debugln!("{:?}", g));

        if wrong_gates.len() < n_pairs * 2 {
            debugln!("Not enough wrong gate to try permutations...");
            return vec![];
        }

        debugln!("Trying permutation...");
        for permutation in wrong_gates.iter().permutations(n_pairs * 2) {
            let outputs = permutation.iter().map(|g| g.output.clone()).collect();

            let mut device = self.clone();

            for chunk in permutation.chunks_exact(2) {
                device.swap_gates_output(chunk[0], chunk[1]);
            }

            device.build_indexes();

            if device.converge().is_err() {
                // debugln!("Invalid (loop)");
                continue;
            }

            debugln!(
                "{} == {} ({:?})",
                expected_value,
                device.wires_number('z'),
                outputs
            );

            if expected_value == device.wires_number('z') {
                debugln!("Correct permutation !");
                return outputs;
            }
        }

        vec![]
    }

    fn find_wrong_gates(&self) -> HashSet<Rc<Gate>> {
        let mut wrong_gates = HashSet::new();

        for gate in self.gates.iter() {
            let inputs_on_x_y =
                gate.input.0.starts_with(['x', 'y']) && gate.input.1.starts_with(['x', 'y']);

            let output_on_z = gate.output.starts_with('z');

            // First Z gate (half adder)
            if *gate.output == "z00" {
                if gate.gate_type != GateType::Xor
                    || !["x00", "y00"].contains(&gate.input.0.as_str())
                    || !["x00", "y00"].contains(&gate.input.1.as_str())
                {
                    wrong_gates.insert(gate.clone());
                }
                continue;
            }

            // Last Z bit is carry detection
            if *gate.output == "z45" {
                if gate.gate_type != GateType::Or || inputs_on_x_y {
                    wrong_gates.insert(gate.clone());
                }

                continue;
            }

            if inputs_on_x_y
                && gate.input.0.ends_with("00")
                && gate.input.1.ends_with("00")
                && gate.gate_type == GateType::And
            {
                if gate.output.starts_with('z') {
                    wrong_gates.insert(gate.clone());
                }

                continue;
            }

            // Short-circuit
            // Xor gate not connected to X/Y must be connected to Z
            if gate.gate_type == GateType::Xor && !inputs_on_x_y {
                if !gate.output.starts_with('z') {
                    wrong_gates.insert(gate.clone());
                }

                continue;
            }

            // If output is on Z, Gate must be XOR and must not have input on X/Y
            if output_on_z && (gate.gate_type != GateType::Xor || inputs_on_x_y) {
                wrong_gates.insert(gate.clone());
            }

            // If Input is on X/Y with Xor gate, output must not be on Z
            // If input is on X/Y with And gate (Carry), output must not be on Z
            if inputs_on_x_y
                && (gate.gate_type == GateType::Xor || gate.gate_type == GateType::And)
                && output_on_z
            {
                wrong_gates.insert(gate.clone());
            }

            // And carry gate (not connected to X/Y) must not be on Z
            // Or carry gate must not be on Z
            if !inputs_on_x_y
                && (gate.gate_type == GateType::And || gate.gate_type == GateType::Or)
                && output_on_z
            {
                wrong_gates.insert(gate.clone());
            }

            if inputs_on_x_y && gate.gate_type == GateType::Xor {
                // We check that we connect to Xor & And gates
                if let Some(following_gates) = self.input_index.get(&gate.output) {
                    // If we don't have 2 Gates (Xor & And)
                    if following_gates.len() != 2
                        || following_gates
                            .iter()
                            .filter(|g| g.gate_type == GateType::Xor)
                            .count()
                            != 1
                        || following_gates
                            .iter()
                            .filter(|g| g.gate_type == GateType::And)
                            .count()
                            != 1
                    {
                        debugln!("({}) XOR gate not connected to Xor/And gate", gate.output);
                        wrong_gates.insert(gate.clone());
                    }
                } else {
                    wrong_gates.insert(gate.clone());
                }
            }

            if gate.gate_type == GateType::And {
                // We check that we connect to Xor & And gates
                if let Some(following_gates) = self.input_index.get(&gate.output) {
                    // If we don't have 2 Gates (Xor & And)
                    if following_gates.len() != 1
                        || following_gates
                            .iter()
                            .filter(|g| g.gate_type == GateType::Or)
                            .count()
                            != 1
                    {
                        debugln!("({}) AND gate not connected to Or gate", gate.output);
                        wrong_gates.insert(gate.clone());
                    }
                } else {
                    wrong_gates.insert(gate.clone());
                }
            }

            // Only match carry gate because we continue on z45
            if gate.gate_type == GateType::Or {
                // Check that ouput is connected on a Xor Gate with Z output or And gate (on non Z
                // output)
                if let Some(following_gates) = self.input_index.get(&gate.output) {
                    // If we don't have 2 Gates (Xor & And)
                    if following_gates.len() != 2
                        || following_gates
                            .iter()
                            .filter(|g| g.gate_type == GateType::Xor)
                            .count()
                            != 1
                        || following_gates
                            .iter()
                            .filter(|g| g.gate_type == GateType::And)
                            .count()
                            != 1
                    {
                        debugln!("({}) OR gate not connected to Xor/And gate", gate.output);
                        wrong_gates.insert(gate.clone());
                    }
                } else {
                    wrong_gates.insert(gate.clone());
                }
            }
        }

        wrong_gates
    }

    fn swap_gates_output(&mut self, a: &Rc<Gate>, b: &Rc<Gate>) {
        let new_a = Gate {
            input: a.input.clone(),
            gate_type: a.gate_type,
            output: b.output.clone(),
        };

        let new_b = Gate {
            input: b.input.clone(),
            gate_type: b.gate_type,
            output: a.output.clone(),
        };

        // debugln!("Switching {} <-> {}", a.output, b.output);

        self.gates.remove(a);
        self.gates.remove(b);

        self.gates.insert(Rc::new(new_a));
        self.gates.insert(Rc::new(new_b));
    }

    pub fn wires_values(&self, start: char) -> Vec<u8> {
        self.wires
            .iter()
            .filter(|w| w.starts_with(start))
            .rev()
            .map(|w| if self.state[w].unwrap() { 1 } else { 0 })
            .collect()
    }

    pub fn convert_values_to_number(values: &[u8]) -> u64 {
        values.iter().fold(0, |acc, n| (acc << 1) + *n as u64)
    }

    pub fn wires_number(&self, start: char) -> u64 {
        Self::convert_values_to_number(&self.wires_values(start))
    }

    pub fn debug(&self) {
        debugln!("Gates: {}", self.gates.len());

        debugln!("=== STATE ===");

        let mut keys: Vec<Rc<String>> = self.state.keys().cloned().collect();
        keys.sort();

        for key in keys {
            debugln!("{} = {:?}", key, self.state[&key]);
        }
    }
}

impl Gate {
    pub fn process(&self, a: bool, b: bool) -> bool {
        match self.gate_type {
            GateType::And => a & b,
            GateType::Or => a | b,
            GateType::Xor => a ^ b,
        }
    }
}
