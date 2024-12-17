use crate::debug::debugln;

#[derive(Clone)]
pub struct Cpu {
    program: Vec<u8>,

    pub reg_a: u64,
    reg_b: u64,
    reg_c: u64,

    pc: usize,
}

impl Cpu {
    pub fn from(input: &str) -> Self {
        let (registers, program) = input.split_once("\n\n").unwrap();

        let registers: Vec<u64> = registers
            .lines()
            .map(|l| l.trim().split_once(":").unwrap().1.trim())
            .flat_map(|n| n.parse())
            .collect();

        let program = program
            .trim()
            .replace("Program: ", "")
            .split(",")
            .flat_map(|op| op.parse())
            .collect();

        Cpu {
            program,
            pc: 0,
            reg_a: registers[0],
            reg_b: registers[1],
            reg_c: registers[2],
        }
    }

    pub fn run(&mut self) -> Vec<u8> {
        let mut stdout = Vec::new();
        self.pc = 0;

        while let Some(opcode) = self.program.get(self.pc) {
            let operand = self.program.get(self.pc + 1).unwrap();

            match opcode {
                // ADV
                0 => self.reg_a /= 2u64.pow(self.combo(*operand) as u32),
                // BXL
                1 => self.reg_b ^= *operand as u64,
                // BST
                2 => self.reg_b = self.combo(*operand) & 0b111,
                // JNZ
                3 => {
                    if self.reg_a != 0 {
                        self.pc = *operand as usize;
                        continue;
                    }
                }
                // BXC
                4 => self.reg_b ^= self.reg_c,
                // OUT
                5 => stdout.push((self.combo(*operand) & 0b111) as u8),
                // BDV
                6 => self.reg_b = self.reg_a / 2u64.pow(self.combo(*operand) as u32),
                // CDV
                7 => self.reg_c = self.reg_a / 2u64.pow(self.combo(*operand) as u32),
                _ => {
                    panic!("Unsupported opcode")
                }
            }

            self.pc += 2;
        }

        stdout
    }

    pub fn reset(&mut self) {
        self.reg_a = 0;
        self.reg_b = 0;
        self.reg_b = 0;
        self.pc = 0;
    }

    pub fn find_program_as_stdout(&mut self) -> u64 {
        let program: Vec<u8> = self.program.iter().rev().copied().collect();

        debugln!("Value to find: {:?}", program);

        let r = self.find_factors(&program, &[]).unwrap();

        debugln!("{:?}", r);

        self.a_from_factors(&r)
    }

    fn find_factors(&mut self, values: &[u8], previous: &[u32]) -> Option<Vec<u32>> {
        if values.is_empty() {
            return Some(previous.to_vec());
        }

        let value = values[0];
        let remaining_values = &values[1..values.len()];

        debugln!("{}Previous: {:?}", "\t".repeat(previous.len()), previous);

        debugln!(
            "{}Searching value: {} (remaining: {:?})",
            "\t".repeat(previous.len()),
            value,
            remaining_values
        );
        let range = self.range_for_factors(previous);

        debugln!("{}For range: {:?}", "\t".repeat(previous.len()), range);

        for a in range {
            if value == self.run_for_a(a as u64)[0] {
                let mut factor = (a % 8) as u32;

                // We need to discard the first pass [0] (as it doesn't translate in other ranges)
                if previous.is_empty() {
                    factor -= 1
                }

                debugln!("Found {} for A = {} (factor: {})", value, a, factor);

                let mut previous = previous.to_vec();
                previous.insert(0, factor);

                if let Some(factors) = self.find_factors(remaining_values, &previous) {
                    return Some(factors);
                }
            }
        }

        debugln!("{}<- Nope", "\t".repeat(previous.len() + 1));
        None
    }

    fn range_for_factors(&self, factors: &[u32]) -> std::ops::Range<usize> {
        if factors.is_empty() {
            return 1..8;
        }

        let start = 8usize.pow(factors.len() as u32);

        let delta: usize = factors
            .iter()
            .enumerate()
            .map(|(i, f)| (*f as usize) * 8usize.pow(1 + i as u32))
            .sum();

        debugln!("[range] ({}, {})", start, delta);

        (start + delta)..(start + delta + 8)
    }

    fn run_for_a(&mut self, a: u64) -> Vec<u8> {
        self.reset();
        self.reg_a = a;
        self.run()
    }

    fn a_from_factors(&self, factors: &[u32]) -> u64 {
        let delta: u64 = factors
            .iter()
            .enumerate()
            .map(|(i, n)| *n as u64 * 8u64.pow(i as u32))
            .sum();

        8u64.pow(factors.len() as u32 - 1) + delta
    }

    fn combo(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => {
                panic!("Unsupported combo operand");
            }
        }
    }
}
