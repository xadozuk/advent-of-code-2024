use std::collections::{HashMap, HashSet, VecDeque};

use lib::{debugln, Grid2d, Point, CARDINAL_DIRECTIONS};

struct Region(char, HashMap<Point, usize>);

pub struct Farm {
    land: Grid2d<char>,
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

        debugln!("===");
        for r in self.regions.iter() {
            let corners = self.count_region_corners(r);

            debugln!("({}) [{}] {}", r.0, r.1.len(), corners);

            acc += r.1.len() as u64 * corners;
        }
        debugln!("===");

        acc
    }

    fn region_fence_price(&self, Region(_, plots): &Region) -> u64 {
        let area = plots.len();
        let perimeter = plots.values().sum::<usize>();

        (area * perimeter) as u64
    }

    fn count_region_corners(&self, region: &Region) -> u64 {
        let mut corners = 0;

        for pos in region.1.keys() {
            let (nw, n, ne, w, e, sw, s, se) = self.land.all_neighboors_tuple(*pos);
            let current = self.land.at(pos).unwrap();

            if nw != Some(current) && n == Some(current) && w == Some(current)
                || nw != Some(current) && n != Some(current) && w != Some(current)
                || nw == Some(current) && n != Some(current) && w != Some(current)
            {
                corners += 1;
            }

            if ne != Some(current) && n == Some(current) && e == Some(current)
                || ne != Some(current) && n != Some(current) && e != Some(current)
                || ne == Some(current) && n != Some(current) && e != Some(current)
            {
                corners += 1;
            }

            if sw != Some(current) && s == Some(current) && w == Some(current)
                || sw != Some(current) && s != Some(current) && w != Some(current)
                || sw == Some(current) && s != Some(current) && w != Some(current)
            {
                corners += 1;
            }

            if se != Some(current) && s == Some(current) && e == Some(current)
                || se != Some(current) && s != Some(current) && e != Some(current)
                || se == Some(current) && s != Some(current) && e != Some(current)
            {
                corners += 1;
            }
        }

        corners
    }

    fn compute_regions(&mut self) {
        self.regions.clear();

        let mut visited: HashSet<Point> = HashSet::new();

        for (i, line) in self.land.iter() {
            for (j, &plot) in line.iter() {
                let pos = (i as i64, j as i64).into();

                if visited.contains(&pos) {
                    continue;
                }

                let region = self.find_region(plot, pos);

                region.iter().for_each(|(c, _)| {
                    visited.insert(*c);
                });

                self.regions.push(Region(plot, region));
            }
        }
    }

    fn find_region(&self, plot: char, starting_pos: Point) -> HashMap<Point, usize> {
        let mut continuous_plot_with_different_neighboors = HashMap::new();
        let mut todos = VecDeque::new();

        todos.push_back(starting_pos);
        continuous_plot_with_different_neighboors.insert(starting_pos, 0);

        while let Some(pos) = todos.pop_front() {
            for d in CARDINAL_DIRECTIONS {
                let new_pos = pos + d.into();

                if continuous_plot_with_different_neighboors.contains_key(&new_pos) {
                    continue;
                }

                if Some(&plot) == self.land.at(&new_pos) {
                    todos.push_back(new_pos);
                    continuous_plot_with_different_neighboors.insert(new_pos, 0);
                } else {
                    *continuous_plot_with_different_neighboors
                        .entry(pos)
                        .or_insert(0) += 1
                }
            }
        }

        continuous_plot_with_different_neighboors
    }
}

impl From<Vec<Vec<char>>> for Farm {
    fn from(value: Vec<Vec<char>>) -> Self {
        let mut farm = Farm {
            land: Grid2d::new(value),
            regions: Vec::new(),
        };

        farm.compute_regions();

        farm
    }
}
