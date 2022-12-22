use color_eyre::{eyre::bail, Result};

const INPUT: &str = include_str!("./input");

pub fn first() -> String {
    INPUT
        .lines()
        .map(|line| {
            let line_len = line.len();
            let (comp_a, comp_b) = line.split_at(line_len / 2);
            let shared = comp_a.chars().find(|c| comp_b.contains(*c)).unwrap();
            priority(shared).unwrap()
        })
        .sum::<u32>()
        .to_string()
}

pub fn second() -> String {
    let mut sum = 0;
    let mut lines = INPUT.lines().peekable();
    while lines.peek().is_some() {
        let (a, b, c) = (
            lines.next().unwrap(),
            lines.next().unwrap(),
            lines.next().unwrap(),
        );
        let badge = a
            .chars()
            .find(|ch| b.contains(*ch) && c.contains(*ch))
            .unwrap();
        sum += priority(badge).unwrap();
    }
    sum.to_string()
}

/// Lowercase item types a through z have priorities 1 through 26.
/// Uppercase item types A through Z have priorities 27 through 52.
fn priority(c: char) -> Result<u32> {
    match c {
        'A'..='Z' => Ok(c as u32 - 65 + 27),
        'a'..='z' => Ok(c as u32 - 96),
        _ => bail!("Unexpected character {c}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        assert_eq!(
            dbg!(first()),
            advent_of_code::solve(2022, 3, 1, INPUT).unwrap()
        );
        assert_eq!(
            dbg!(second()),
            advent_of_code::solve(2022, 3, 2, INPUT).unwrap()
        );
    }
}
