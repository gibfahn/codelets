#![feature(external_doc)]
#![doc(include = "../Question.md")]

use std::cmp::{max,min};
use std::str::FromStr;

use failure::{Error, format_err, bail};

const INPUT: &str = include_str!("../input");

pub fn answer() -> (String, String) {
    (String::from(""), String::from(""))
}

#[derive(Debug, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn distance(&self, p: Self) -> u32 {
        max(self.x, p.x) - min(self.x, p.x) +
            max(self.y, p.y) - min(self.y, p.y)
    }
}

impl FromStr for Point {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s.split_terminator(", ").map(|w| w.parse::<u32>()).collect::<Result<Vec<u32>, _>>()?;
        if numbers.len() != 2 {
            bail!("Wrong number of numbers found, should be two, found: {:?}", numbers);
        }
        Ok(Point {
            x: numbers[0],
            y: numbers[1],
        })
    }
}

fn safest_place(s: &str) -> Result<usize, Error> {
    let (points, errors): (Vec<_>, Vec<_>) = s.lines().map(|l| l.parse::<Point>()).partition(Result::is_ok);

    let points: Vec<_> = points.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();

    if !errors.is_empty() {
        bail!("Parsing errors: {:?}", errors);
    }
    Ok(4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {

        let input = "1, 1\n\
                     1, 6\n\
                     8, 3\n\
                     3, 4\n\
                     5, 5\n\
                     8, 9\n";

        assert_eq!(safest_place(input).unwrap(), 17);

    }

    #[test]
    fn test_answer() {
        assert_eq!(answer(), (String::from(""), String::from("")));
    }

    #[test]
    fn second_example() {}
}
