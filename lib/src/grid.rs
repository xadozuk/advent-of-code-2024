use crate::Point;

#[derive(Clone)]
pub struct Grid2d<T> {
    values: Vec<Vec<T>>,
}

pub struct Grid2dLine<'a, T> {
    line: &'a Vec<T>,
}

pub struct Grid2dLineIterator<'a, T> {
    lines: &'a Vec<Vec<T>>,
    index: usize,
}

pub struct Grid2dCellIterator<'a, T> {
    cells: &'a Vec<T>,
    index: usize,
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

    pub fn iter(&self) -> Grid2dLineIterator<T> {
        Grid2dLineIterator {
            index: 0,
            lines: &self.values,
        }
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
    pub fn iter(&self) -> Grid2dCellIterator<T> {
        Grid2dCellIterator {
            cells: self.line,
            index: 0,
        }
    }
}

impl<'a, T> Iterator for Grid2dLineIterator<'a, T> {
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

impl<'a, T> Iterator for Grid2dCellIterator<'a, T> {
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
