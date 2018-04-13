#![feature(inclusive_range_syntax)]
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

fn main() {
    let mut file = File::open("./input").expect("Could not open input");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Could not read input");
    println!("Possible triangles: {}", puzzle(&input));
    println!("Possible triangles 2: {}", puzzle2(&input));
}

pub fn puzzle(s: &str) -> usize {
    s.lines()
        .map(|l| l.trim())
        .map(|l| l.parse::<Triangle>().expect("Couldn't parse triangle"))
        //.inspect(|l| println!("l: {:?}, is_valid: {}", l, l.is_valid()))
        .filter(|l| l.is_valid())
        .count()
}

pub fn puzzle2(s: &str) -> usize {
    let mut new_s = String::new();
    let mut it = s.lines().peekable();
    let mut v = vec![];
    let mut v_newc = vec![];
    loop {
        for _ in 1..=3 {
            v_newc.push(String::new());
        }
        if it.peek() == None {
            break;
        }
        for _ in 1..=3 {
            v.push(it.next().expect("Number of lines in input should be a multiple of 3"));
        }
        for line in &v {
            for (n, word) in line.split_whitespace().enumerate() {
                v_newc[n].push_str(word);
                v_newc[n].push(' ');
            }
        }
        for line in &v_newc {
            new_s.push_str(line);
            new_s.push('\n');
        }

        v_newc.clear();
        v.clear();
    }
    puzzle(&new_s)
}


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Triangle(i32, i32, i32);

impl Triangle {
    pub fn is_valid(&self) -> bool {
        let mut sides = [self.0, self.1, self.2];
        sides.sort();
        //println!("Sides: {:?}", sides);
        sides[2] < sides[0] + sides[1]
    }
}

impl FromStr for Triangle {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.split_whitespace().count() != 3 {
            Err(format!("Couldn't parse into Triangle: {}", s).into())
        } else {
            let mut sides = [0, 0, 0];
            for (n, side) in s.split_whitespace().enumerate() {
                sides[n] = i32::from_str_radix(side, 10)?;
            }
            Ok(Triangle(sides[0], sides[1], sides[2]))
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(!Triangle(1, 2, 3).is_valid());
        assert!(!Triangle(5, 10, 25).is_valid());
        assert!(Triangle(5, 10, 13).is_valid());
    }
    #[test]
    fn test_parse_triangle() {
        assert_eq!("4 5 6".parse::<Triangle>().unwrap(), Triangle(4, 5, 6));
        assert_eq!("4 7 6".parse::<Triangle>().unwrap(), Triangle(4, 7, 6));
        let t = "5 10 25".parse::<Triangle>().unwrap();
        assert!(!t.is_valid());
        assert_eq!(t, Triangle(5, 10, 25));
    }
    #[test]
    fn test_puzzle2() {
        let s = "1 4 7 \n 2 5 8\n 3 6 9";
        let s2 = "1 2 3 \n 4 5 6\n 7 8 9";
        let output = puzzle(s);
        let output2 = puzzle2(s2);
        assert_eq!(output, output2);
    }
}
