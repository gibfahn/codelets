use std::{cmp::Ordering, collections::HashSet};

pub const DAY: u8 = 9;
pub const INPUT: &str = include_str!("./input");

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn move_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            'U' => Self::Up,
            'D' => Self::Down,
            _ => panic!("Unexpected input char '{c}'"),
        }
    }
}

#[derive(Debug)]
struct Rope {
    coords: Vec<Coord>,
    tail_visited: HashSet<Coord>,
}

impl Rope {
    fn follow_motions(&mut self, input: &str) {
        for line in input.lines() {
            let mut line_chars = line.chars();
            let direction = Direction::from_char(line_chars.next().unwrap());
            let _ = line_chars.next();
            let distance: u32 = line_chars.collect::<String>().parse().unwrap();
            for _ in 0..distance {
                self.move_rope(direction);
            }
        }
    }

    fn count_tail_visited(&self) -> usize {
        self.tail_visited.len()
    }

    /// If the head is ever two steps directly up, down, left, or right from the tail, the tail must also move one step in that direction so it remains close enough.
    /// Otherwise, if the head and tail aren't touching and aren't in the same row or column, the tail always moves one step diagonally to keep up.
    fn move_rope(&mut self, direction: Direction) {
        self.coords[0].move_direction(direction);

        for i in 1..self.coords.len() {
            let tail = &self.coords[i];
            let head = &self.coords[i - 1];
            if (head.x - tail.x).abs() <= 1 && (head.y - tail.y).abs() <= 1 {
                return;
            }

            let x = match head.x.cmp(&tail.x) {
                Ordering::Less => tail.x - 1,
                Ordering::Equal => tail.x,
                Ordering::Greater => tail.x + 1,
            };

            let y = match head.y.cmp(&tail.y) {
                Ordering::Less => tail.y - 1,
                Ordering::Equal => tail.y,
                Ordering::Greater => tail.y + 1,
            };

            self.coords[i] = Coord { x, y };
        }

        self.tail_visited
            .insert(self.coords.last().unwrap().clone());
    }

    fn with_length(length: usize) -> Self {
        let mut tail_visited = HashSet::new();
        tail_visited.insert(Coord { x: 0, y: 0 });
        let coords = vec![Coord::default(); length];
        Self {
            tail_visited,
            coords,
        }
    }
}

pub fn first(input: &str) -> String {
    let mut rope = Rope::with_length(2);
    rope.follow_motions(input);
    rope.count_tail_visited().to_string()
}

pub fn second(input: &str) -> String {
    let mut rope = Rope::with_length(10);
    rope.follow_motions(input);
    rope.count_tail_visited().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        assert_eq!(
            dbg!(first(INPUT)),
            advent_of_code::solve(2022, DAY, 1, INPUT).unwrap()
        );
        assert_eq!(
            dbg!(second(INPUT)),
            advent_of_code::solve(2022, DAY, 2, INPUT).unwrap()
        );
    }

    #[test]
    fn examples() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(dbg!(first(input)), "13");
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        assert_eq!(dbg!(second(input)), "36");
    }
}
