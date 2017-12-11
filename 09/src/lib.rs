#[derive (PartialEq)]
pub enum Problem {
    First,
    Second,
}

/// Given a string of input, work out:
/// - How many groups there are (if `p` is `&Problem::First`)
/// - How much garbage there is (if `p` is `&Problem::Second`)
pub fn groups(s: &str, p: &Problem) -> usize {
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
    if p == &Problem::First {
        group_count
    } else {
        garbage_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_1() {
        assert_eq!(groups("{}", &Problem::First), 1);
        assert_eq!(groups("{{{}}}", &Problem::First), 6);
        assert_eq!(groups("{{},{}}", &Problem::First), 5);
        assert_eq!(groups("{{{},{},{{}}}}", &Problem::First), 16);
        assert_eq!(groups("{<a>,<a>,<a>,<a>}", &Problem::First), 1);
        assert_eq!(groups("{{<ab>},{<ab>},{<ab>},{<ab>}}", &Problem::First), 9);
        assert_eq!(groups("{{<!!>},{<!!>},{<!!>},{<!!>}}", &Problem::First), 9);
    }

    #[test]
    fn examples_1_2() {
        assert_eq!(groups("{{<a!>},{<a!>},{<a!>},{<ab>}}", &Problem::First), 3);
    }

    #[test]
    fn problem_1() {
        assert_eq!(groups(include_str!("../input.txt"), &Problem::First), 9251);
    }

    #[test]
    fn examples_2() {
        assert_eq!(groups("<>", &Problem::Second), 0);
        assert_eq!(groups("<random characters>", &Problem::Second), 17);
        assert_eq!(groups("<<<<>", &Problem::Second), 3);
        assert_eq!(groups("<{!>}>", &Problem::Second), 2);
        assert_eq!(groups("<!!>", &Problem::Second), 0);
        assert_eq!(groups("<!!!>>", &Problem::Second), 0);
        assert_eq!(groups("<{o\"i!a,<{i<a>", &Problem::Second), 10);
    }

    #[test]
    fn problem_2() {
        assert_eq!(groups(include_str!("../input.txt"), &Problem::Second), 4322);
    }
}

