use snafu::{OptionExt, ResultExt, Snafu};
use std::{
    cmp::{max, min},
    convert::TryInto,
    ops::AddAssign,
    str::FromStr,
    usize,
};

const INPUT: &str = include_str!("input");

#[derive(Debug, Snafu, PartialEq)]
pub enum Error {
    #[snafu(display("Invalid direction: {}.", direction))]
    InvalidDirection { direction: String },
    #[snafu(display("Missing direction: {}", s))]
    MissingDirection { s: String },
    #[snafu(display("Invalid Length {}: {}.", s, source))]
    InvalidLength {
        s: String,
        source: std::num::ParseIntError,
    },
    #[snafu(display("No lines intersect with each other."))]
    NoIntersections,

    #[snafu(display("Conversion failed: {}", source))]
    ConversionError { source: std::num::TryFromIntError },
}

pub fn answer() -> (String, String) {
    (
        first(INPUT).unwrap().to_string(),
        second(INPUT).unwrap().to_string(),
    )
}

fn first(input: &str) -> Result<usize, Error> {
    get_intersections(input)?
        .iter()
        .map(|i| i.point.manhattan_distance())
        .filter(|&n| n > 0)
        .min()
        .ok_or(Error::NoIntersections {})
}

fn second(input: &str) -> Result<usize, Error> {
    get_intersections(input)?
        .iter()
        .map(|i| {
            length_until(&i.wire_a, i.point).unwrap() + length_until(&i.wire_b, i.point).unwrap()
        })
        .filter(|&n| n > 0)
        .min()
        .ok_or(Error::NoIntersections {})
}

fn length_until(wire: &[Line], point: Point) -> Result<usize, Error> {
    let mut length = 0;
    for segment in wire.iter() {
        if segment.touches(point) {
            let delta: usize = match segment.direction.orientation() {
                Orientation::Horizontal => (point.x - segment.a.x).abs(),
                Orientation::Vertical => (point.y - segment.a.y).abs(),
            }
            .try_into()
            .context(ConversionError {})?;
            length += delta;
            break;
        } else {
            length += segment.length;
        }
    }
    Ok(length)
}

fn get_intersections(input: &str) -> Result<Vec<Intersection>, Error> {
    let wires = input
        .lines()
        .map(|line| {
            let mut position = Point::default();
            line.split_terminator(',')
                .map(|i| i.parse::<WireSegment>())
                .map(|segment_result| {
                    segment_result
                        .and_then(|segment| Ok(Line::from_segment(&segment, &mut position)))
                })
                .collect::<Result<Vec<_>, Error>>()
        })
        .collect::<Result<Vec<_>, Error>>()?;

    let mut intersections = Vec::new();

    for (i, wire) in wires.iter().enumerate() {
        for other_wire in wires.iter().skip(i + 1) {
            for line in wire {
                for other_line in other_wire {
                    if let Some(intersection_point) = line.intersection_point(other_line) {
                        intersections.push(Intersection {
                            point: intersection_point,
                            wire_a: wire.clone(),
                            wire_b: other_wire.clone(),
                        });
                    }
                }
            }
        }
    }
    Ok(intersections)
}

#[derive(Debug, PartialEq, Clone)]
struct Intersection {
    point: Point,
    wire_a: Vec<Line>,
    wire_b: Vec<Line>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq, Clone)]
struct WireSegment {
    direction: Direction,
    length: usize,
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Clone)]
struct Line {
    a: Point,
    b: Point,
    direction: Direction,
    length: usize,
}

impl Line {
    fn from_segment(segment: &WireSegment, position: &mut Point) -> Self {
        let old_position = *position;
        *position += segment.clone();
        Line {
            a: old_position,
            b: *position,
            direction: segment.direction,
            length: segment.length,
        }
    }

    fn touches(&self, point: Point) -> bool {
        match self.direction.orientation() {
            Orientation::Horizontal => {
                point.y == self.a.y
                    && point.x >= min(self.a.x, self.b.x)
                    && point.x <= max(self.a.x, self.b.x)
            }
            Orientation::Vertical => {
                point.x == self.a.x
                    && point.y >= min(self.a.y, self.b.y)
                    && point.y <= max(self.a.y, self.b.y)
            }
        }
    }

    fn intersection_point(&self, other: &Line) -> Option<Point> {
        match (self.direction.orientation(), other.direction.orientation()) {
            (Orientation::Horizontal, Orientation::Horizontal)
            | (Orientation::Vertical, Orientation::Vertical) => None,
            (Orientation::Horizontal, Orientation::Vertical) => {
                if self.a.y <= max(other.a.y, other.b.y)
                    && self.a.y >= min(other.a.y, other.b.y)
                    && other.a.x <= max(self.a.x, self.b.x)
                    && other.a.x >= min(self.a.x, self.b.x)
                {
                    Some(Point {
                        x: other.a.x,
                        y: self.a.y,
                    })
                } else {
                    None
                }
            }
            (Orientation::Vertical, Orientation::Horizontal) => {
                if other.a.y <= max(self.a.y, self.b.y)
                    && other.a.y >= min(self.a.y, self.b.y)
                    && self.a.x <= max(other.a.x, other.b.x)
                    && self.a.x >= min(other.a.x, other.b.x)
                {
                    Some(Point {
                        x: self.a.x,
                        y: other.a.y,
                    })
                } else {
                    None
                }
            }
        }
    }
}

impl Direction {
    fn orientation(self) -> Orientation {
        match self {
            Direction::Left | Direction::Right => Orientation::Horizontal,
            Direction::Up | Direction::Down => Orientation::Vertical,
        }
    }
}

impl Point {
    fn manhattan_distance(self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

impl AddAssign<WireSegment> for Point {
    fn add_assign(&mut self, segment: WireSegment) {
        match segment.direction {
            Direction::Left => self.x -= segment.length as i32,
            Direction::Right => self.x += segment.length as i32,
            Direction::Up => self.y += segment.length as i32,
            Direction::Down => self.y -= segment.length as i32,
        }
    }
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => InvalidDirection { direction: s }.fail(),
        }
    }
}

impl FromStr for WireSegment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let direction = chars
            .next()
            .context(MissingDirection { s })?
            .to_string()
            .parse::<Direction>()?;
        let length = chars
            .collect::<String>()
            .parse::<usize>()
            .context(InvalidLength {
                s: s.chars().skip(1).collect::<String>(),
            })?;
        Ok(WireSegment { direction, length })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_examples() {
        assert_eq!(first("R8,U5,L5,D3,\nU7,R6,D4,L4,"), Ok(6));
        assert_eq!(
            first("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            Ok(159)
        );
        assert_eq!(
            first(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            Ok(135)
        );
    }

    #[test]
    fn first_real() {
        assert_eq!(first(INPUT), Ok(865));
    }

    #[test]
    fn second_examples() {
        assert_eq!(second("R8,U5,L5,D3,\nU7,R6,D4,L4,"), Ok(30));
        assert_eq!(
            second("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            Ok(610)
        );
        assert_eq!(
            second(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            Ok(410)
        );
    }

    #[test]
    fn second_real() {
        assert_eq!(second(INPUT), Ok(35038));
    }
}
