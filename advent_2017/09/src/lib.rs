#[derive (PartialEq)]
pub enum Part { One, Two, }

/// Given a string of input, work out:
/// - How many groups there are (if `p` is `&Part::One`)
/// - How much garbage there is (if `p` is `&Part::Two`)
pub fn groups(s: &str, p: &Part) -> usize {
    let mut group_count = 0;
    let mut garbage_count = 0;
    let mut group_depth = 0;
    let mut garbage = false;
    let mut ignore_next = false;
    for c in s.chars() {
        match c {
            _ if ignore_next => ignore_next = false,
            '!' => ignore_next = true,
            '>' if garbage => garbage = false,
            _ if garbage => garbage_count += 1,
            '<' =>  garbage = true,
            '{' =>  group_depth += 1,
            '}' => { group_count += group_depth; group_depth -= 1;},
            _ => {},
        }
    }
    if p == &Part::One { group_count } else { garbage_count }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_1() {
        assert_eq!(groups("{}", &Part::One), 1);
        assert_eq!(groups("{{{}}}", &Part::One), 6);
        assert_eq!(groups("{{},{}}", &Part::One), 5);
        assert_eq!(groups("{{{},{},{{}}}}", &Part::One), 16);
        assert_eq!(groups("{<a>,<a>,<a>,<a>}", &Part::One), 1);
        assert_eq!(groups("{{<ab>},{<ab>},{<ab>},{<ab>}}", &Part::One), 9);
        assert_eq!(groups("{{<!!>},{<!!>},{<!!>},{<!!>}}", &Part::One), 9);
    }

    #[test]
    fn examples_1_2() {
        assert_eq!(groups("{{<a!>},{<a!>},{<a!>},{<ab>}}", &Part::One), 3);
    }

    #[test]
    fn problem_1() {
        assert_eq!(groups(include_str!("../input"), &Part::One), 9251);
    }

    #[test]
    fn examples_2() {
        assert_eq!(groups("<>", &Part::Two), 0);
        assert_eq!(groups("<random characters>", &Part::Two), 17);
        assert_eq!(groups("<<<<>", &Part::Two), 3);
        assert_eq!(groups("<{!>}>", &Part::Two), 2);
        assert_eq!(groups("<!!>", &Part::Two), 0);
        assert_eq!(groups("<!!!>>", &Part::Two), 0);
        assert_eq!(groups("<{o\"i!a,<{i<a>", &Part::Two), 10);
    }

    #[test]
    fn problem_2() {
        assert_eq!(groups(include_str!("../input"), &Part::Two), 4322);
    }
}

