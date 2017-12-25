use std::cmp::max;

#[derive(Default)]
pub struct Location { x: isize, y: isize, }

impl Location {
    pub fn walk(&mut self, step: &str) {
        match step {
            "n" => { self.x += 1; self.y += 1; },
            "s" => { self.x -= 1; self.y -= 1; },
            "ne" => self.y += 1,
            "sw" => self.y -= 1,
            "nw" => self.x += 1,
            "se" => self.x -= 1,
            _ => panic!("How did we get here?"),
        }
    }

    fn distance(&mut self) -> usize {
        max((self.x - self.y).abs(), max(self.x.abs(), self.y.abs())) as usize
    }
}

pub fn total_distance(s: &str) -> usize {
    let mut location = Location::default();
    for step in s.trim().split(',') {
        location.walk(step);
    }
    location.distance()
}

pub fn max_distance(s: &str) -> usize {
    let mut location = Location::default();
    s.trim().split(',').fold(0, |max_d, step| {
        location.walk(step);
        max(max_d, location.distance())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1_1() {
        assert_eq!(total_distance("ne,ne,ne"), 3);
        assert_eq!(total_distance("ne,ne,sw,sw"), 0);
        assert_eq!(total_distance("ne,ne,s,s"), 2);
        assert_eq!(total_distance("se,sw,se,sw,sw"), 3);
    }

    #[test]
    fn problem_1() {
        assert_eq!(total_distance(include_str!("../input")), 650);
    }

    #[test]
    fn problem_2() {
        assert_eq!(max_distance(include_str!("../input")), 1465);
    }
}
