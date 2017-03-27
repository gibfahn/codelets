use std::fs::File;
use std::io::prelude::Read;

fn main() {
    let mut file = File::open("./input").expect("Unable to open file.");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Could not read file.");
    //println!("Input lines: {}", input.lines().count());
    println!("Puzzle 1: {}", puzzle1(&input));
    println!("Puzzle 2: {}", puzzle2(&input));
}

fn puzzle1(s: &str) -> usize {
    s.lines()
        .map(|l| l.trim())
        .filter(|s| has_tls(s))
        .count()
}

fn puzzle2(s: &str) -> usize {
    s.lines()
        .map(|l| l.trim())
        .filter(|s| has_ssl(s))
        .count()
}

fn has_ssl(s: &str) -> bool {
    let v = s.chars().collect::<Vec<char>>();
    let length = v.len();
    let mut aba: Vec<(char, char)> = Vec::new();
    let mut bab: Vec<(char, char)> = Vec::new();
    let mut in_hypernet = false;
    for i in 0..length {
        if i >= length - 2 {
            break;
        }
        match v[i] {
            '[' => {
                in_hypernet = true;
                continue;
            }
            ']' => {
                in_hypernet = false;
                continue;
            }
            _ => {}
        }

        if v[i..i + 3].iter().any(|&c| c == '[' || c == ']') {
            continue;
        }
        if v[i] == v[i + 2] && v[i] != v[i + 1] {
            if in_hypernet {
                bab.push((v[i], v[i + 1]));
            } else {
                aba.push((v[i], v[i + 1]));
            }
        }
    }
    aba.iter().any(|&x| bab.contains(&(x.1, x.0)))
}

fn has_tls(s: &str) -> bool {
    let v = s.chars().collect::<Vec<char>>();
    let length = v.len();
    let mut in_hypernet = false;
    let mut found_abba = false; // Have we found an ABBA outside hypernet?
    for i in 0..length {
        if i >= length - 3 {
            break;
        }
        match v[i] {
            '[' => {
                in_hypernet = true;
                continue;
            }
            ']' => {
                in_hypernet = false;
                continue;
            }
            _ => {}
        }

        if v[i..i + 4].iter().any(|&c| c == '[' || c == ']') {
            continue;
        }
        if v[i] == v[i + 3] && v[i + 1] == v[i + 2] && v[i] != v[i + 1] {
            if in_hypernet {
                return false;
            } else {
                found_abba = true;
            }
        }
    }
    found_abba
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_tls() {
        assert!(has_tls("abba[mnop]qrst"));
        assert!(!has_tls("abcd[bddb]xyyx"));
        assert!(!has_tls("aaaa[qwer]tyui"));
        assert!(has_tls("ioxxoj[asdfgh]zxcvbn"));
    }

    #[test]
    fn test_has_ssl() {
        assert!(has_ssl("aba[bab]xyz"));
        assert!(!has_ssl("xyx[xyx]xyx"));
        assert!(has_ssl("aaa[kek]eke"));
        assert!(has_ssl("zazbz[bzb]cdb"));
    }
}
