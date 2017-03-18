use std::error::Error;
use std::str::FromStr;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("./input").expect("Could not open input");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Could not read input file");
    let shortest_path_length = distance_from_origin(travel(&input));
    println!("Shortest path length: {}", shortest_path_length);
}

pub fn travel(s: &str) -> Position {
    let mut loc = Position::default();
    for instr in s.split_terminator(", ").map(|s| s.parse().expect("Parse error.")) {
        loc.mv(instr);
    }
    loc
}

pub fn distance(start: Position, end: Position) -> i32 {
    (end.x - start.x).abs() + (end.y - start.y).abs()
}

pub fn distance_from_origin(position: Position) -> i32 {
    distance(Position::default(), position)
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn turn(&mut self, instr: Instruction) {
        use Instruction::*;
        match instr {
            Left(_) => self.turn_left(),
            Right(_) => self.turn_right(),
        }
    }

    pub fn turn_left(&mut self) {
        use Direction::*;
        *self = match *self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }
    pub fn turn_right(&mut self) {
        use Direction::*;
        *self = match *self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::North
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Position {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Position {
    pub fn mv(&mut self, instr: Instruction) {
        use Instruction::*;
        use Direction::*;
        let d;
        match instr {
            Left(n) => d = -n,
            Right(n) =>  d = n,
        }
        match self.dir {
            North => self.x += d,
            South => self.x -= d,
            West  => self.y += d,
            East  => self.y -= d,
        }
        self.dir.turn(instr);
    }
}
#[derive(Debug, PartialEq)]
pub enum Instruction {
    Left(i32),
    Right(i32),
}

impl FromStr for Instruction {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num: String = s.chars().skip(1).collect();
        if num.is_empty() {
            return Err("Instruction too short.".into());
        }
        match s.chars().next().expect("Empty instruction.") {
            'R' | 'r' => Ok(Instruction::Right(num.parse()?)),
            'L' | 'l' => Ok(Instruction::Left(num.parse()?)),
            _ => unimplemented!(),
        }
    }
}

/// Tests for private Types and Functions.
#[cfg(test)]
mod test {
    use super::*;

    /// Helper function to parse an instruction.
    fn instr_parse(s: &str) -> Instruction {
        s.parse().expect(&format!("Couldn't parse instruction: {}", s))
    }

    #[test]
    fn parse_instructions() {
        assert_eq!(instr_parse("R41"), Instruction::Right(41));
        assert_eq!(instr_parse("L41"), Instruction::Left(41));
    }

    #[test]
    fn process_direction() {
        let position = travel("R2, L3");
        assert_eq!(position,
                   Position {
                       x: 2,
                       y: 3,
                       dir: Direction::North,
                   });
        assert_eq!(distance_from_origin(position), 5);
        let position = travel("R2, R2, R2");
        assert_eq!(position,
                   Position {
                       x: 0,
                       y: -2,
                       dir: Direction::West,
                   });
        assert_eq!(distance_from_origin(position), 2);
        let position = travel("R5, L5, R5, R3");
        assert_eq!(position,
                   Position {
                       x: 10,
                       y: 2,
                       dir: Direction::South,
                   });
        assert_eq!(distance_from_origin(position), 12);
    }
}
