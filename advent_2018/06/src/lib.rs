#![feature(external_doc)]
#![doc(include = "../Question.md")]

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use failure::{bail, Error};

const INPUT: &str = include_str!("../input");

pub fn answer() -> (String, String) {
    let grid = Grid::from(INPUT).unwrap();
    (
        grid.most_remote().unwrap().to_string(),
        grid.busiest_region(10000).unwrap().to_string(),
    )
}

#[derive(Debug)]
struct Grid {
    coords: Vec<Point>,
    max: Point,
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    /// Finds the Manhattan distance between two points.
    fn distance(self, p: Self) -> u32 {
        max(self.x, p.x) - min(self.x, p.x) + max(self.y, p.y) - min(self.y, p.y)
    }

    /// Returns the four adjacent points to the current point (above, below, to the left,
    /// and to the right).
    fn adjacent(self) -> [Point; 4] {
        [
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }
}

impl Grid {
    fn from(s: &str) -> Result<Self, Error> {
        let (coords, errors): (Vec<_>, Vec<_>) = s
            .lines()
            .map(|l| l.parse::<Point>())
            .partition(Result::is_ok);

        let coords: Vec<_> = coords.into_iter().map(Result::unwrap).collect();
        let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();

        if !errors.is_empty() {
            bail!("Parsing errors: {:?}", errors);
        }

        let max = Point {
            x: coords.iter().max_by_key(|p| p.x).unwrap().x,
            y: coords.iter().max_by_key(|p| p.y).unwrap().y,
        };

        Ok(Grid { coords, max })
    }

    /// Given a point `point`, finds the coordinate in `self.coords` with the minimum Manhattan
    /// distance to the point.
    fn closest(&self, point: Point) -> Point {
        *self
            .coords
            .iter()
            .min_by_key(|p| p.distance(point))
            .unwrap()
    }

    /// Finds the points adjacent to `p` that are on the grid.
    fn adjacent(&self, p: Point) -> Vec<Point> {
        p.adjacent()
            .iter()
            .filter(|p| p.x > 0 && p.x <= self.max.x && p.y > 0 && p.y <= self.max.y)
            .cloned()
            .collect()
    }

    /// Finds the total Manhattan distance between all coords in the grid and `point`.
    fn total_distance(&self, point: Point) -> u32 {
        self.coords.iter().fold(0, |acc, p| acc + p.distance(point))
    }

    fn most_remote(&self) -> Option<u32> {
        let coord_set: HashSet<Point> = self.coords.iter().cloned().collect();
        let mut infinite: HashSet<Point> = HashSet::new();
        let mut sizes: HashMap<Point, u32> = HashMap::new();
        let mut checked: HashSet<Point> = coord_set.clone();

        // Loop top and bottom edges.
        for &y in &[1_u32, self.max.y] {
            for x in 1..=self.max.x {
                infinite.insert(self.closest(Point { x, y }));
                checked.insert(Point { x, y });
            }
        }

        // Loop left and right edges.
        for &x in &[1_u32, self.max.x] {
            for y in 2..self.max.y {
                infinite.insert(self.closest(Point { x, y }));
                checked.insert(Point { x, y });
            }
        }

        let mut to_check: Vec<(Point, Point)> = self
            .coords
            .iter()
            .filter(|p| !infinite.contains(p))
            .map(|p| (*p, *p))
            .collect();

        while let Some((p, p_closest)) = to_check.pop() {
            for adj_p in self.adjacent(p) {
                if !checked.contains(&adj_p) {
                    let closest = self.closest(adj_p);
                    (*sizes.entry(closest).or_insert(1)) += 1;
                    checked.insert(adj_p);
                    if p_closest == closest {
                        to_check.push((adj_p, closest));
                    }
                }
            }
        }

        sizes.values().max().cloned()
    }

    fn busiest_region(&self, total_distance: u32) -> Option<u32> {
        let mut to_check: Vec<Point> = Vec::new();
        let mut checked: HashSet<Point> = HashSet::new();
        let mut region_size = 0;

        let coords_len = self.coords.len();
        let totals = self
            .coords
            .iter()
            .fold((0, 0), |acc, p| (acc.0 + p.x, acc.1 + p.y));
        let starting_point = (
            f64::from(totals.0) / coords_len as f64,
            f64::from(totals.1) / coords_len as f64,
        );

        // only one of these could be within the max_distance.
        for &x in &[
            starting_point.0.floor() as u32,
            starting_point.0.ceil() as u32,
        ] {
            for &y in &[
                starting_point.1.floor() as u32,
                starting_point.1.ceil() as u32,
            ] {
                if !to_check.contains(&Point { x, y }) {
                    to_check.push(Point { x, y });
                }
            }
        }

        while let Some(p) = to_check.pop() {
            if self.total_distance(p) <= total_distance && !checked.contains(&p) {
                checked.insert(p);
                region_size += 1;
                for adj_p in self.adjacent(p) {
                    if !checked.contains(&adj_p) {
                        to_check.push(adj_p);
                    }
                }
            }
        }

        Some(region_size)
    }
}

impl FromStr for Point {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .split_terminator(", ")
            .map(|w| w.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()?;
        if numbers.len() != 2 {
            bail!(
                "Wrong number of numbers found, should be two, found: {:?}",
                numbers
            );
        }
        Ok(Point {
            x: numbers[0],
            y: numbers[1],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let input = "1, 1\n\
                     1, 6\n\
                     8, 3\n\
                     3, 4\n\
                     5, 5\n\
                     8, 9\n";

        let grid = Grid::from(input).unwrap();
        assert_eq!(grid.most_remote().unwrap(), 17);
        assert_eq!(grid.busiest_region(30).unwrap(), 16);
    }

    #[test]
    fn test_answer() {
        assert_eq!(answer(), (String::from("4011"), String::from("46054")));
    }
}
