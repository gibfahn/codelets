#![feature(inclusive_range_syntax)]
use std::fs::File;
use std::io::prelude::Read;

fn main() {
    let mut file = File::open("./input").expect("Unable to open file.");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Could not read file.");
    //println!("Input lines: {}", input.lines().count());
    println!("Puzzle 1: {}", puzzle1(&input));
}

fn puzzle1(s: &str) -> usize {
    s.lines()
        .map(|l| l.trim())
        .filter(|s| has_tls(s))
        .count()
}

fn has_tls(s: &str) -> bool {
    //println!("Input: '{}'", s);
    let v = s.chars().collect::<Vec<char>>();
    let length = v.len();
    let mut in_hypernet = false;
    let mut found_abba = false; // Have we found an ABBA outside hypernet?
    for i in 0..length {
        if i >= length - 3 { break; }
        match v[i] {
            '[' => { in_hypernet = true; continue; },
            ']' => { in_hypernet = false; continue; },
            _ => {},
        }

        if v[i..i+4].iter().any(|&c| c == '[' || c == ']') { continue; }
        if v[i] == v[i+3] && v[i+1] == v[i+2] && v[i] != v[i+1] {
            //println!("Found abba: {:?}", &v[i..i+4]);
            if in_hypernet {
                return false;
            } else {
                found_abba = true;
            }
        }
    }
    //println!("Found abba: {}", found_abba);
    found_abba
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_abba() {
        assert!(has_tls("abba[mnop]qrst"));
        assert!(!has_tls("abcd[bddb]xyyx"));
        assert!(!has_tls("aaaa[qwer]tyui"));
        assert!(has_tls("ioxxoj[asdfgh]zxcvbn"));
    }
}
