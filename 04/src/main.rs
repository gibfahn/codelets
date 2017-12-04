use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}


/// Checks whether a passphrase contains any words that repeat.
pub fn is_valid(s: &str) -> bool {
    let mut seen: HashMap<&str, ()> = HashMap::new();
    for word in s.split_whitespace() {
        if seen.get(word).is_some() {
            return false;
        } else {
            seen.insert(word, ());
        }
    }
    true
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
