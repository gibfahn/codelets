use std::collections::HashSet;

const INPUT: &str = include_str!("./input");

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

    fn from_tuple(tuple: (i32, i32)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
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

#[derive(Debug, Default)]
struct Rope {
    head: Coord,
    tail: Coord,
    tail_visited: HashSet<Coord>,
}

impl Rope {
    fn follow_motions(&mut self, input: &str) {
        self.tail_visited.insert(self.tail.clone());
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

    fn move_rope(&mut self, direction: Direction) {
        self.head.move_direction(direction);

        if (self.head.x - self.tail.x).abs() <= 1 && (self.head.y - self.tail.y).abs() <= 1 {
            return;
        }

        if ((direction == Direction::Left || direction == Direction::Right)
            && self.head.y == self.tail.y)
            || ((direction == Direction::Up || direction == Direction::Down)
                && self.head.x == self.tail.x)
        {
            self.tail.move_direction(direction);
        } else {
            let (x, y) = (self.head.x, self.head.y);
            self.tail = Coord::from_tuple(match direction {
                Direction::Up => (x, y - 1),
                Direction::Down => (x, y + 1),
                Direction::Left => (x + 1, y),
                Direction::Right => (x - 1, y),
            });
        }

        self.tail_visited.insert(self.tail.clone());
    }
}

pub fn first(input: &str) -> String {
    let mut rope = Rope::default();
    rope.follow_motions(input);
    rope.count_tail_visited().to_string()
}

pub fn second() -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        assert_eq!(
            dbg!(first(INPUT)),
            advent_of_code::solve(2022, 9, 1, INPUT).unwrap()
        );
        assert_eq!(
            dbg!(second()),
            advent_of_code::solve(2022, 9, 2, INPUT).unwrap()
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
    }
}
