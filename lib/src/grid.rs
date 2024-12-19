use crate::Point;

#[derive(Clone)]
pub struct Grid2d<T> {
    values: Vec<Vec<T>>,
}

pub struct Grid2dLine<'a, T> {
    line: &'a Vec<T>,
}

impl<T> Grid2d<T> {
    pub fn new(values: Vec<Vec<T>>) -> Self {
        Grid2d { values }
    }

    pub fn at(&self, pos: &Point) -> Option<&T> {
        if pos.x < 0
            || pos.y < 0
            || pos.x as usize >= self.values.len()
            || pos.y as usize >= self.values[0].len()
        {
            return None;
        }

        Some(&self.values[pos.x as usize][pos.y as usize])
    }

    pub fn at_mut(&mut self, pos: &Point) -> Option<&mut T> {
        if pos.x < 0
            || pos.y < 0
            || pos.x as usize >= self.values.len()
            || pos.y as usize >= self.values[0].len()
        {
            return None;
        }

        Some(&mut self.values[pos.x as usize][pos.y as usize])
    }

    pub fn iter(&self) -> iterator::Grid2dLines<T> {
        iterator::Grid2dLines::new(&self.values)
    }

    pub fn cardinal_neighboors(&self, pos: Point) -> iterator::CardinalNeighboors<T> {
        iterator::CardinalNeighboors::new(self, pos)
    }
}

impl<T> std::fmt::Debug for Grid2d<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (_, l) in self.iter() {
            for (_, c) in l.iter() {
                write!(f, "{:?}", c)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T> Grid2dLine<'_, T> {
    pub fn iter(&self) -> iterator::Grid2dCells<T> {
        iterator::Grid2dCells::new(self.line)
    }
}

mod iterator {
    use crate::{Direction, CARDINAL_DIRECTIONS};

    use super::{Grid2d, Grid2dLine, Point};

    pub struct Grid2dLines<'a, T> {
        lines: &'a Vec<Vec<T>>,
        index: usize,
    }

    pub struct Grid2dCells<'a, T> {
        cells: &'a Vec<T>,
        index: usize,
    }

    impl<'a, T> Grid2dLines<'a, T> {
        pub fn new(lines: &'a Vec<Vec<T>>) -> Self {
            Grid2dLines { lines, index: 0 }
        }
    }

    impl<'a, T> Grid2dCells<'a, T> {
        pub fn new(cells: &'a Vec<T>) -> Self {
            Grid2dCells { cells, index: 0 }
        }
    }
    impl<'a, T> Iterator for Grid2dLines<'a, T> {
        type Item = (usize, Grid2dLine<'a, T>);

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(line) = self.lines.get(self.index) {
                let r = (self.index, Grid2dLine { line });

                self.index += 1;

                return Some(r);
            }

            None
        }
    }

    impl<'a, T> Iterator for Grid2dCells<'a, T> {
        type Item = (usize, &'a T);

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(cell) = self.cells.get(self.index) {
                let r = (self.index, cell);

                self.index += 1;

                return Some(r);
            }

            None
        }
    }

    pub struct CardinalNeighboors<'a, T> {
        grid: &'a Grid2d<T>,
        pos: Point,
        direction_index: usize,
    }

    impl<'a, T> CardinalNeighboors<'a, T> {
        pub fn new(grid: &'a Grid2d<T>, pos: Point) -> Self {
            CardinalNeighboors {
                grid,
                pos,
                direction_index: 0,
            }
        }
    }

    impl<'a, T> Iterator for CardinalNeighboors<'a, T> {
        type Item = (&'a T, Point, Direction);

        fn next(&mut self) -> Option<Self::Item> {
            for (i, dir) in CARDINAL_DIRECTIONS
                .iter()
                .enumerate()
                .skip(self.direction_index)
            {
                let n_pos = self.pos + dir.into();

                if let Some(cell) = self.grid.at(&n_pos) {
                    self.direction_index = i + 1;

                    return Some((cell, n_pos, *dir));
                }
            }

            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Direction;

    use super::Grid2d;

    #[test]
    fn test_cardinal_neighboors() {
        let grid_one_cell = Grid2d::new(vec![vec![0; 1]; 1]);

        assert_eq!(
            None,
            grid_one_cell.cardinal_neighboors((0, 0).into()).next()
        );

        let two_by_two_grid = Grid2d::new(vec![vec![0; 2]; 2]);
        let mut it = two_by_two_grid.cardinal_neighboors((0, 0).into());

        assert_eq!(it.next(), Some((&0, (0, 1).into(), Direction::Right)));
        assert_eq!(it.next(), Some((&0, (1, 0).into(), Direction::Down)));
        assert_eq!(it.next(), None);

        let mut it = two_by_two_grid.cardinal_neighboors((1, 1).into());

        assert_eq!(it.next(), Some((&0, (0, 1).into(), Direction::Up)));
        assert_eq!(it.next(), Some((&0, (1, 0).into(), Direction::Left)));
        assert_eq!(it.next(), None);

        let three_by_three_grid = Grid2d::new(vec![vec![0; 3]; 3]);
        let mut it = three_by_three_grid.cardinal_neighboors((1, 1).into());

        assert_eq!(it.next(), Some((&0, (0, 1).into(), Direction::Up)));
        assert_eq!(it.next(), Some((&0, (1, 2).into(), Direction::Right)));
        assert_eq!(it.next(), Some((&0, (2, 1).into(), Direction::Down)));
        assert_eq!(it.next(), Some((&0, (1, 0).into(), Direction::Left)));
    }
}
