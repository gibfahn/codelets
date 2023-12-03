const INPUT: &str = include_str!("./input");

const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn first() -> String {
    INPUT
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut line_iter = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| char::to_digit(c, 10).unwrap());
            let first = line_iter.next().unwrap();
            let last = line_iter.last().unwrap_or(first);
            first * 10 + last
        })
        .sum::<u32>()
        .to_string()
}

pub fn second() -> String {
    let digits_slices = DIGITS.map(|digit_str| digit_str.chars().collect::<Vec<_>>());
    INPUT
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut chars: Vec<_> = line.chars().collect();
            let mut digits = Vec::new();
            while !chars.is_empty() {
                if chars[0].is_ascii_digit() {
                    digits.push(char::to_digit(chars.remove(0), 10).unwrap());
                    continue;
                }
                for (i, digit_slice) in digits_slices.iter().enumerate() {
                    if chars.starts_with(digit_slice) {
                        digits.push(i as u32);
                    }
                    continue;
                }
                chars.remove(0);
            }
            let first = digits[0];
            let last = digits.last().unwrap();
            first * 10 + last
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        assert_eq!(dbg!(first()), "55029",);
        assert_eq!(dbg!(second()), "55686",);
    }
}
