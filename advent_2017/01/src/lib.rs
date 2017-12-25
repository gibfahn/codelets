pub fn captcha1(s: &str) -> u32 {
    captcha(s, 1)
}

pub fn captcha2(s: &str) -> u32 {
    captcha(s, s.len() / 2)
}

pub fn captcha(s: &str, skip: usize) -> u32 {
    let input = s.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let length = input.len();
    let mut matches = Vec::new();
    for (i, c) in input.iter().enumerate() {
        let next = (i + skip) % length;
        if c == &input[next] {
            matches.push(*c);
        }
    }
    matches.iter().sum()
}

#[test]
fn example_1_1() {
    assert_eq!(captcha1("1122"), 3);
}

#[test]
fn example_1_2() {
    assert_eq!(captcha1("1111"), 4);
}

#[test]
fn example_1_3() {
    assert_eq!(captcha1("91212129"), 9);
}

#[test]
fn question_1() {
    assert_eq!(captcha1(include_str!("../input").trim()), 1182);
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

#[test]
fn question_2() {
    assert_eq!(captcha2(include_str!("../input").trim()), 1152);
}

