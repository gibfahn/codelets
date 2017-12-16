use std::collections::HashMap;

/// Checks whether a given pattern (`pattern`) matches a string (`s`). The pattern matches if
/// each letter in the pattern makes up a word in the string. For example `abba` matches
/// `redblueredblue` when a=red, b=blue.
pub fn matches(pattern: &str, string: &str) -> bool {
    let mut map = HashMap::new(); // Map of pattern char -> Word.
    let mut repeated_chars = false;
    for p in pattern.chars() {
        if map.get(&p).is_some() {
            repeated_chars = true;
        } else {
            map.insert(p, String::new());
        }
    }
    if ! repeated_chars { return true; }
    check_string(pattern, string, map)
}

fn check_string(pattern: &str, string: &str, map: HashMap<char, String>) -> bool {
    if pattern.is_empty() || string.is_empty() {
        return pattern.is_empty() && string.is_empty();
    }
    let p = pattern.chars().nth(0).unwrap(); // Next char in pattern.
    let p_string = map.get(&p).unwrap(); // The word the map thinks p maps to.
    if p_string.is_empty() { // Have not seen pattern before.
        for i in 1..string.len() {
            let mut new_map = map.clone();
            new_map.insert(p, string[0..i].to_string());
            // Try all possible matches for p.
            if check_string(&pattern[1..], &string[i..], new_map) {
                return true;
            }
        }
        false
    } else { // Have seen pattern before.
        let p_string_len = p_string.len();
        if p_string_len <= string.len() && p_string == &string[0..p_string_len] { // Pattern matches previously seen pattern.
            check_string(&pattern[1..], &string[p_string_len..], map.clone()) // Try rest of string.
        } else { // Pattern doesn't match.
            false
        }
    }
}

#[test]
fn test_1() {
    assert_eq!(matches("abdc", "odsihpoyywepqriohweoyafpsdoyh"), true);
}

#[test]
fn test_2() {
    assert_eq!(matches("abba", "redbluebluered"), true);
    assert_eq!(matches("abba", "redbluebluereda"), false);
    assert_eq!(matches("abba", "abcxyzxyzabc"), true);
    assert_eq!(matches("abba", "abcxyzxyzabc"), true);
    assert_eq!(matches("baab", "abcxyzxyzabc"), true);
    assert_eq!(matches("dzzd", "abcxyzxyzabc"), true);
    assert_eq!(matches("dzzd", "dzzda"), false);
}
