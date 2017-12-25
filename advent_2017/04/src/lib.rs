use std::collections::HashMap;

/// Counts the number of passphrases that are valid (uses `is_valid()` to test).
pub fn count_valid(s: &str) -> usize {
    s.lines().filter(|line| is_valid(line)).count()
}

/// Checks whether a passphrase contains any words that repeat.
pub fn is_valid(s: &str) -> bool {
    let mut seen: HashMap<&str, ()> = HashMap::new();
    s.split_whitespace()
        .all(|word| { if seen.get(word).is_some() {
            false
        } else {
            seen.insert(word, ());
            true
        }
    })
}

/// Counts the number of passphrases that are valid (uses `is_valid2()` to test).
pub fn count_valid2(s: &str) -> usize {
    s.lines().filter(|line| is_valid2(line)).count()
}

/// Checks whether a passphrase contains any words that are anagrams of previous words.
pub fn is_valid2(s: &str) -> bool {
    let mut seen: HashMap<String, ()> = HashMap::new();
    s.split_whitespace()
        .all(|word| {
        let mut word = word.chars().collect::<Vec<char>>();
        word.sort();
        let word = word.into_iter().collect::<String>();
        if seen.get(&word).is_some() {
            false
        } else {
            seen.insert(word.clone(), ());
            true
        }
    })
}



#[test]
fn example_1_1() {
    assert_eq!(is_valid("aa bb cc dd ee"), true);
}

#[test]
fn example_1_2() {
    assert_eq!(is_valid("aa bb cc dd aaa"), true);
}

#[test]
fn example_1_3() {
    assert_eq!(is_valid("aa bb cc dd aa"), false);
}

#[test]
fn problem_1() {
    let input = include_str!("../input").trim();
    assert_eq!(count_valid(input), 466);
}

#[test]
fn example_2_2() {
    assert_eq!(is_valid2("abcde fghij"), true);
}

#[test]
fn example_2_3() {
    assert_eq!(is_valid2("abcde xyz ecdab"), false);
}

#[test]
fn example_2_4() {
    assert_eq!(is_valid2("iiii oiii ooii oooi oooo"), true);
}

#[test]
fn example_2_5() {
    assert_eq!(is_valid2("a ab abc abd abf abj"), true);
}

#[test]
fn example_2_6() {
    assert_eq!(is_valid2("oiii ioii iioi iiio"), false);
}

#[test]
fn problem_2() {
    let input = include_str!("../input").trim();
    assert_eq!(count_valid2(input), 251);
}
