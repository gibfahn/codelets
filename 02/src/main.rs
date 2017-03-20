use std::error::Error;
use std::str::FromStr;
use std::fs::File;
use std::io::prelude::*;
use std::fmt;

fn main() {
    let mut file = File::open("./input").expect("Could not open input");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Could not read input file");
    let puzzle = puzzle(&input);
    println!("Second output: {}", puzzle);
}

pub fn puzzle(s: &str) -> String {
    let mut output = String::new();
    let mut key = Key::default();
    for line in s.lines().map(|l| l.trim()) {
        for c in line.chars() {
            key.execute(c.to_string().parse().expect("Bad puzzle input"));
        }
        output += &format!("{}", key);
    }
    output
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Key { One, Two, Three, Four, Five, Six, Seven, Eight, Nine, A, B, C, D }

impl Key {
    pub fn execute(&mut self, instr: Instruction) {
        match instr {
            Instruction::Up => self.move_up(),
            Instruction::Down => self.move_down(),
            Instruction::Left => self.move_left(),
            Instruction::Right => self.move_right(),
        }
    }

    pub fn to_int(&self) -> i32 {
        use Key::*;
        match *self {
            One => 1,
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            A => 10,
            B => 11,
            C => 12,
            D => 13,
        }
    }

    pub fn move_up(&mut self) {
        let i = self.to_int();
        *self = match i {
            3 => 1.to_key(),
            6...8 | 10...12 => (i - 4).to_key(),
            13 => 11.to_key(),
            _ if i < 14 => i.to_key(),
            _ => panic!(format!("Can't move_up: {}", i)),
        }
    }
    pub fn move_down(&mut self) {
        let i = self.to_int();
        *self = match i {
            11 => 13.to_key(),
            2...4 | 6...8 => (i + 4).to_key(),
            1 => 3.to_key(),
            _ if i < 14 => i.to_key(),
            _ => panic!(format!("Can't move_down: {}", i)),
        }
    }
    pub fn move_left(&mut self) {
        let i = self.to_int();
        *self = match i {
            3 | 4 | 6...9 | 11 | 12 => (i - 1).to_key(),
            _ if i < 14 => i.to_key(),
            _ => panic!(format!("Can't move_left: {}", i)),
        }
    }
    pub fn move_right(&mut self) {
        let i = self.to_int();
        *self = match i {
            2 | 3 | 5...8 | 10 | 11 => (i + 1).to_key(),
            _ if i < 14 => i.to_key(),
            _ => panic!(format!("Can't move_right: {}", i)),
        }
    }
}

impl Default for Key {
    fn default() -> Self {
        Key::Five
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output: char = match self.to_int() {
            1...9 => self.to_int().to_string().chars().nth(0).unwrap(),
            10 => 'A',
            11 => 'B',
            12 => 'C',
            13 => 'D',
            _ => panic!(format!("Can't display Key: {:?}", self)),
        };
        write!(f, "{}", output)
    }
}

trait ToKey {
    fn to_key(&self) -> Key;
}

impl ToKey for i32 {
    fn to_key(&self) -> Key {
        use Key::*;
        match *self {
            1 => One,
            2 => Two,
            3 => Three,
            4 => Four,
            5 => Five,
            6 => Six,
            7 => Seven,
            8 => Eight,
            9 => Nine,
            10 => A,
            11 => B,
            12 => C,
            13 => D,
            _ => panic!("Can't deInt that number"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Instruction { Up, Down, Left, Right, }

impl FromStr for Instruction {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            Err(format!("Instruction length not 1: {}", s).into())
        } else {
            match s {
                "U" | "u" => Ok(Instruction::Up),
                "D" | "d" => Ok(Instruction::Down),
                "L" | "l" => Ok(Instruction::Left),
                "R" | "r" => Ok(Instruction::Right),
                _ => Err(format!("Unknown instruction: {}", s).into()),
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_int() {
        use Key::*;
        assert_eq!(Four.to_int(), 4);
        assert_eq!(9.to_key(), Nine);
    }

    #[test]
    fn test_move_up() {
        use Key::*;
        let mut f = Four;
        f.move_up();
        assert_eq!(f, One);
        f.move_down();
        assert_eq!(f, Four);
        f.move_right();
        assert_eq!(f, Five);
        f.move_left();
        assert_eq!(f, Four);
    }
    #[test]
    fn test_puzzle() {
        let input = "ULL
                    RRDDD
                    LURDL
                    UUUUD";
        assert_eq!(puzzle(input), "1985");
    }
}
