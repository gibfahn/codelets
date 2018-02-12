use std::collections::HashMap;

/// Checks whether a given pattern (`pattern`) matches a input (`s`). The pattern matches if
/// each letter in the pattern makes up a word in the input. For example `abba` matches
/// `redblueredblue` when a=red, b=blue.
pub fn matches(pattern: &str, input: &str) -> bool {
    if pattern.len() > input.len() { return false; }
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
    check_string(pattern, input, map)
}

/// Check a string for a pattern, calls itself recursively.
fn check_string(pattern: &str, input: &str, map: HashMap<char, String>) -> bool {
    if pattern.is_empty() || input.is_empty() {
        return pattern.is_empty() && input.is_empty();
    }
    let p = pattern.chars().nth(0).unwrap(); // Next char in pattern.
    let p_string = map.get(&p).unwrap(); // The word the map thinks p maps to.
    if p_string.is_empty() { // Have not seen pattern before.
        for i in 1..input.len() {
            let mut new_map = map.clone();
            new_map.insert(p, input[0..i].to_string());
            // Try all possible matches for p.
            if check_string(&pattern[1..], &input[i..], new_map.clone()) {
                // Only return true if there are no duplicate values (ignore empty strings).
                let vals: Vec<_> = new_map.values().filter(|x| !x.is_empty()).collect();
                let has_dups = (1..vals.len()).any(|i| vals[i..].contains(&vals[i - 1]));
                if !has_dups { return true; }
            }
        }
        false
    } else { // Have seen pattern before.
        let p_string_len = p_string.len();
        if p_string_len <= input.len() && p_string == &input[0..p_string_len] { // Pattern matches previously seen pattern.
            check_string(&pattern[1..], &input[p_string_len..], map.clone()) // Try rest of string.
        } else { // Pattern doesn't match.
            false
        }
    }
}

#[test]
/// If the pattern has no repeating characters, it matches as long as the string is at least as
/// long as the pattern.
fn test_simple() {
    assert_eq!(matches("a"         , "efghi"                         ) , true  );
    assert_eq!(matches("abdc"      , "odsihpoyywepqriohweoyafpsdoyh" ) , true  );
    assert_eq!(matches("abcdefghi" , "cat"                           ) , false );
}

#[test]
fn test_complex() {
    assert_eq!(matches("abba"      , "redbluebluered"                ) , true  );
    assert_eq!(matches("abba"      , "redbluebluereda"               ) , false );
    assert_eq!(matches("abba"      , "abcxyzxyzabc"                  ) , true  );
    assert_eq!(matches("abba"      , "abcxyzxyzabc"                  ) , true  );
    assert_eq!(matches("baab"      , "abcxyzxyzabc"                  ) , true  );
    assert_eq!(matches("dzzd"      , "abcxyzxyzabc"                  ) , true  );
    assert_eq!(matches("dzzd"      , "dzzda"                         ) , false );
    assert_eq!(matches("aba"       , "patrpatrr"                     ) , false );
    assert_eq!(matches("abba"      , "redbluebluered"                ) , true  );
    assert_eq!(matches("abba"      , "redbluebluereda"               ) , false );
    assert_eq!(matches("abba"      , "abcxyzxyzabc"                  ) , true  );
    assert_eq!(matches("abba"      , "abcxyzxyzabc"                  ) , true  );
    assert_eq!(matches("baab"      , "abcxyzxyzabc"                  ) , true  );
    assert_eq!(matches("dzzd"      , "abcxyzxyzabc"                  ) , true  );
    assert_eq!(matches("dzzd"      , "dzzda"                         ) , false );
    assert_eq!(matches("abba"      , "rblblr"                        ) , true  );
    assert_eq!(matches("abab"      , "redblueredblue"                ) , true  );
    assert_eq!(matches("abba"      , "catdogdogcat"                  ) , true  );
    assert_eq!(matches("abab"      , "redblueredblue"                ) , true  );
    assert_eq!(matches("abab"      , "catdogcatdog"                  ) , true  );
    assert_eq!(matches("aba"       , "catdogcat"                     ) , true  );
    assert_eq!(matches("abba"      , "catdogdogcat"                  ) , true  );
    assert_eq!(matches("abcac"     , "catdogmousecatmouse"           ) , true  );
    assert_eq!(matches("abcde"     , "efghi"                         ) , true  );
    assert_eq!(matches("abab"      , "catdogcatcat"                  ) , false );
    assert_eq!(matches("abab"      , "catdogcatdogg"                 ) , false );
    assert_eq!(matches("abab"      , "catdocatdog"                   ) , false );
    assert_eq!(matches("abab"      , "catdogcat"                     ) , false );
    assert_eq!(matches("abba"      , "redblueredblue"                ) , false );
    assert_eq!(matches("aba"       , "patrpatrr"                     ) , false );
}

#[test]
fn test_dups() {
    assert_eq!(matches("abba"      , "redredredred"                  ) , false );
}
