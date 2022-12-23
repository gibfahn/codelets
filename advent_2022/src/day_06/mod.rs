use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("./input");

fn non_repeating_point(length: usize) -> String {
    let mut queue = VecDeque::new();
    let mut set: HashSet<char> = HashSet::new();
    for (i, c) in INPUT.chars().enumerate() {
        queue.push_back(c);
        if queue.len() < length {
            continue;
        }
        if queue.len() == length + 1 {
            let _ = queue.pop_front();
        }
        set.clear();
        set.extend(queue.iter());
        if set.len() == length {
            return (i + 1).to_string();
        }
    }
    panic!("How did we get here?");
}

pub fn first() -> String {
    non_repeating_point(4)
}

pub fn second() -> String {
    non_repeating_point(14)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        assert_eq!(
            dbg!(first()),
            advent_of_code::solve(2022, 6, 1, INPUT).unwrap()
        );
        assert_eq!(
            dbg!(second()),
            advent_of_code::solve(2022, 6, 2, INPUT).unwrap()
        );
    }
}
