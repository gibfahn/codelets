use std::collections::HashMap;

pub fn matches(pattern: &str, s: &str) -> bool {
    let mut pattern_strings: HashMap<char, String> = HashMap::new();
    let mut repeated_chars = false;
    for c in pattern.chars() {
        if pattern_strings.get(&c).is_some() {
            repeated_chars = true;
        } else {
            pattern_strings.insert(c, String::new());
        }
    }
    if ! repeated_chars { return true; }

    println!("\nPattern strings: {:?}", pattern_strings);
    check_string(pattern, s, pattern_strings)
    // println!("\nMatching pattern: {}, s: {}", pattern, s);
}

pub fn check_string(pattern: &str, s: &str,
                pattern_strings: HashMap<char, String>) -> bool {
    println!("\n▶ {} - {} -> {:?}", pattern, s, pattern_strings);
    if pattern.is_empty() || s.is_empty() {
        println!("Done: {}", pattern.is_empty() && s.is_empty());
        return pattern.is_empty() && s.is_empty();
    }
    let p_next = pattern.chars().next().unwrap();
    let p_match = pattern_strings.get(&p_next).unwrap();
    println!("p_next: {:?}, p_match: {:?}", p_next, p_match);
    if p_match != "" { // Have seen pattern before.
        println!("{:?} == {:?} -> {}", p_match, "", p_match.len() == 0);
        print!("Existing pattern pattern {:?}, checking further: ", p_next);
        // println!("{:?} == {:?} -> {}", p_match, p_match.len() <= s.len() && &s[0..p_match.len()], &s[0..p_match.len()] == p_match);
        if p_match.len() <= s.len() && &s[0..p_match.len()] == p_match { // Pattern matches previously seen pattern.
            check_string(&pattern[1..], &s[p_match.len()..], pattern_strings.clone())
        } else { // Pattern doesn't match.
            println!("False");
            false
        }
    } else { // Have not seen pattern before.
        for i in 1..s.len() {
            let mut new_pattern_strings = pattern_strings.clone();
            new_pattern_strings.insert(p_next, s[0..i].to_string());
            println!("New pattern {:?}, checking further {:?}", p_next, pattern_strings);
            if check_string(&pattern[1..], &s[i..], new_pattern_strings) {
                println!("True");
                return true;
            }
        }
        println!("All checked, False");
        false
    }
    // if let Some(p) = pattern.chars().next() {
    //     if let Some(c) = s.chars().next() {
    //         let x = pattern_strings.entry(p).or_insert_with(String::new);
    //         (*x).push(c);
    //     }
    // }
    // if s.len() > 0 && pattern.len() > 0 {
    //     for i in 1..s.len() {
    //         check_string(&pattern[1..], &s[i..], pattern_strings.clone());
    //     }
    // }
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
