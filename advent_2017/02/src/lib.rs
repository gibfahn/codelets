use std::cmp;

pub fn checksum(s: &str) -> u32 {
    s.lines().map(|line| {
        line.split_whitespace()
            .fold( (std::u32::MAX, std::u32::MIN), |(min, max), c| {
                let n = c.parse::<u32>().unwrap();
                (cmp::min(min, n), cmp::max(max, n))
            })
    })
    .fold(0, | total, (min, max) | total + max - min)
}

pub fn checksum2(s: &str) -> u32 {
    let mut sum = 0;
    for line in s.lines() {
        let v: Vec<_> = line.split_whitespace()
            .map(|w| w.parse::<u32>().unwrap())
            .collect();
        'line: for small in &v {
            for big in &v {
                if big % small == 0 && big != small {
                    sum += big / small;
                    break 'line;
                }
            }
        }
    }
    sum
}

#[test]
fn example_1() {
    let input =
"5 1 9 5
7 5 3
2 4 6 8";
    assert_eq!(checksum(input), 18);
}

#[test]
fn puzzle_1() {
    let input = include_str!("../input").trim();
    assert_eq!(checksum(input), 44887);
}

#[test]
fn example_2() {
    let input =
"5 9 2 8
9 4 7 3
3 8 6 5";
    assert_eq!(checksum2(input), 9);
}

#[test]
fn puzzle_2() {
    let input = include_str!("../input").trim();
    assert_eq!(checksum2(input), 242);
}
