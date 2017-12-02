// use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("First result: {}", captcha(input));
    println!("Second result: {}", captcha2(input));
}

fn captcha2(s: &str) -> u32 {
    let input = s.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let length = input.len();
    let mut matches = Vec::new();
    for (i, c) in input.iter().enumerate() {
        let next = (i + length / 2) % length;
        if c == &input[next] {
            matches.push(*c);
        }
    }
    matches.iter().sum()
}

#[test]
fn example_2_1() {
    assert_eq!(captcha2("1212"), 6);
}

#[test]
fn example_2_2() {
    assert_eq!(captcha2("1221"), 0);
}

#[test]
fn example_2_3() {
    assert_eq!(captcha2("123425"), 4);
}

#[test]
fn example_2_4() {
    assert_eq!(captcha2("123123"), 12);
}

#[test]
fn example_2_5() {
    assert_eq!(captcha2("12131415"), 4);
}


fn captcha(s: &str) -> u32 {
    let input = s.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let length = input.len() - 1;
    let mut matches = Vec::new();
    for (i, c) in input.iter().enumerate() {
        let next = if i == length { 0 } else { i + 1 };
        if c == &input[next] {
            matches.push(*c);
        }
    }
    matches.iter().sum()
}

#[test]
fn example_1_1() {
    assert_eq!(captcha("1122"), 3);
}

#[test]
fn example_1_2() {
    assert_eq!(captcha("1111"), 4);
}

#[test]
fn example_1_3() {
    assert_eq!(captcha("91212129"), 9);
}
