use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    ops::Add,
};

#[derive(Hash, Eq, PartialEq, Clone)]
struct Coord(i32, i32);

struct Region(char, HashMap<Coord, usize>);

struct BoundingBox {
    top: i32,
    bottom: i32,
    left: i32,
    right: i32,
}

impl Region {
    fn contains(&self, other: &Region) -> bool {
        let my_bb = self.bounding_box();
        let other_bb = other.bounding_box();

        my_bb.top < other_bb.top
            && my_bb.bottom > other_bb.bottom
            && my_bb.left < other_bb.left
            && my_bb.right > other_bb.right
    }

    fn is_touching(&self, other: &Region) -> bool {
        for coord_a in self.1.keys() {
            for coord_b in other.1.keys() {
                if coord_a + Direction::Up.into() == *coord_b
                    || coord_a + Direction::Down.into() == *coord_b
                    || coord_a + Direction::Left.into() == *coord_b
                    || coord_a + Direction::Right.into() == *coord_b
                {
                    return true;
                }
            }
        }

        false
    }

    fn bounding_box(&self) -> BoundingBox {
        let top = self.1.keys().map(|c| c.0).min().unwrap();
        let bottom = self.1.keys().map(|c| c.0).max().unwrap();
        let left = self.1.keys().map(|c| c.1).min().unwrap();
        let right = self.1.keys().map(|c| c.1).max().unwrap();

        BoundingBox {
            top,
            bottom,
            left,
            right,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const ALL: [Self; 4] = [Self::Down, Self::Left, Self::Up, Self::Right];

    pub fn turn_right(&self) -> Self {
        match self {
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
            Self::Right => Self::Down,
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Self::Down => Self::Right,
            Self::Right => Self::Up,
            Self::Up => Self::Left,
            Self::Left => Self::Down,
        }
    }

    pub fn turn_around(&self) -> Self {
        match self {
            Self::Down => Self::Up,
            Self::Up => Self::Down,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub fn n_turns_from(&self, initial: &Direction) -> usize {
        if *self == initial.turn_left() || *self == initial.turn_right() {
            1
        } else if *self == initial.turn_around() {
            2
        } else {
            0
        }
    }
}

impl From<Direction> for Coord {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Coord(-1, 0),
            Direction::Down => Coord(1, 0),
            Direction::Left => Coord(0, -1),
            Direction::Right => Coord(0, 1),
        }
    }
}

pub struct Farm {
    land: Vec<Vec<char>>,
    regions: Vec<Region>,
}

impl Farm {
    pub fn fence_total_price(&self) -> u64 {
        self.regions
            .iter()
            .map(|r| self.region_fence_price(r))
            .sum()
    }

    pub fn fence_discount_price(&self) -> u64 {
        let mut acc = 0;

        println!("===");
        for r in self.regions.iter() {
            let outer_corners = self.count_region_outer_corners(r);
            let inner_corners = self.count_region_inner_corners(r);

            println!(
                "({}) [{}] {{{}, {}}}",
                r.0,
                r.1.len(),
                outer_corners,
                inner_corners
            );

            acc += r.1.len() as u64 * (outer_corners + inner_corners);
        }

        acc
    }

    fn region_fence_price(&self, Region(_, plots): &Region) -> u64 {
        let area = plots.len();
        let perimeter = plots.values().sum::<usize>();

        (area * perimeter) as u64
    }

    fn count_region_inner_corners(&self, region: &Region) -> u64 {
        // Find inner region
        let inner_regions: Vec<&Region> = self
            .regions
            .iter()
            .filter(|r| region.contains(r))
            .filter(|r| region.is_touching(r))
            .collect();

        inner_regions
            .iter()
            .map(|r| self.count_region_outer_corners(r))
            .sum()
    }

    fn count_region_outer_corners(&self, Region(_rop, plots): &Region) -> u64 {
        if plots.len() == 1 {
            return 4;
        }

        let mut sorted_plots: BTreeSet<&Coord> = plots.keys().collect();
        //find topest exterior point of the shape
        let starting_pos = sorted_plots.pop_first().unwrap();

        // Do a first step to be in the "correct" direction
        let mut cur_vector = self
            .walk_perimeter(&(starting_pos.clone(), Direction::Right), plots)
            .unwrap();

        let mut visited: HashSet<(Coord, Direction)> = HashSet::new();
        let mut turns = 0;

        visited.insert(cur_vector.clone());

        while let Some(next_vector) = self.walk_perimeter(&cur_vector, plots) {
            if next_vector.1 != cur_vector.1 {
                turns += next_vector.1.n_turns_from(&cur_vector.1);
            }

            if visited.contains(&next_vector) {
                break;
            }

            visited.insert(next_vector.clone());
            cur_vector = next_vector;
        }

        turns as u64
    }

    fn walk_perimeter(
        &self,
        (coord, dir): &(Coord, Direction),
        plots: &HashMap<Coord, usize>,
    ) -> Option<(Coord, Direction)> {
        let next_steps = [dir.turn_left(), *dir, dir.turn_right(), dir.turn_around()];

        for next_dir in next_steps {
            let next_pos = coord + next_dir.into();

            if self.plot_at(&next_pos).is_some() && plots.contains_key(&next_pos) {
                return Some((next_pos, next_dir));
            }
        }

        None
    }

    fn compute_regions(&mut self) {
        self.regions.clear();

        let mut visited: HashSet<Coord> = HashSet::new();

        for (i, line) in self.land.iter().enumerate() {
            for (j, &plot) in line.iter().enumerate() {
                let pos = (i, j).into();

                if visited.contains(&pos) {
                    continue;
                }

                let region = self.find_region(plot, pos);

                region.iter().for_each(|(c, _)| {
                    visited.insert(c.clone());
                });

                self.regions.push(Region(plot, region));
            }
        }
    }

    fn find_region(&self, plot: char, starting_pos: Coord) -> HashMap<Coord, usize> {
        let mut continuous_plot_with_different_neighboors = HashMap::new();
        let mut todos = VecDeque::new();

        todos.push_back(starting_pos.clone());
        continuous_plot_with_different_neighboors.insert(starting_pos, 0);

        while let Some(pos) = todos.pop_front() {
            for d in Direction::ALL {
                let new_pos = pos.clone() + d.into();

                if continuous_plot_with_different_neighboors.contains_key(&new_pos) {
                    continue;
                }

                if Some(plot) == self.plot_at(&new_pos) {
                    todos.push_back(new_pos.clone());
                    continuous_plot_with_different_neighboors.insert(new_pos, 0);
                } else {
                    *continuous_plot_with_different_neighboors
                        .entry(pos.clone())
                        .or_insert(0) += 1
                }
            }
        }

        continuous_plot_with_different_neighboors
    }

    fn plot_at(&self, pos: &Coord) -> Option<char> {
        if pos.0 < 0
            || pos.1 < 0
            || pos.0 as usize >= self.land.len()
            || pos.1 as usize >= self.land[0].len()
        {
            None
        } else {
            Some(self.land[pos.0 as usize][pos.1 as usize])
        }
    }
}

impl From<Vec<Vec<char>>> for Farm {
    fn from(value: Vec<Vec<char>>) -> Self {
        let mut farm = Farm {
            land: value,
            regions: Vec::new(),
        };

        farm.compute_regions();

        farm
    }
}

impl From<(usize, usize)> for Coord {
    fn from((x, y): (usize, usize)) -> Self {
        Coord(x as i32, y as i32)
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<Coord> for &Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.0.cmp(&other.0) {
            Ordering::Equal => self.1.cmp(&other.1),
            o => o,
        }
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
