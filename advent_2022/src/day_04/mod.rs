use color_eyre::{eyre::eyre, Result};

const INPUT: &str = include_str!("./input");

#[derive(Debug)]
struct Section {
    from: u64,
    to: u64,
}
impl Section {
    fn from_str(a: &str) -> Result<Self> {
        let (from, to) = a
            .split_once('-')
            .ok_or_else(|| eyre!("Didn't find '-' in input {a}"))?;
        Ok(Self {
            from: from.parse()?,
            to: to.parse()?,
        })
    }
}

fn is_redundant(a: Section, b: Section) -> bool {
    (a.from >= b.from && a.to <= b.to) || (b.from >= a.from && b.to <= a.to)
}

/// a.to >=b.from
/// a: |   |
/// b:     |   |
///
///
/// a.from <= b.to
/// a:     |   |
/// b: |   |
fn overlap(a: Section, b: Section) -> bool {
    a.to >= b.from && a.from <= b.to
}

pub fn first() -> String {
    INPUT
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .filter(|(a, b)| is_redundant(Section::from_str(a).unwrap(), Section::from_str(b).unwrap()))
        .count()
        .to_string()
}

pub fn second() -> String {
    INPUT
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .filter(|(a, b)| overlap(Section::from_str(a).unwrap(), Section::from_str(b).unwrap()))
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        assert_eq!(
            dbg!(first()),
            advent_of_code::solve(2022, 4, 1, INPUT).unwrap()
        );
        assert_eq!(
            dbg!(second()),
            advent_of_code::solve(2022, 4, 2, INPUT).unwrap()
        );
    }
}
