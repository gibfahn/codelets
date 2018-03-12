use std::fs::File;
use std::io::prelude::Read;

fn main() {
    let mut file = File::open("./input").expect("Unable to open file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Could not read file");
    println!("Puzzle 1: {}", puzzle(&input));
    println!("Puzzle 2: {}", puzzle2(&input));
}

fn puzzle(s: &str) -> String {
    let mut output = String::new();
    for line in &transpose(s) {
        output.push(most_common(line));
    }
    output
}

fn puzzle2(s: &str) -> String {
    let mut output = String::new();
    for line in &transpose(s) {
        output.push(least_common(line));
    }
    output
}

/// Reads the string vertically and returns it horizontally.
fn transpose(s: &str) -> Vec<String> {
    let length = s.lines()
        .map(|l| l.trim())
        .next()
        .unwrap()
        .len();
    let mut output = vec![String::new(); length];
    for line in s.lines().map(|l| l.trim()) {
        for (n, c) in line.chars().enumerate() {
            output[n].push(c);
        }
    }
    output
}

/// Return most frequent char in the input string. Doesn't handle empty input.
fn most_common(s: &str) -> char {
    use std::collections::HashMap;
    let mut char_map: HashMap<char, u64> = HashMap::new();
    for c in s.chars() {
        *char_map.entry(c).or_insert(0) += 1;
    }
    let (c, _) =
        char_map.iter().fold((' ', 0),
                             |max, (&c, &n)| if n > max.1 { (c, n) } else { max });
    c
}

/// Return least frequent char in the input string. Doesn't handle empty input.
fn least_common(s: &str) -> char {
    use std::collections::HashMap;
    let mut char_map: HashMap<char, u64> = HashMap::new();
    for c in s.chars() {
        *char_map.entry(c).or_insert(0) += 1;
    }
    let (c, _) =
        char_map.iter().fold((' ', std::u64::MAX),
                             |min, (&c, &n)| if n < min.1 { (c, n) } else { min });
    c
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transpose() {
        let input = "eedadn
                     drvtee
                     eandsr
                     raavrd
                     atevrs
                     tsrnev
                     sdttsa
                     rasrtv
                     nssdts
                     ntnada
                     svetve
                     tesnvt
                     vntsnd
                     vrdear
                     dvrsen
                     enarar";
        let expected = ["ederatsrnnstvvde",
                        "eraatsdastvenrvn",
                        "dvnaertssnestdra",
                        "atdvvntrdatnsesr",
                        "desrresttdvvnaea",
                        "nerdsvavsaetdrnr"];
        println!("Transposed: {:?}", transpose(input));
        for (n, s) in transpose(input).iter().enumerate() {
            assert_eq!(s, expected[n]);
        }
    }

    #[test]
    fn test_most_common() {
        assert_eq!(most_common("hello"), 'l');
        assert_eq!(most_common("hhhll"), 'h');
        assert_eq!(most_common("hhlll"), 'l');
    }
}
