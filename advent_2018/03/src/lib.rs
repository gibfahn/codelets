#![feature(external_doc)]
#![doc(include = "../Question.md")]

use failure::{bail, ensure, Error};
use std::str::FromStr;

const INPUT: &str = include_str!("../input");

pub fn first() -> String {
    overlaps(INPUT).unwrap().to_string()
}

pub fn second() -> String {
    no_overlap(INPUT).unwrap().to_string()
}

#[derive(Debug,PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug,PartialEq)]
struct Claim {
    id: u32,
    top_left: Point,
    bottom_right: Point,
}

impl Claim {
    fn overlaps(&self, other: &Self) -> bool {

        // If one rectangle is on left side of other
        if (self.top_left.x > other.bottom_right.x || self.bottom_right.x > other.top_left.x) {
            return false;
        }

        // If one rectangle is above other
        if (self.top_left.y < other.bottom_right.y || self.bottom_right.y < other.top_left.y) {
            return false;
        }

        return true;
    }
}

impl FromStr for Claim {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let re = regex::Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

        let matches = re.captures(s);
        if matches.is_none() {
            bail!("Failed to parse into a Claim: {:?}", s);
        }
        let matches = matches.unwrap();

        if matches.len() != 6 {
            bail!("Wrong number of matches found when parsing Claim:\ninput: {:?}, matches: {:?}", s, matches);
        }

        let (id, left_offset, top_offset, width, height) = (
            matches[1].parse::<u32>()?,
            matches[2].parse::<u32>()?,
            matches[3].parse::<u32>()?,
            matches[4].parse::<u32>()?,
            matches[5].parse::<u32>()?,
            );

        Ok(Claim {id,
            top_left: Point { x: left_offset, y: top_offset },
            bottom_right: Point { x: left_offset + width, y: top_offset + height },
        })

        /*
        let words: Vec<&str> = s.split_whitespace().collect();
        if words.len() != 4 {
            bail!("Couldn't parse into Claim: {}", s)
        }

        let mut chars = words[0].chars();
        let mut chunk: String = chars.by_ref().take(1).collect();
        ensure!(chunk == "#",
                "Expected first char to be a '#', instead it was a {:?}",
                chunk);
        let id = chars.collect::<String>().parse::<u32>()?;

        ensure!(words[1] == "@",
                "Expected second word to be '@', instead it was {:?}",
                words[1]);

        chars = words[2].chars();
        chunk = chars.by_ref().take_while(|c| *c != ',').collect();
        let left_offset = chunk.parse::<u32>()?;
        chunk = chars.by_ref().take_while(|c| *c != ':').collect();
        let top_offset = chunk.parse::<u32>()?;

        let trailing_chars = chars.collect::<String>();
        ensure!(trailing_chars == "",
                "Third word has trailing characters: {:?}",
                trailing_chars);

        chars = words[3].chars();
        chunk = chars.by_ref().take_while(|c| *c != 'x').collect();
        let width = chunk.parse::<u32>()?;
        let height = chars.collect::<String>().parse::<u32>()?;

        Ok(Claim {id,
            top_left: Point { x: left_offset, y: top_offset },
            bottom_right: Point { x: left_offset + width, y: top_offset + height },
        })
        */
    }
}

/// Given an input that is a newline-separated list of claims, work out how many inches of
/// the cloth have at least two overlapping claims.
fn overlaps(input: &str) -> Result<usize, Error> {
    let (results, errors): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| l.parse::<Claim>())
        .partition(Result::is_ok);
    if ! errors.is_empty() {
        bail!("{:#?}", errors.into_iter().map(Result::unwrap_err).collect::<Vec<_>>());
    }
    let claims: Vec<_> = results.into_iter().map(Result::unwrap).collect();

    let max_x = claims.iter().max_by_key(|c| c.bottom_right.x).unwrap().bottom_right.x + 1;
    let max_y = claims.iter().max_by_key(|c| c.bottom_right.y).unwrap().bottom_right.y + 1;

    let mut cloth = vec![vec![0_u8; max_x as usize]; max_y as usize];

    for claim in claims {
        // println!("{:?}", claim);
        for y in claim.top_left.y..claim.bottom_right.y {
            for x in claim.top_left.x..claim.bottom_right.x {
                // println!("{}, {}", y, x);
                cloth[y as usize][x as usize] = cloth[y as usize][x as usize].saturating_add(1);
            }
        }
    }

    // println!("\n");
    // for row in &cloth {
    //     for val in row {
    //         print!("{}", val);
    //     }
    //     println!("");
    // }
    // println!("\n\n Cloth:\n{:?}", cloth);
    Ok(cloth.iter().flat_map(|row| row.iter()).filter(|count| **count > 1).count())
}

/// Return the ID of the claim with no overlap with any others.
fn no_overlap(input: &str) -> Result<u32, Error> {
    let (results, errors): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| l.parse::<Claim>())
        .partition(Result::is_ok);
    if ! errors.is_empty() {
        bail!("{:#?}", errors.into_iter().map(Result::unwrap_err).collect::<Vec<_>>());
    }
    let claims: Vec<_> = results.into_iter().map(Result::unwrap).collect();

    let max_x = claims.iter().max_by_key(|c| c.bottom_right.x).unwrap().bottom_right.x + 1;
    let max_y = claims.iter().max_by_key(|c| c.bottom_right.y).unwrap().bottom_right.y + 1;

    let mut output = Vec::new();
    for (n, claim) in claims.iter().enumerate() {
        let mut overlaps = false;
        for (m, other_claim) in claims.iter().enumerate() {
            if n == m {
                continue;
            }
            if claim.overlaps(&other_claim) {
                overlaps = true;
            }
        }
        if overlaps == false {
            output.push(claim.id);
        }
    }
    ensure!(output.len() == 1,
            "Wrong number of non-overlapping claims found, expected 1, found {:?}",
            output);
    Ok(output[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_test() {
        assert_eq!("#1 @ 1,3: 4x4".parse::<Claim>().unwrap(),
            Claim {id: 1, top_left: Point {x: 1, y: 3}, bottom_right: Point {x: 5, y: 7}}
            );
    }

    #[test]
    fn first_example() {
        let input = "#1 @ 1,3: 4x4\n\
                     #2 @ 3,1: 4x4\n\
                     #3 @ 5,5: 2x2\n";
        let output = overlaps(input);
        assert_eq!(output.unwrap(), 4);
    }

    #[test]
    fn first_answer() {
        assert_eq!(&first(), "124850");
    }

    #[test]
    fn second_example() {
        let input = "#1 @ 1,3: 4x4\n\
                     #2 @ 3,1: 4x4\n\
                     #3 @ 5,5: 2x2\n";
        let output = no_overlap(input);
        assert_eq!(output.unwrap(), 3);
    }

    #[test]
    fn second_answer() {
        assert_eq!(&second(), "1097");
    }
}
