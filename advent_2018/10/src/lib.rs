#![feature(external_doc)]
#![doc(include = "../Question.md")]
#![feature(test)]

extern crate test;

use failure::{bail, format_err, Error};
use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashSet;
use std::fmt;
use std::i32;
use std::str::FromStr;

const INPUT: &str = include_str!("../input");

pub fn answer() -> (String, String) {
    let (coords, time) = Sky::from(INPUT).unwrap().message().unwrap();
    (coords.to_string(), time.to_string())
}

#[derive(Debug, Clone, Copy)]
struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}

#[derive(Debug, Clone)]
struct Sky {
    points: Vec<Point>,
}

struct Coords {
    list: Vec<(i32, i32)>,
    map: HashSet<(i32, i32)>,
}

impl Point {
    /// Work out where a Point will have moved to at time t.
    fn position_at(&self, t: i32) -> (i32, i32) {
        (
            self.position.0 + self.velocity.0 * t,
            self.position.1 + self.velocity.1 * t,
        )
    }
}

impl FromStr for Point {
    type Err = Error;
    /// e.g. position=< 9,  1> velocity=< 0,  2>
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref re: Regex =
                Regex::new(r"^\s*position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>$")
                    .unwrap();
        }

        let matches = re
            .captures(s)
            .ok_or_else(|| format_err!("Failed to parse into an Event: {:?}", s))?;

        Ok(Point {
            position: (matches[1].parse::<i32>()?, matches[2].parse::<i32>()?),
            velocity: (matches[3].parse::<i32>()?, matches[4].parse::<i32>()?),
        })
    }
}

impl Sky {
    fn from(s: &str) -> Result<Self, Error> {
        let (results, errors): (Vec<_>, Vec<_>) = s
            .lines()
            .map(|l| l.parse::<Point>())
            .partition(Result::is_ok);

        if !errors.is_empty() {
            bail!(
                "{:#?}",
                errors
                    .into_iter()
                    .map(Result::unwrap_err)
                    .collect::<Vec<_>>()
            );
        }
        Ok(Sky {
            points: results
                .into_iter()
                .map(Result::unwrap)
                .collect::<Vec<Point>>(),
        })
    }

    fn coords_at(&self, t: i32) -> Coords {
        Coords::from(self.points.iter().map(|p| p.position_at(t)).collect())
    }

    /// Return the message in the sky (as a coordinate list, convert to string to show the
    /// message) and the time at which it was generated.
    fn message(&self) -> Result<(Coords, i32), Error> {
        for t in 0..i32::MAX {
            let coords = self.coords_at(t);
            if coords.all_touching() {
                return Ok((coords, t));
            }
        }
        Err(format_err!("Didn't find a point where they all touch."))
    }
}

impl Coords {
    fn from(v: Vec<(i32, i32)>) -> Self {
        let mut coord_map = HashSet::new();
        for &coord in &v {
            coord_map.insert(coord);
        }
        Coords {
            list: v,
            map: coord_map,
        }
    }

    fn all_touching(&self) -> bool {
        self.list.iter().all(|&(x, y)| {
            self.map.contains(&(x, y + 1))
                || self.map.contains(&(x, y - 1))
                || self.map.contains(&(x + 1, y))
                || self.map.contains(&(x - 1, y))
                || self.map.contains(&(x - 1, y - 1))
                || self.map.contains(&(x - 1, y + 1))
                || self.map.contains(&(x + 1, y - 1))
                || self.map.contains(&(x + 1, y + 1))
        })
    }
}

impl fmt::Display for Coords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut max = (i32::MIN, i32::MIN);
        let mut min = (i32::MAX, i32::MAX);

        for &(x, y) in &self.list {
            if y > max.1 {
                max.1 = y;
            }
            if y < min.1 {
                min.1 = y;
            }
            if x > max.0 {
                max.0 = x;
            }
            if x < min.0 {
                min.0 = x;
            }
        }

        for y in min.1..=max.1 {
            for x in min.0..=max.0 {
                if self.map.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const EXAMPLE_INPUT: &str = "position=< 9,  1> velocity=< 0,  2>\n\
                                 position=< 7,  0> velocity=<-1,  0>\n\
                                 position=< 3, -2> velocity=<-1,  1>\n\
                                 position=< 6, 10> velocity=<-2, -1>\n\
                                 position=< 2, -4> velocity=< 2,  2>\n\
                                 position=<-6, 10> velocity=< 2, -2>\n\
                                 position=< 1,  8> velocity=< 1, -1>\n\
                                 position=< 1,  7> velocity=< 1,  0>\n\
                                 position=<-3, 11> velocity=< 1, -2>\n\
                                 position=< 7,  6> velocity=<-1, -1>\n\
                                 position=<-2,  3> velocity=< 1,  0>\n\
                                 position=<-4,  3> velocity=< 2,  0>\n\
                                 position=<10, -3> velocity=<-1,  1>\n\
                                 position=< 5, 11> velocity=< 1, -2>\n\
                                 position=< 4,  7> velocity=< 0, -1>\n\
                                 position=< 8, -2> velocity=< 0,  1>\n\
                                 position=<15,  0> velocity=<-2,  0>\n\
                                 position=< 1,  6> velocity=< 1,  0>\n\
                                 position=< 8,  9> velocity=< 0, -1>\n\
                                 position=< 3,  3> velocity=<-1,  1>\n\
                                 position=< 0,  5> velocity=< 0, -1>\n\
                                 position=<-2,  2> velocity=< 2,  0>\n\
                                 position=< 5, -2> velocity=< 1,  2>\n\
                                 position=< 1,  4> velocity=< 2,  1>\n\
                                 position=<-2,  7> velocity=< 2, -2>\n\
                                 position=< 3,  6> velocity=<-1, -1>\n\
                                 position=< 5,  0> velocity=< 1,  0>\n\
                                 position=<-6,  0> velocity=< 2,  0>\n\
                                 position=< 5,  9> velocity=< 1, -2>\n\
                                 position=<14,  7> velocity=<-2,  0>\n\
                                 position=<-3,  6> velocity=< 2, -1>\n";

    #[test]
    fn first_example() {
        let output = "#...#..###\n\
                      #...#...#.\n\
                      #...#...#.\n\
                      #####...#.\n\
                      #...#...#.\n\
                      #...#...#.\n\
                      #...#...#.\n\
                      #...#..###\n";

        let (coords, time) = Sky::from(EXAMPLE_INPUT).unwrap().message().unwrap();

        assert_eq!(
            coords.to_string(),
            String::from(output),
            "\n\nInput: \n{}\n\nOutput: \n{}\n\n",
            coords,
            output,
        );

        assert_eq!(time, 3);
    }

    #[test]
    fn test_answer() {
        // GPJLLLLH
        let output = ".####...#####......###..#.......#.......#.......#.......#....#\n\
                      #....#..#....#......#...#.......#.......#.......#.......#....#\n\
                      #.......#....#......#...#.......#.......#.......#.......#....#\n\
                      #.......#....#......#...#.......#.......#.......#.......#....#\n\
                      #.......#####.......#...#.......#.......#.......#.......######\n\
                      #..###..#...........#...#.......#.......#.......#.......#....#\n\
                      #....#..#...........#...#.......#.......#.......#.......#....#\n\
                      #....#..#.......#...#...#.......#.......#.......#.......#....#\n\
                      #...##..#.......#...#...#.......#.......#.......#.......#....#\n\
                      .###.#..#........###....######..######..######..######..#....#\n";

        assert_eq!(answer(), (String::from(output), 10515.to_string()));
    }

    #[bench]
    fn bench_example(b: &mut Bencher) {
        b.iter(|| Sky::from(EXAMPLE_INPUT).unwrap().message().unwrap())
    }

    #[bench]
    fn bench_answer(b: &mut Bencher) {
        b.iter(|| Sky::from(INPUT).unwrap().message().unwrap())
    }

}
