//! [Advent of Code Day 5](http://adventofcode.com/2017/day/5):
//! Works out the number of jumps taken to get out of a provided jump list.

/// Does the test use the first (simpler) problem style or the second (more complex) one?
#[derive (PartialEq)]
pub enum Problem {
    First,
    Second,
}

/// ### Takes:
/// - `s`: a string where each line is an integer (or ignored).  
/// - `p`: Problem type (First or Second).  
///
/// ### Returns:
/// - Number of instructions taken before the instruction pointer leaves the provided instruction
///  set.
pub fn program_count(s: &str, p: &Problem) -> usize {
    let mut position: isize = 0; // Start at the beginning.
    let mut instructions = s.lines()
        .filter_map(|line| line.parse::<isize>().ok())
        .collect::<Vec<isize>>();
    let mut count = 0;
    let program_length = instructions.len() as isize;
    while position >= 0 && position < program_length {
        count += 1;
        let old_pos = position as usize;
        position += instructions[position as usize];
        if p == &Problem::Second && instructions[old_pos] >= 3 {
            instructions[old_pos] -= 1;
        } else {
            instructions[old_pos] += 1;
        }
    }
    count
}

#[test]
fn example_1() {
    let input = "
0
3
0
1
-3";
    assert_eq!(program_count(input, &Problem::First), 5);
}

#[test]
fn problem_1() {
    let input = include_str!("../input");
    assert_eq!(program_count(input, &Problem::First), 372139);
}

#[test]
fn example_2() {
    let input = "
0
3
0
1
-3";
    assert_eq!(program_count(input, &Problem::Second), 10);
}

#[test]
fn problem_2() {
    let input = include_str!("../input");
    assert_eq!(program_count(input, &Problem::Second), 29629538);
}
