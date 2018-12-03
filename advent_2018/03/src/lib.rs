#![feature(external_doc)]
#![doc(include = "../Question.md")]

use std::collections::{HashMap, HashSet};

use failure::{bail, ensure, Error};
use std::str::FromStr;

const INPUT: &str = include_str!("../input");

pub fn answer() -> (String, String) {
    (
        overlaps(INPUT).unwrap().to_string(),
        no_overlap(INPUT).unwrap().to_string(),
    )
}

#[derive(Debug, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq)]
struct Claim {
    id: u32,
    top_left: Point,
    bottom_right: Point,
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
            bail!(
                "Wrong number of matches found when parsing Claim:\ninput: {:?}, matches: {:?}",
                s,
                matches
            );
        }

        let (id, left_offset, top_offset, width, height) = (
            matches[1].parse::<u32>()?,
            matches[2].parse::<u32>()?,
            matches[3].parse::<u32>()?,
            matches[4].parse::<u32>()?,
            matches[5].parse::<u32>()?,
        );

        Ok(Claim {
            id,
            top_left: Point {
                x: left_offset,
                y: top_offset,
            },
            bottom_right: Point {
                x: left_offset + width,
                y: top_offset + height,
            },
        })
    }
}

/// Given an input that is a newline-separated list of claims, work out how many inches of
/// the cloth have at least two overlapping claims.
fn overlaps(input: &str) -> Result<usize, Error> {
    let (results, errors): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| l.parse::<Claim>())
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
    let claims: Vec<_> = results.into_iter().map(Result::unwrap).collect();

    let mut claim_map = HashMap::new();
    let mut claim_names = HashMap::new();
    let mut overlapping = HashSet::new();
    let mut all = HashSet::new();

    for claim in claims {
        all.insert(claim.id);
        for x in claim.top_left.x..claim.bottom_right.x {
            for y in claim.top_left.y..claim.bottom_right.y {
                *claim_map.entry((x, y)).or_insert(0) += 1;
                if !claim_names.contains_key(&(x, y)) {
                    claim_names.insert((x, y), claim.id);
                } else {
                    overlapping.insert(claim_names[&(x, y)]);
                    overlapping.insert(claim.id);
                }
            }
        }
    }

    Ok(claim_map.values().filter(|v| **v > 1).count())
}

/// Return the ID of the claim with no overlap with any others.
fn no_overlap(input: &str) -> Result<u32, Error> {
    let (results, errors): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| l.parse::<Claim>())
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
    let claims: Vec<_> = results.into_iter().map(Result::unwrap).collect();

    let mut claim_map = HashMap::new();
    let mut claim_names = HashMap::new();
    let mut overlapping = HashSet::new();
    let mut all = HashSet::new();

    for claim in claims {
        all.insert(claim.id);
        for x in claim.top_left.x..claim.bottom_right.x {
            for y in claim.top_left.y..claim.bottom_right.y {
                *claim_map.entry((x, y)).or_insert(0) += 1;
                // TODO(gib): Can we use claim_names.entry((x, y)) here as per clippy?
                if claim_names.contains_key(&(x, y)) {
                    overlapping.insert(claim.id);
                    overlapping.insert(claim_names[&(x, y)]);
                } else {
                    claim_names.insert((x, y), claim.id);
                }
            }
        }
    }

    let non_overlapping: HashSet<_> = all.difference(&overlapping).collect();
    ensure!(
        non_overlapping.len() == 1,
        "Too many non-overlapping values found: {:?}",
        non_overlapping
    );
    Ok(*all.difference(&overlapping).next().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_test() {
        assert_eq!(
            "#1 @ 1,3: 4x4".parse::<Claim>().unwrap(),
            Claim {
                id: 1,
                top_left: Point { x: 1, y: 3 },
                bottom_right: Point { x: 5, y: 7 }
            }
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
    fn test_answer() {
        assert_eq!(answer(), (String::from("124850"), String::from("1097")));
    }

    #[test]
    fn second_example() {
        let input = "#1 @ 1,3: 4x4\n\
                     #2 @ 3,1: 4x4\n\
                     #3 @ 5,5: 2x2\n";
        let output = no_overlap(input);
        assert_eq!(output.unwrap(), 3);
    }
}
