#![feature(external_doc)]
#![doc(include = "../Question.md")]

use std::collections::{hash_map::Entry, HashMap};
use std::fmt;

use failure::{format_err, Error};

const INPUT: &str = include_str!("../input");

pub fn answer() -> (String, String) {
    let mut map = Map::from(INPUT).unwrap();
    (
        map.clone().crash().to_string(),
        map.last_cart_standing().to_string(),
    )
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Turn {
    Left,
    Right,
    Straight,
}

/// Tracks consist of straight paths (| and -), curves (/ and \), and intersections (+). Curves
/// connect exactly two perpendicular pieces of track.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Track {
    Horizontal,
    Vertical,
    ForwardCorner,
    BackCorner,
    Intersection,
}

#[derive(Debug, Clone, Copy)]
struct Cart {
    direction: Direction,
    next_turn: Turn,
}

#[derive(Debug, Clone)]
struct Map {
    tick: u32,
    tracks: Vec<Vec<Option<Track>>>,
    carts: HashMap<Point, Cart>,
}

impl Track {
    fn from(c: char) -> Option<Self> {
        match c {
            '-' | '<' | '>' => Some(Track::Horizontal),
            '|' | '^' | 'v' => Some(Track::Vertical),
            '/' => Some(Track::ForwardCorner),
            '\\' => Some(Track::BackCorner),
            '+' => Some(Track::Intersection),
            _ => None,
        }
    }
}

impl Cart {
    /// Returns Some(Cart) if there's a cart, else None.
    fn from(c: char) -> Option<Self> {
        match c {
            '^' => Some(Cart {
                direction: Direction::Up,
                next_turn: Turn::Left,
            }),
            'v' => Some(Cart {
                direction: Direction::Down,
                next_turn: Turn::Left,
            }),
            '<' => Some(Cart {
                direction: Direction::Left,
                next_turn: Turn::Left,
            }),
            '>' => Some(Cart {
                direction: Direction::Right,
                next_turn: Turn::Left,
            }),
            _ => None,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)?;
        Ok(())
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (y, row) in self.tracks.iter().enumerate() {
            for (x, track) in row.iter().enumerate() {
                write!(
                    f,
                    "{}",
                    if self.carts.contains_key(&Point { x, y }) {
                        match self.carts[&Point { x, y }].direction {
                            Direction::Up => '^',
                            Direction::Right => '>',
                            Direction::Down => 'v',
                            Direction::Left => '<',
                        }
                    } else {
                        match track {
                            Some(Track::Horizontal) => '-',
                            Some(Track::Vertical) => '|',
                            Some(Track::ForwardCorner) => '/',
                            Some(Track::BackCorner) => '\\',
                            Some(Track::Intersection) => '+',
                            None => ' ',
                        }
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn from(s: &str) -> Result<Self, Error> {
        let y_count = s.lines().count();
        let x_count = s
            .lines()
            .next()
            .ok_or_else(|| format_err!("Expected at least one line in map input"))?
            .chars()
            .count();
        let mut tracks: Vec<Vec<Option<Track>>> = vec![Vec::with_capacity(x_count); y_count];
        let mut carts = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                tracks[y].push(Track::from(c));
                if let Some(cart) = Cart::from(c) {
                    carts.insert(Point { x, y }, cart);
                }
            }
        }
        Ok(Map {
            tracks,
            carts,
            tick: 0,
        })
    }

    fn crash(&mut self) -> Point {
        loop {
            let mut carts_vec: Vec<(Point, Cart)> =
                self.carts.iter().map(|(&p, &c)| (p, c)).collect();
            carts_vec.sort_by_key(|(p, _)| (p.y, p.x));
            let mut output = None;
            for (point, cart) in &carts_vec {
                if output.is_some() && &output.unwrap() == point {
                    continue;
                }
                self.carts.remove(point);
                let Point { mut x, mut y } = point;
                match cart.direction {
                    Direction::Up => y -= 1,
                    Direction::Down => y += 1,
                    Direction::Left => x -= 1,
                    Direction::Right => x += 1,
                }
                let new_point = Point { x, y };

                let mut new_direction = cart.direction;
                let mut new_next_turn = cart.next_turn;
                match self.tracks[y][x] {
                    Some(Track::Horizontal) => {
                        debug_assert!(
                            cart.direction == Direction::Left || cart.direction == Direction::Right
                        );
                    }
                    Some(Track::Vertical) => {
                        debug_assert!(
                            cart.direction == Direction::Up || cart.direction == Direction::Down
                        );
                    }
                    Some(Track::ForwardCorner) => {
                        new_direction = match cart.direction {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Down,
                            Direction::Right => Direction::Up,
                        }
                    }
                    Some(Track::BackCorner) => {
                        new_direction = match cart.direction {
                            Direction::Up => Direction::Left,
                            Direction::Down => Direction::Right,
                            Direction::Left => Direction::Up,
                            Direction::Right => Direction::Down,
                        }
                    }
                    Some(Track::Intersection) => match cart.next_turn {
                        Turn::Left => {
                            new_next_turn = Turn::Straight;
                            new_direction = match cart.direction {
                                Direction::Up => Direction::Left,
                                Direction::Down => Direction::Right,
                                Direction::Left => Direction::Down,
                                Direction::Right => Direction::Up,
                            }
                        }
                        Turn::Straight => {
                            new_next_turn = Turn::Right;
                        }
                        Turn::Right => {
                            new_next_turn = Turn::Left;
                            new_direction = match cart.direction {
                                Direction::Up => Direction::Right,
                                Direction::Down => Direction::Left,
                                Direction::Left => Direction::Up,
                                Direction::Right => Direction::Down,
                            }
                        }
                    },
                    None => {
                        panic!("How did we get here?");
                    }
                }

                if let Entry::Occupied(entry) = self.carts.entry(new_point) {
                    output = Some(new_point);
                    entry.remove();
                } else {
                    self.carts.insert(
                        new_point,
                        Cart {
                            direction: new_direction,
                            next_turn: new_next_turn,
                        },
                    );
                }
            }
            if output.is_some() {
                return output.unwrap();
            }
        }
    }

    fn last_cart_standing(&mut self) -> Point {
        loop {
            if self.carts.len() == 1 {
                return self.carts.keys().next().unwrap().to_owned();
            }
            self.crash();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        let input = r#"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   "#;

        assert_eq!(
            Map::from(input).unwrap().clone().crash(),
            Point { x: 7, y: 3 }
        );
    }

    #[test]
    fn test_answer() {
        assert_eq!(answer(), (String::from("83,49"), String::from("73,36")));
    }

    #[test]
    fn second_example() {
        let input = r#"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"#;
        assert_eq!(
            Map::from(input).unwrap().last_cart_standing(),
            Point { x: 6, y: 4 }
        );
    }
}
