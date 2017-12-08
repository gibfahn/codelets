use std::collections::HashMap;

pub enum Problem {
    First,
    Second,
}

/// Parses a list of assembly instructions and returns the largest value stored in a register.
pub fn get_largest(s: &str, p: &Problem) -> isize {
    let mut registers: HashMap<String, isize> = HashMap::new();
    let mut max_val = 0;
    for line in s.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        // println!("Words: {:?}", words);
        assert_eq!(words[3], "if");
        let condition = { // If the condition doesn't match we shouldn't perform the operation.
            let lhs: isize = words[4].parse::<isize>().ok().unwrap_or_else(|| *registers.entry(words[4].to_owned()).or_insert(0));
            let rhs: isize = words[6].parse::<isize>().ok().unwrap_or_else(|| *registers.entry(words[6].to_owned()).or_insert(0));
            match words[5] {
                ">" => lhs > rhs,
                "<" => lhs < rhs,
                "==" => lhs == rhs,
                ">=" => lhs >= rhs,
                "<=" => lhs <= rhs,
                "!=" => lhs != rhs,
                x => panic!(format!("Unknown operator in position 5: {}", x)),
            }
        };
        if condition {
            let register = registers.entry(String::from(words[0])).or_insert(0);
            let number: isize = words[2].parse().unwrap();
            match words[1] {
                "inc" => *register += number,
                "dec" => *register -= number,
                x => panic!(format!("Unknown operator in position 1: {}", x)),
            };
            max_val = std::cmp::max(*register, max_val);
        }
        // println!("Registers: {:?}\n", registers);
    }
    match *p {
        Problem::First => *registers.iter().max_by_key(|&(_, val)| val).unwrap().1,
        Problem::Second => max_val,
    }
}

#[test]
fn example_1() {
    let input = "
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
    assert_eq!(get_largest(input.trim(), &Problem::First), 1);
    assert_eq!(get_largest(input.trim(), &Problem::Second), 10);
}

#[test]
fn problem_1() {
    let input = include_str!("../input.txt");
    assert_eq!(get_largest(input.trim(), &Problem::First), 3612);
}

#[test]
fn problem_2() {
    let input = include_str!("../input.txt");
    assert_eq!(get_largest(input.trim(), &Problem::Second), 3818);
}
