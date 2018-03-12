use std::error::Error;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("./input.txt").expect("Could not open input.txt");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Could not read input.txt");

    let mut floor = Floor::new(&input, 40_000);
    floor.fill();
    let answer = floor.count_safe();

    println!("The answer is {}", answer);
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    Safe,
    Trap,
    Unknown,
}

fn match_tile(c: char) -> Result<Tile, Box<Error>> {
    match c {
        '.' => Ok(Tile::Safe),
        '^' => Ok(Tile::Trap),
        other => Err(format!("Unknown Tile: {}", other).into()),
    }
}

fn calc_tile(left: Tile, right: Tile) -> Tile {
    use Tile::*;
    if left == Unknown || right == Unknown {
        Unknown
    } else if left == Safe && right == Trap || left == Trap && right == Safe {
        Trap
    } else {
        Safe
    }

}

#[derive(Debug)]
struct Floor {
    v: Vec<Vec<Tile>>,
}

impl Floor {
    fn new(s: &str, rows: usize) -> Self {
        //println!("\nString: {}", s);
        let lines = s.lines().map(|x| x.trim());
        let columns = s.lines()
            .map(|x| x.trim())
            .take(1)
            .collect::<String>()
            .len();
        //println!("rows: {}, columns: {}, s.len(): {}", rows, columns, s.len());
        let mut v = vec![vec![Tile::Unknown; columns]; rows];
        //println!("Floor:");
        //for i in 0..v.len() {
        //    println!("{:?}", v[i]);
        //}
        for (row, line) in lines.enumerate() {
            if line.len() != columns {
                panic!(format!("Line {} length is wrong, length: {}, columns: {}",
                               row,
                               line.len(),
                               columns));
            }
            for (col, c) in line.chars().enumerate() {
                v[row][col] = match_tile(c).expect("Bad tile");
                //println!("v[{}][{}]: {:?}", row, col, v[row][col]);
            }
        }
        //println!("Floor created: {:?}", v);
        Floor { v: v }
    }

    fn get(&self, row: usize, col: usize) -> Option<Tile> {
        self.v
            .get(row)
            .and_then(|r| r.get(col))
            .cloned()
    }

    fn fill(&mut self) {
        use Tile::*;
        for row in 0..self.v.len() {
            for col in 0..self.v[0].len() {
                if self.get(row, col) == Some(Unknown) {
                    if row == 0 {
                        panic!("First row must be filled");
                    }
                    let left = if col == 0 {
                        Safe
                    } else {
                        self.get(row - 1, col - 1).unwrap()
                    };
                    let right = if col == self.v[0].len() - 1 {
                        Safe
                    } else {
                        self.get(row - 1, col + 1).unwrap()
                    };
                    self.v[row][col] = calc_tile(left, right);
                }
            }
        }
        //println!("Floor created: {:?}", self.v);
    }

    fn count_safe(&self) -> usize {
        self.v.iter().fold(0,
                           |acc, row| acc + row.iter().filter(|x| **x == Tile::Safe).count())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn placeholder() {
        let input = "..^^.
                     .^^^^
                     ^^..^";
        let floor = Floor::new(input, 3);
        assert_eq!(floor.v.len(), 3);
        assert_eq!(floor.get(0, 4).unwrap(), Tile::Safe);
        assert_eq!(floor.get(2, 0).unwrap(), Tile::Trap);
    }
}
