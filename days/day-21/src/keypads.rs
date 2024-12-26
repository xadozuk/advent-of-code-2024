use std::{collections::HashMap, usize, vec};

use lib::{debugln, Grid2d, Point};

#[derive(PartialEq, Eq)]
enum DirectionalButton {
    Up,
    Down,
    Left,
    Right,
    A,
    Empty,
}

#[derive(PartialEq, Eq)]
enum NumericButton {
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B9,
    B0,
    A,
    Empty,
}

pub type GlobalSequenceCache = HashMap<String, String>;
pub type SequenceCache = HashMap<(Point, Point), String>;
pub trait Keypad {
    fn sequence_for(
        &mut self,
        code: &str,
        cache: &mut SequenceCache,
        global_cache: &mut GlobalSequenceCache,
    ) -> String;
}

pub struct DirectionalKeyPad {
    cursor: Point,
    grid: Grid2d<DirectionalButton>,
}

impl DirectionalKeyPad {
    pub fn new() -> Self {
        use DirectionalButton::*;

        Self {
            grid: Grid2d::new(vec![vec![Empty, Up, A], vec![Left, Down, Right]]),
            cursor: (0, 2).into(),
        }
    }

    fn button_for(&self, c: char) -> DirectionalButton {
        use DirectionalButton::*;

        match c {
            '^' => Up,
            '<' => Left,
            '>' => Right,
            'v' => Down,
            'A' => A,
            _ => panic!("Unsupported button: {}", c),
        }
    }

    fn move_to(&mut self, button: DirectionalButton, cache: &mut SequenceCache) -> String {
        // First down, then right to avoid empty cell
        let target_position = self.find_button(button).unwrap();

        if let Some(seq) = cache.get(&(self.cursor, target_position)) {
            self.cursor = target_position;

            return seq.clone();
        }

        let vector = target_position - self.cursor;
        let mut seq = String::with_capacity(5);

        if vector.x > 0 {
            seq.push_str(&"v".repeat(vector.x.unsigned_abs() as usize));
        }
        if vector.y > 0 {
            seq.push_str(&">".repeat(vector.y.unsigned_abs() as usize));
        }
        if vector.x < 0 {
            seq.push_str(&"^".repeat(vector.x.unsigned_abs() as usize));
        }
        if vector.y < 0 {
            seq.push_str(&"<".repeat(vector.y.unsigned_abs() as usize));
        }

        seq.push('A');
        cache.insert((self.cursor, target_position), seq.clone());

        self.cursor = target_position;

        seq
    }

    fn find_button(&self, button: DirectionalButton) -> Option<Point> {
        for (i, l) in self.grid.iter() {
            for (j, t) in l.iter() {
                if *t == button {
                    return Some((i as i64, j as i64).into());
                }
            }
        }

        None
    }
}

pub struct NumericKeypad {
    cursor: Point,
    grid: Grid2d<NumericButton>,
}

impl NumericKeypad {
    pub fn new() -> Self {
        use NumericButton::*;

        Self {
            grid: Grid2d::new(vec![
                vec![B7, B8, B9],
                vec![B4, B5, B6],
                vec![B1, B2, B3],
                vec![Empty, B0, A],
            ]),
            cursor: (3, 2).into(),
        }
    }

    fn button_for(&self, c: char) -> NumericButton {
        use NumericButton::*;

        match c {
            '0' => B0,
            '1' => B1,
            '2' => B2,
            '3' => B3,
            '4' => B4,
            '5' => B5,
            '6' => B6,
            '7' => B7,
            '8' => B8,
            '9' => B9,
            'A' => A,
            _ => panic!("Unsupported button: {}", c),
        }
    }

    fn move_to(&mut self, button: NumericButton, cache: &mut SequenceCache) -> String {
        let target_position = self.find_button(button).unwrap();

        if let Some(seq) = cache.get(&(self.cursor, target_position)) {
            self.cursor = target_position;
            return seq.clone();
        }

        let vector = target_position - self.cursor;
        let mut seq = String::with_capacity(10);

        debugln!("Cursor: {:?}", self.cursor);
        debugln!("Target: {:?}", target_position);
        debugln!("Vector: {:?}", vector);

        // First right to optimize sequence for other Keypads
        // But we neeed to handle the case around the empty space
        if self.cursor.x == 3 && target_position.y == 0 {
            seq.push_str(&"^".repeat(vector.x.unsigned_abs() as usize));
            seq.push_str(&"<".repeat(vector.y.unsigned_abs() as usize));
        } else if self.cursor.y == 0 && target_position.x == 3 {
            seq.push_str(&">".repeat(vector.y.unsigned_abs() as usize));
            seq.push_str(&"v".repeat(vector.x.unsigned_abs() as usize));
        } else {
            if vector.y < 0 {
                seq.push_str(&"<".repeat(vector.y.unsigned_abs() as usize));
            }
            if vector.x > 0 {
                seq.push_str(&"v".repeat(vector.x.unsigned_abs() as usize));
            }
            if vector.y > 0 {
                seq.push_str(&">".repeat(vector.y.unsigned_abs() as usize));
            }
            if vector.x < 0 {
                seq.push_str(&"^".repeat(vector.x.unsigned_abs() as usize));
            }
        }

        seq.push('A');

        cache.insert((self.cursor, target_position), seq.clone());

        self.cursor = target_position;

        seq
    }

    fn find_button(&self, button: NumericButton) -> Option<Point> {
        for (i, l) in self.grid.iter() {
            for (j, t) in l.iter() {
                if *t == button {
                    return Some((i as i64, j as i64).into());
                }
            }
        }

        None
    }
}

impl Keypad for NumericKeypad {
    fn sequence_for(
        &mut self,
        code: &str,
        cache: &mut SequenceCache,
        _: &mut GlobalSequenceCache,
    ) -> String {
        let mut sequence: Vec<String> = Vec::with_capacity(code.len());

        for c in code.chars() {
            let button = self.button_for(c);
            let seq = self.move_to(button, cache);

            sequence.push(seq);
        }

        sequence.join("")
    }
}

impl Keypad for DirectionalKeyPad {
    fn sequence_for(
        &mut self,
        code: &str,
        cache: &mut SequenceCache,
        global_cache: &mut GlobalSequenceCache,
    ) -> String {
        if let Some(seq) = global_cache.get(code) {
            return seq.clone();
        }

        let mut sequence = Vec::with_capacity(code.len());

        for c in code.chars() {
            let button = self.button_for(c);
            let seq = self.move_to(button, cache);

            sequence.push(seq);
        }

        let result = sequence.join("");

        global_cache.insert(code.to_string(), result.clone());

        result
    }
}
