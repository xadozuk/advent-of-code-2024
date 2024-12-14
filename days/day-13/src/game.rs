pub struct Point {
    pub x: i64,
    pub y: i64,
}

pub struct ClawMachine {
    prize: Point,
    a: Point,
    b: Point,
}

impl From<(i64, i64)> for Point {
    fn from((x, y): (i64, i64)) -> Self {
        Point { x, y }
    }
}

impl ClawMachine {
    pub const A_TOKEN: i64 = 3;
    pub const B_TOKEN: i64 = 1;
    const CONVERSION: i64 = 10_000_000_000_000;

    pub fn new(prize: Point, a: Point, b: Point) -> Self {
        ClawMachine { prize, a, b }
    }

    pub fn solve(&self) -> Option<Point> {
        self.solve_for(&self.prize).filter(|solution| {
            if solution.x > 100 || solution.y > 100 {
                println!("Out-of-range solution");
                return false;
            }

            true
        })
    }

    pub fn solve_with_converstion(&self) -> Option<Point> {
        self.solve_for(
            &(
                self.prize.x + Self::CONVERSION,
                self.prize.y + Self::CONVERSION,
            )
                .into(),
        )
        .filter(|solution| {
            if solution.x < 100 || solution.y < 100 {
                println!("Out-of-range solution");
                return false;
            }

            true
        })
    }

    fn solve_for(&self, target: &Point) -> Option<Point> {
        let b_prime = self.a.x * target.y - self.a.y * target.x;
        let factor = self.a.x * self.b.y - self.a.y * self.b.x;

        if b_prime % factor != 0 {
            println!("Non-integer solution");
            return None;
        }

        let b = b_prime / factor;
        let a_prime = target.x - self.b.x * b;

        if a_prime % self.a.x != 0 {
            println!("Non-interger solution");
            return None;
        }

        let a = a_prime / self.a.x;

        if a < 0 || b < 0 {
            println!("Out-of-range solution");
            return None;
        }

        println!(
            "Solution found for (X={}, Y={}): ({}, {})",
            target.x, target.y, a, b
        );

        Some((a, b).into())
    }
}
