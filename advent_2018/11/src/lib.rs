#![feature(external_doc)]
#![doc(include = "../Question.md")]

// Looks like you're supposed to use this: https://en.wikipedia.org/wiki/Summed-area_table

use failure::Error;
use rayon::prelude::*;

use std::i32;

const INPUT: &str = include_str!("../input");

pub fn answer() -> (String, String) {
    let grid = Grid::from(INPUT).unwrap();
    let answer_1 = grid.total_power(3).0;
    let answer_2 = grid.largest_total_power();
    (
        format!("{},{}", answer_1.0, answer_1.1),
        format!("{},{},{}", (answer_2.0).0, (answer_2.0).1, answer_2.2),
    )
}

#[derive(Debug)]
struct Grid {
    points: Vec<Vec<i32>>,
    serial_number: i32,
}

impl Grid {
    fn from(s: &str) -> Result<Self, Error> {
        let serial_number = s.trim_end().parse::<i32>()?;
        let points: Vec<Vec<i32>> = (0..301)
            .into_par_iter()
            .map(|y| {
                (0..301)
                    .into_par_iter()
                    .map(|x| power_level((x, y), serial_number))
                    .collect()
            })
            .collect();
        Ok(Grid {
            points,
            serial_number,
        })
    }

    fn largest_total_power(&self) -> ((usize, usize), i32, usize) {
        (0_usize..300_usize)
            .into_par_iter()
            .map(|n| {
                let p = self.total_power(n);
                (((p.0).0, (p.0).1), p.1, n)
            })
            .max_by(|x, y| x.1.cmp(&y.1))
            .unwrap()
    }

    fn total_power(&self, n: usize) -> ((usize, usize), i32) {
        (0..(300 - n))
            .flat_map(|y| (0..(300 - n)).map(move |x| (x, y)))
            .fold(((0, 0), 0), |acc, (x, y)| {
                let mut total_power = 0_i32;
                for j in 0..n {
                    for i in 0..n {
                        total_power += self.points[y + j][x + i];
                    }
                }

                if total_power >= acc.1 {
                    ((x, y), total_power)
                } else {
                    acc
                }
            })
    }
}

/// ((x + 10) * y + s) * (x + 10)
/// (x + 10)²y + (x + 10)s
/// (xy + 10y + s) * (x + 10)
/// (xxy + 10xy + xs + 10xy + 100y + 10s)
/// (xxy + 20xy + 100y + xs + 10s)
fn power_level((x, y): (i32, i32), serial_number: i32) -> i32 {
    let s = serial_number;

    let pwr = x * x * y + 20 * x * y + 100 * y + x * s + 10 * s;
    (pwr % 1000 - pwr % 100) / 100 - 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        assert_eq!(power_level((3, 5), 8), 4);
        assert_eq!(Grid::from("18").unwrap().total_power(3), ((33, 45), 29));
        assert_eq!(Grid::from("42").unwrap().total_power(3), ((21, 61), 30));
    }

    #[test]
    fn test_answer() {
        assert_eq!(
            answer(),
            (String::from("21,61"), String::from("232,251,12"))
        );
    }

    #[test]
    fn second_example() {
        assert_eq!(
            Grid::from("18").unwrap().largest_total_power(),
            ((90, 269), 113, 16)
        );
        assert_eq!(
            Grid::from("42").unwrap().largest_total_power(),
            ((232, 251), 119, 12)
        );
    }
}
