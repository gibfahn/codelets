#![feature(external_doc)]
#![doc(include = "../Question.md")]
#![feature(range_contains)]

use std::cmp;
use std::collections::{HashSet, VecDeque};
use std::mem;

use failure::{bail, ensure, format_err, Error};

const INPUT: &str = include_str!("../input");

pub fn answer() -> (String, String) {
    let cave = Cave::from(INPUT).unwrap();
    (
        cave.gen_sum(20).unwrap().to_string(),
        cave.gen_sum(50_000_000_000).unwrap().to_string(),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pot {
    Full,
    Empty,
}

#[derive(Debug)]
struct Cave {
    /// The initial set of filled pots.
    initial: Vec<Pot>,
    /// Combinations of pots which result in a filled pot in the next generation.
    rules: HashSet<[Pot; 5]>,
}

impl Default for Pot {
    fn default() -> Self {
        Pot::Empty
    }
}

impl Pot {
    fn from(c: char) -> Result<Self, Error> {
        match c {
            '#' => Ok(Pot::Full),
            '.' => Ok(Pot::Empty),
            _ => bail!("Invalid input for Pot: '{}'", c),
        }
    }
}

/// Parse a rule of the form: `...## => #` into an Array of five Pots and an output Pot, where '#'
/// is Pot::Full and '.' is Pot::Empty.
fn parse_rule(s: &str) -> Result<([Pot; 5], Pot), Error> {
    let mut words = s.split_whitespace();
    let first_word = words
        .next()
        .ok_or_else(|| format_err!("Rule requires at least one word to parse."))?;

    ensure!(
        first_word.len() == 5,
        "First word in the rule should have a length of 5"
    );

    let mut matcher_pots = first_word.chars().map(Pot::from);

    let matcher = [
        matcher_pots.next().unwrap()?,
        matcher_pots.next().unwrap()?,
        matcher_pots.next().unwrap()?,
        matcher_pots.next().unwrap()?,
        matcher_pots.next().unwrap()?,
    ];

    ensure!(
        words.next() == Some("=>"),
        "Second word in rule should be '=>'."
    );

    let third_word = words
        .next()
        .ok_or_else(|| format_err!("Rule should have three words"))?;

    ensure!(
        third_word.len() == 1,
        "Third word should be a single character."
    );

    let output = Pot::from(third_word.chars().next().unwrap())?;

    Ok((matcher, output))
}

impl Cave {
    fn from(s: &str) -> Result<Self, Error> {
        let mut lines = s.lines();
        let first_line = lines
            .next()
            .ok_or_else(|| format_err!("No lines in input."))?;

        let mut first_chars = first_line.chars();
        ensure!(
            first_chars.by_ref().take(15).eq("initial state: ".chars()),
            "First line should start with 'initial state: '"
        );

        // Return the first parsing error if there is one.
        let initial = first_chars.map(Pot::from).collect::<Result<Vec<_>, _>>()?;

        ensure!(lines.next() == Some(""), "Second line should be blank");

        let rules: HashSet<[Pot; 5]> = lines
            .map(|l| parse_rule(l))
            // Only store the rules that map to a filled pot.
            .filter_map(|rule| match rule {
                Ok(r) if r.1 == Pot::Full => Some(Ok(r.0)),
                Err(e) => Some(Err(e)),
                _ => None,
            })
            .collect::<Result<HashSet<_>, _>>()?;

        Ok(Cave { initial, rules })
    }

    fn gen_sum(&self, generations: usize) -> Result<i64, Error> {
        ensure!(
            !self.rules.contains(&[Pot::Empty; 5]),
            "Infinite plant growth possible"
        );

        let initial_length = self.initial.len();
        let mut first_index = 4;
        let mut pots_length = initial_length + 8;
        let mut state = VecDeque::with_capacity(pots_length + cmp::min(1000, generations));
        state.extend((0..4).map(|_| Pot::Empty));
        state.extend(&self.initial);
        state.extend((0..4).map(|_| Pot::Empty));

        let mut prev = state.clone();
        let mut products = VecDeque::with_capacity(20);

        for gen in 0..generations {
            mem::swap(&mut state, &mut prev);
            state.clear();
            state.extend((0..pots_length).map(|n| {
                if (2..pots_length - 2).contains(&n)
                    && self.rules.contains(&[
                        prev[n - 2],
                        prev[n - 1],
                        prev[n],
                        prev[n + 1],
                        prev[n + 2],
                    ])
                {
                    Pot::Full
                } else {
                    Pot::Empty
                }
            }));
            if let Some(n) = state.iter().take(4).position(|p| p == &Pot::Full) {
                let needed = 4 - n;
                first_index += needed;
                pots_length += needed;
                for _ in 0..needed {
                    state.push_front(Pot::Empty);
                }
            }
            if let Some(n) = state.iter().rev().take(4).position(|p| p == &Pot::Full) {
                let needed = 4 - n;
                pots_length += needed;
                for _ in 0..needed {
                    state.push_back(Pot::Empty);
                }
            }
            debug_assert_eq!(pots_length, state.len());

            let total = state
                .iter()
                .enumerate()
                .filter_map(|(n, p)| match p {
                    Pot::Full => Some(n as i64 - first_index as i64),
                    Pot::Empty => None,
                })
                .sum::<i64>();

            if gen > 20 {
                products.pop_front();
            }
            products.push_back(total);
            if gen > 1 {
                let diff = products[1] - products[0];
                if products
                    .iter()
                    .zip(products.iter().skip(1))
                    .all(|(a, b)| b - a == diff)
                {
                    return Ok(total + (generations as i64 - gen as i64 - 1) * diff);
                }
            }
        }

        let out = state
            .iter()
            .enumerate()
            .filter_map(|(n, p)| match p {
                Pot::Full => Some(n as i64 - first_index as i64),
                Pot::Empty => None,
            })
            .sum();
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        let input = "initial state: #..#.#..##......###...###\n\
                     \n\
                     ...## => #\n\
                     ..#.. => #\n\
                     .#... => #\n\
                     .#.#. => #\n\
                     .#.## => #\n\
                     .##.. => #\n\
                     .#### => #\n\
                     #.#.# => #\n\
                     #.### => #\n\
                     ##.#. => #\n\
                     ##.## => #\n\
                     ###.. => #\n\
                     ###.# => #\n\
                     ####. => #\n";

        assert_eq!(Cave::from(input).unwrap().gen_sum(20).unwrap(), 325);
    }

    #[test]
    fn test_answer() {
        assert_eq!(
            answer(),
            (String::from("2349"), String::from("2100000001168"))
        );
    }
}
