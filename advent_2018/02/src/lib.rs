#![feature(external_doc)]
#![doc(include = "../Question.md")]

use std::collections::HashMap;

const INPUT: &str = include_str!("../input");

pub fn answer() -> (String, String) {
    (
        two_counts_times_three_counts(INPUT).to_string(),
        common_letters(INPUT).unwrap().to_string(),
    )
}

/// Takes a newline-separated list of IDs, and returns the product of the number of IDs with
/// a letter appearing exactly twice and the number of IDs with a letter appearing exactly
/// thrice.
fn two_counts_times_three_counts(ids: &str) -> u64 {
    let two_three_counts: (u64, u64) = ids.lines().fold((0, 0), |mut acc, line| {
        let counts = counts_map(line);
        if counts.values().any(|n| *n == 2) {
            acc.0 += 1;
        }
        if counts.values().any(|n| *n == 3) {
            acc.1 += 1;
        }
        acc
    });
    two_three_counts.0 * two_three_counts.1
}

/// Takes an ID as a string, and returns a count of the number of occurrences of each letter
/// in the ID.
fn counts_map(id: &str) -> HashMap<char, u32> {
    let mut counts_map = HashMap::new();
    for c in id.chars().filter(|c| !c.is_whitespace()) {
        *counts_map.entry(c).or_insert(0) += 1;
    }
    counts_map
}

fn common_letters(ids: &str) -> Option<String> {
    let ids: Vec<&str> = ids.lines().collect();

    for (i, id) in ids.iter().enumerate() {
        for other in ids.iter().skip(i + 1) {
            let id_len = id.len();
            if id_len != other.len() {
                continue;
            }
            let common: String = id
                .chars()
                .zip(other.chars())
                .filter(|(a, b)| a == b)
                .map(|x| x.0)
                .collect();
            if common.len() == id_len - 1 {
                return Some(common);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        let ids = "abcdef\n\
                   bababc\n\
                   abbcde\n\
                   abcccd\n\
                   aabcdd\n\
                   abcdee\n\
                   ababab\n";

        assert_eq!(two_counts_times_three_counts(ids), 12);
    }

    #[test]
    fn first_answer() {
        assert_eq!(first(), "8398");
    }

    #[test]
    fn second_example() {
        let ids = "abcde\n\
                   fghij\n\
                   klmno\n\
                   pqrst\n\
                   fguij\n\
                   axcye\n\
                   wvxyz\n";
        assert_eq!(common_letters(ids), Some(String::from("fgij")));
    }

    #[test]
    fn second_answer() {
        assert_eq!(second(), String::from("hhvsdkatysmiqjxunezgwcdpr"));
    }
}
