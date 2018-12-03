#![feature(external_doc)]
#![doc(include = "../Question.md")]
use std::collections::HashSet;

const INPUT: &str = include_str!("../input");

pub fn answer() -> (String, String) {
    let changes = Changes::from(INPUT);
    (
        changes.freq().to_string(),
        changes.freq_repeat().to_string(),
    )
}

struct Changes {
    changes: Vec<i32>,
}

impl Changes {
    fn from(input: &str) -> Self {
        Changes {
            changes: input.lines().map(|change| change.parse::<i32>().unwrap()).collect()
        }
    }

    /// Takes an initial value and a newline-delineated set of numbers, and parses it into a set
    /// of numbers. Returns the sum of initial and the change numbers.
    fn freq(&self) -> i32 {
        self.changes.iter().fold(0, |acc, n| acc + n)
    }

    /// Takes an initial value and a newline-delineated set of numbers, and applies the numbers
    /// cyclically to the sum value until a repeated number is reached.
    fn freq_repeat(&self) -> i32 {
        let mut sum = 0;
        let mut seen: HashSet<i32> = HashSet::new();
        seen.insert(0);

        for change in self.changes.iter().cycle() {
            sum += change;
            if !seen.insert(sum) {
                break;
            }
        }
        sum
    }
}

// fn freq(initial: i64, changes: &str) -> i64 {
//     changes
//         .lines()
//         .map(|n| n.parse::<i64>().unwrap())
//         .fold(initial, |acc, n| acc + n)
// }

// fn freq_repeat(initial: i64, changes: &str) -> i64 {
//     let mut sum = initial;
//     let mut seen: HashSet<i64> = HashSet::new();
//     seen.insert(initial);

//     for change in changes
//         .lines()
//         .map(|change| change.parse::<i64>().unwrap())
//         .cycle()
//     {
//         sum += change;
//         if !seen.insert(sum) {
//             break;
//         }
//     }
//     sum
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_examples() {
        assert_eq!(Changes::from("+1\n-2\n+3\n+1\n").freq(), 3);
        assert_eq!(Changes::from("+1\n+1\n+1").freq(), 3);
        assert_eq!(Changes::from("-1\n-2\n-3\n").freq(), -6);
    }

    #[test]
    fn test_answer() {
        assert_eq!(answer(), (String::from("459"), String::from("65474")));
    }

    #[test]
    fn second_examples() {
        assert_eq!(Changes::from("+1\n-2\n+3\n+1\n").freq_repeat(), 2);
        assert_eq!(Changes::from("+1\n-1").freq_repeat(), 0);
        assert_eq!(Changes::from("+3\n+3\n+4\n-2\n-4").freq_repeat(), 10);
        assert_eq!(Changes::from("-6\n+3\n+8\n+5\n-6").freq_repeat(), 5);
        assert_eq!(Changes::from("+7\n+7\n-2\n-7\n-4").freq_repeat(), 14);
    }
}
