#![feature(field_init_shorthand)]
use std::error::Error;
use std::str::FromStr;
use std::fs::File;
use std::io::prelude::*;
use std::cmp;

fn main() {
    let mut file = File::open("./input").expect("Could not open input");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Could not read input file");
    let shortest_path_length = distance_from_origin(travel(&input));
    println!("Shortest path length: {}", shortest_path_length);
    let shortest_path_length = distance_from_origin(travel_no_repeat(&input));
    println!("Shortest path length (no repeat): {}", shortest_path_length);
}

pub fn travel(s: &str) -> Position {
    let mut loc = Position::default();
    for instr in s.split_terminator(", ").map(|s| s.parse().expect("Parse error.")) {
        loc.mv(instr);
    }
    loc
}

pub fn travel_no_repeat(s: &str) -> Position {
    //println!();
    let mut loc = Position::default();
    let mut prev_loc: Position;
    let mut lines: Vec<Line> = Vec::new();
    'instr_loop: for instr in s.split_terminator(", ").map(|s| s.parse().expect("Parse error.")) {
        //println!("loc:  {:?},\tinstr: {:?}", loc, instr);
        prev_loc = loc;
        loc.mv(instr);
        //println!("loc:  {:?},\tinstr: {:?}", loc, instr);
        for line in &lines {
            if line.intersects(Line(prev_loc, loc)) {
                loc = line.intersection(Line(prev_loc, loc));
                //println!("Intersection: {:?}", loc);
                //println!("Line: {:?}", line);
                //println!("Lines: {:?}", lines);
                break 'instr_loop;
            }
        }
        lines.push(Line(prev_loc, loc));
    }
    //println!();
    loc
}

pub fn is_between(x: i32, start: i32, end: i32) -> bool {
    let big;
    let small;
    if start < end {
        small = start;
        big = end
    } else {
        big = start;
        small = end
    }
    x >= small && x <= big
}

pub fn distance(start: Position, end: Position) -> i32 {
    (end.x - start.x).abs() + (end.y - start.y).abs()
}

pub fn distance_from_origin(position: Position) -> i32 {
    distance(Position::default(), position)
}

#[derive(Debug, PartialEq, Copy, Clone)]
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

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Position {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Position {
    pub fn covers(&self, pos: Position) -> bool {
        self.x == pos.x && self.y == pos.y
    }
    pub fn mv(&mut self, instr: Instruction) {
        use Instruction::*;
        use Direction::*;
        let d;
        match instr {
            Left(n) => d = -n,
            Right(n) => d = n,
        }
        match self.dir {
            North => self.x += d,
            South => self.x -= d,
            West => self.y += d,
            East => self.y -= d,
        }
        self.dir.turn(instr);
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Line(Position, Position);

impl Line {
    /// Whether two lines intersect, false if they only touch.
    pub fn intersects(&self, line: Line) -> bool {
        if self.is_horizontal() != line.is_horizontal() &&
           (self.0.covers(line.0) || self.0.covers(line.1) || self.1.covers(line.0) ||
            self.1.covers(line.1)) {
            false
        } else {
            (is_between(self.0.y, line.0.y, line.1.y) &&
             is_between(line.1.x, self.0.x, self.1.x)) ||
            (is_between(self.0.x, line.0.x, line.1.x) && is_between(line.1.y, self.0.y, self.1.y))
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.0.y == self.1.y
    }

    pub fn is_increasing(&self) -> bool {
        let start;
        let end = if self.is_horizontal() {
            start = self.0.y;
            self.1.y
        } else {
            start = self.0.x;
            self.1.x
        };
        end > start
    }

    pub fn intersection(&self, line: Line) -> Position {
        let x;
        let y;
        let dir = line.1.dir;
        //println!();
        //println!("a: {}, b: {}, c: {}",
        //         self.is_horizontal(),
        //         line.is_horizontal(),
        //         line.is_increasing());
        //println!("Self: {:?}\nline: {:?}", self, line);
        //println!();
        match (self.is_horizontal(), line.is_horizontal(), line.is_increasing()) {
            (false, true, _) => {
                x = self.0.x;
                y = line.0.y;
            }
            (true, false, _) => {
                x = line.0.x;
                y = self.0.y;
            }
            (false, false, true) => {
                x = self.0.x;
                y = cmp::min(self.0.y, self.1.y);
            }
            (false, false, false) => {
                x = self.0.x;
                y = cmp::max(self.0.y, self.1.y);
            }
            (true, true, true) => {
                x = cmp::min(self.0.x, self.1.x);
                y = self.0.y;
            }
            (true, true, false) => {
                x = cmp::max(self.0.x, self.1.x);
                y = self.0.y;
            }
        }
        Position { x, y, dir }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
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
    fn test_travel() {
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

    #[test]
    fn travel_no_repeat_vert_cross_up() {
        let position = travel_no_repeat("R8, R4, R4, R8");
        assert_eq!(position,
                   Position {
                       x: 4,
                       y: 0,
                       dir: Direction::North,
                   });
        assert_eq!(distance_from_origin(position), 4);
    }
    #[test]
    fn travel_no_repeat_vert_cross_down() {
        let position = travel_no_repeat("R8, L4, L4, L8, R100");
        assert_eq!(position,
                   Position {
                       x: 4,
                       y: 0,
                       dir: Direction::South,
                   });
        assert_eq!(distance_from_origin(position), 4);
    }
    #[test]
    fn test_intersects() {
        use Direction::*;
        assert!(Line(Position {
                         x: 4,
                         y: 0,
                         dir: West,
                     },
                     Position {
                         x: 4,
                         y: 4,
                         dir: North,
                     })
                        .intersects(Line(Position {
                                             x: 3,
                                             y: 3,
                                             dir: North,
                                         },
                                         Position {
                                             x: 7,
                                             y: 3,
                                             dir: East,
                                         })));
    }
    #[test]
    fn travel_no_repeat_hor_cross_right() {
        let position = travel_no_repeat("R4, L4, L1, L1, L4");
        assert_eq!(position,
                   Position {
                       x: 4,
                       y: 3,
                       dir: Direction::East,
                   });
        assert_eq!(distance_from_origin(position), 7);
    }
    #[test]
    fn travel_no_repeat_hor_cross_left() {
        let position = travel_no_repeat("R4, L4, R1, R3, R3");
        assert_eq!(position,
                   Position {
                       x: 4,
                       y: 1,
                       dir: Direction::West,
                   });
        assert_eq!(distance_from_origin(position), 5);
    }
    #[test]
    fn travel_no_repeat_hor_meet_left() {
        let position = travel_no_repeat("R4, L2, R2, R2, R4");
        assert_eq!(position,
                   Position {
                       x: 4,
                       y: 0,
                       dir: Direction::West,
                   });
        assert_eq!(distance_from_origin(position), 4);
    }
}
