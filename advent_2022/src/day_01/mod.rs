const INPUT: &str = include_str!("./input");

pub fn first() -> String {
    INPUT
        .split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .map(|line| line.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn second() -> String {
    let mut values = INPUT
        .split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .map(|line| line.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .collect::<Vec<_>>();
    values.sort();
    values.iter().rev().take(3).sum::<u64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        assert_eq!(
            dbg!(first()),
            advent_of_code::solve(2022, 1, 1, INPUT).unwrap()
        );
        assert_eq!(
            dbg!(second()),
            advent_of_code::solve(2022, 1, 2, INPUT).unwrap()
        );
    }
}
