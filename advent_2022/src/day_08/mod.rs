const INPUT: &str = include_str!("./input");

#[derive(Debug)]
struct Trees(Vec<Vec<u32>>);

impl Trees {
    fn from_str(s: &str) -> Self {
        Self(
            s.lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        )
    }

    fn count_visible(&self) -> u32 {
        let mut count = 0;
        for (x, row) in self.0.iter().enumerate() {
            for (y, height) in row.iter().enumerate() {
                if self.visible(x, y, *height, Direction::Up)
                    || self.visible(x, y, *height, Direction::Left)
                    || self.visible(x, y, *height, Direction::Down)
                    || self.visible(x, y, *height, Direction::Right)
                {
                    count += 1;
                }
            }
        }
        count
    }

    /// This function counts how many trees are visible from this tree position.
    fn visible(&self, mut x: usize, mut y: usize, height: u32, direction: Direction) -> bool {
        loop {
            (x, y) = match direction {
                Direction::Up if y == 0 => return true,
                Direction::Up => (x, y - 1),
                Direction::Left if x == 0 => return true,
                Direction::Left => (x - 1, y),
                Direction::Down => (x, y + 1),
                Direction::Right => (x + 1, y),
            };
            let Some(new_height) = self.0.get(x).and_then(|row| row.get(y)) else {
                return true;
            };
            if new_height >= &height {
                return false;
            }
        }
    }

    fn count_scenic(&self, mut x: usize, mut y: usize, height: u32, direction: Direction) -> u32 {
        let mut count = 0;
        loop {
            (x, y) = match direction {
                Direction::Up if y == 0 => break,
                Direction::Up => (x, y - 1),
                Direction::Left if x == 0 => break,
                Direction::Left => (x - 1, y),
                Direction::Down => (x, y + 1),
                Direction::Right => (x + 1, y),
            };
            let Some(new_height) = self.0.get(x).and_then(|row| row.get(y)) else {
                break;
            };
            count += 1;
            if new_height >= &height {
                break;
            }
        }
        count
    }

    fn max_scenic(&self) -> u32 {
        let mut most_scenic = 0;
        for (x, row) in self.0.iter().enumerate() {
            for (y, height) in row.iter().enumerate() {
                let scenic = self.count_scenic(x, y, *height, Direction::Up)
                    * self.count_scenic(x, y, *height, Direction::Left)
                    * self.count_scenic(x, y, *height, Direction::Down)
                    * self.count_scenic(x, y, *height, Direction::Right);
                if scenic > most_scenic {
                    most_scenic = scenic;
                }
            }
        }
        most_scenic
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

pub fn first(input: &str) -> String {
    let trees = Trees::from_str(input);
    trees.count_visible().to_string()
}

pub fn second(input: &str) -> String {
    let trees = Trees::from_str(input);
    trees.max_scenic().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        assert_eq!(
            dbg!(first(INPUT)),
            advent_of_code::solve(2022, 8, 1, INPUT).unwrap()
        );
        assert_eq!(
            dbg!(second(INPUT)),
            advent_of_code::solve(2022, 8, 2, INPUT).unwrap()
        );
    }

    #[test]
    fn examples() {
        let input = "30373
25512
65332
33549
35390";
        assert_eq!(dbg!(first(input)), "21");
    }
}
