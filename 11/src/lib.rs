#[derive(Default)]
pub struct Point {
    x: isize,
    y: isize
}

impl Point {
    pub fn take_step(&mut self, step: &str) {
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
        if self.x > 0 && self.y > 0 || self.x < 0 && self.y < 0 {
            std::cmp::max(isize::abs(self.x),isize::abs(self.y)) as usize
        } else {
            isize::abs(self.x - self.y) as usize
        }
    }
}

pub fn total_distance(s: &str) -> usize {
    let mut location = Point::default();
    for step in s.trim().split(',') {
        location.take_step(step);
    }
    location.distance()
}

pub fn max_distance(s: &str) -> usize {
    let mut max_distance = 0;
    let mut location = Point::default();
    for step in s.trim().split(',') {
        location.take_step(step);
        max_distance = std::cmp::max(max_distance, location.distance());
    }
    max_distance
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1_1() {
        assert_eq!(total_distance("ne,ne,ne"), 3);
        assert_eq!(total_distance("ne,ne,sw,sw"), 0);
        assert_eq!(total_distance("ne,ne,s,s"), 2);
    }

    #[test]
    fn example_1_2() {
        assert_eq!(total_distance("se,sw,se,sw,sw"), 3);
    }

    #[test]
    fn problem_1() {
        assert_eq!(total_distance(include_str!("../input.txt")), 650);
    }

    #[test]
    fn problem_2() {
        assert_eq!(max_distance(include_str!("../input.txt")), 1465);
    }
}
