#![feature(external_doc)]
#![doc(include = "../Question.md")]
use std::collections::HashSet;

const INPUT: &str = include_str!("../input");

pub fn first() -> String {
    freq(0, INPUT).to_string()
}

pub fn second() -> String {
    freq_repeat(0, INPUT).to_string()
}

/// Takes an initial value and a newline-delineated set of numbers, and parses it into a set
/// of numbers. Returns the sum of initial and the change numbers.
fn freq(initial: i64, changes: &str) -> i64 {
    changes
        .lines()
        .map(|n| n.parse::<i64>().unwrap())
        .fold(initial, |acc, n| acc + n)
}

/// Takes an initial value and a newline-delineated set of numbers, and applies the numbers
/// cyclically to the sum value until a repeated number is reached.
fn freq_repeat(initial: i64, changes: &str) -> i64 {
    let mut sum = initial;
    let mut seen: HashSet<i64> = HashSet::new();
    seen.insert(initial);

    for change in changes
        .lines()
        .map(|change| change.parse::<i64>().unwrap())
        .cycle()
    {
        sum += change;
        if !seen.insert(sum) {
            break;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_examples() {
        assert_eq!(freq(0, "+1\n-2\n+3\n+1\n"), 3);
        assert_eq!(freq(0, "+1\n+1\n+1"), 3);
        assert_eq!(freq(0, "-1\n-2\n-3\n"), -6);
    }

    #[test]
    fn first_answer() {
        assert_eq!(first(), "459");
    }

    #[test]
    fn second_examples() {
        assert_eq!(freq_repeat(0, "+1\n-2\n+3\n+1\n"), 2);
        assert_eq!(freq_repeat(0, "+1\n-1"), 0);
        assert_eq!(freq_repeat(0, "+3\n+3\n+4\n-2\n-4"), 10);
        assert_eq!(freq_repeat(0, "-6\n+3\n+8\n+5\n-6"), 5);
        assert_eq!(freq_repeat(0, "+7\n+7\n-2\n-7\n-4"), 14);
    }

    #[test]
    fn second_answer() {
        assert_eq!(second(), "65474");
    }
}
