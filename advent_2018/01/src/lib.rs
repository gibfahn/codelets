#![feature(external_doc)]
#![doc(include = "../Question.md")]
use std::collections::HashSet;

pub fn first() -> i64 {
    freq(0, include_str!("../input"))
}

pub fn second() -> i64 {
    freq_repeat(0, include_str!("../input"))
}

/// Takes an initial value and a newline-delineated set of numbers, and parses it into a set
/// of numbers. Returns the sum of initial and the change numbers.
fn freq(initial: i64, changes: &str) -> i64 {
    changes
        .split_terminator('\n')
        .map(|n| n.parse::<i64>().unwrap())
        .fold(initial, |acc, n| acc + n)
}

/// Takes an initial value and a newline-delineated set of numbers, and applies the numbers
/// cyclically to the current value until a repeated number is reached.
fn freq_repeat(initial: i64, changes: &str) -> i64 {
    let mut current = initial;
    let mut seen: HashSet<i64> = HashSet::new();
    seen.insert(current);

    for n in changes
        .split_terminator('\n')
        .map(|n| n.parse::<i64>().unwrap())
        .cycle()
    {
        current += n;
        if seen.contains(&current) {
            break;
        } else {
            seen.insert(current);
        }
    }
    current
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
        assert_eq!(first(), 459);
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
        assert_eq!(second(), 65474);
    }
}
