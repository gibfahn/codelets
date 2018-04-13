#![feature(inclusive_range_syntax)]
use std::fs::File;
use std::io::prelude::*;
use std::cmp::min;
use std::usize::MAX;

const MAX_PATH: usize = MAX;

fn main() {
    let mut file = File::open("./input.txt").expect("Could not open input.txt");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Could not read input.txt");

    println!("Input: {}", input);
    let maze = Maze::new(input.trim().parse().expect("Input isn't a number"));
    println!("{}", maze.shortest_path(Point(1, 1), Point(31, 39)));
    println!("{}", maze.visitable_locations(Point(1, 1), 50));
}

pub struct Maze {
    num: usize,
    min_path: usize,
}

impl Maze {
    pub fn new(magic_number: usize) -> Self {
        Maze {
            num: magic_number,
            min_path: MAX_PATH,
        }
    }

    pub fn is_space(&self, x: usize, y: usize) -> bool {
        let sum: usize = x * x + 3 * x + 2 * x * y + y + y * y + self.num;
        // println!("x: {}, y: {}, sum: {}", x, y, sum);
        sum.count_ones() % 2 == 0
    }

    pub fn shortest_path(&self, from: Point, to: Point) -> usize {
        if from == to {
            return 0;
        };
        // println!("");
        let mut visited = vec![vec![MAX_PATH; self.num + 1]; self.num + 1];
        self.take_step(from, to, 0, &mut visited).expect("No path to destination")
    }

    fn take_step(&self,
                 location: Point,
                 destination: Point,
                 travelled: usize,
                 visited: &mut Vec<Vec<usize>>)
                 -> Option<usize> {
        // println!("location: {:?}, destination: {:?}, travelled: {}, [{},{}]", location,
        //          destination, travelled, visited.len(), visited[visited.len()-1].len());
        if location == destination {
            return Some(travelled);
        }
        let travelled = travelled + 1;
        visited[location.0][location.1] = travelled;

        if travelled > self.min_path {
            return None;
        }

        let mut min_val: usize = MAX_PATH;

        if self.is_space(location.0, location.1 + 1) && location.1 + 1 < self.num &&
           visited[location.0][location.1 + 1] > travelled + 1 {
            min_val = min(self.take_step(Point(location.0, location.1 + 1),
                                         destination,
                                         travelled,
                                         visited)
                              .unwrap_or(MAX_PATH),
                          min_val);
        }
        if self.is_space(location.0 + 1, location.1) && location.0 + 1 < self.num &&
           visited[location.0 + 1][location.1] > travelled + 1 {
            min_val = min(self.take_step(Point(location.0 + 1, location.1),
                                         destination,
                                         travelled,
                                         visited)
                              .unwrap_or(MAX_PATH),
                          min_val);
        }
        if location.1 != 0 && self.is_space(location.0, location.1 - 1) &&
           visited[location.0][location.1 - 1] > travelled + 1 {
            min_val = min(self.take_step(Point(location.0, location.1 - 1),
                                         destination,
                                         travelled,
                                         visited)
                              .unwrap_or(MAX_PATH),
                          min_val);
        }
        if location.0 != 0 && self.is_space(location.0 - 1, location.1) &&
           visited[location.0 - 1][location.1] > travelled + 1 {
            min_val = min(self.take_step(Point(location.0 - 1, location.1),
                                         destination,
                                         travelled,
                                         visited)
                              .unwrap_or(MAX_PATH),
                          min_val);
        }

        Some(min_val)
    }

    pub fn visitable_locations(&self, from: Point, steps: usize) -> usize {
        let mut output: usize = 0;

        for x in -(steps as i64)..=steps as i64 {
            if from.0 as i64 + x > 0 {
                for y in -(steps as i64)..=steps as i64 {
                    if from.1 as i64 + y > 0 {
                        let x = from.0 + x as usize;
                        let y = from.1 + y as usize;
                        if self.is_space(x, y) && self.shortest_path(from, Point(x, y)) <= steps {
                            output += 1;
                        }
                    }
                }
            }
        }
        output
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point(usize, usize);


#[test]
fn ten_by_ten_maze() {
    let maze = Maze::new(10);
    assert!(maze.is_space(1, 1));
    assert!(maze.is_space(0, 1));
    assert!(!maze.is_space(1, 0));
    assert!(maze.is_space(2, 2));
    assert!(!maze.is_space(5, 2));
    assert!(!maze.is_space(9, 6));
    assert!(!maze.is_space(0, 6));
    assert!(maze.is_space(7, 2));
}

#[test]
fn ten_by_ten_path() {
    let maze = Maze::new(10);
    assert_eq!(maze.shortest_path(Point(1, 1), Point(7, 4)), 11);
}
