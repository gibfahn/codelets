use std::collections::HashMap;
use std::collections::HashSet;

pub struct Village {
    pipes: HashMap<usize, Vec<usize>>,
}

impl Village {
    pub fn from(pipe_list: &str) -> Self {
        let mut pipes: HashMap<usize, Vec<usize>> = HashMap::new();
        for line in pipe_list.trim().lines() {
            let mut words = line.split_whitespace();
            pipes.insert(words.next().unwrap().parse::<usize>().unwrap(),
                words.skip(1).map(|word| word.chars().filter(|&c| c != ',').collect::<String>().parse::<usize>().unwrap()).collect::<Vec<usize>>()
            );
        }
        Village { pipes }
    }

    pub fn take_group(&mut self, start: usize) -> HashSet<usize> {
        let mut group: HashSet<usize> = HashSet::new();
        let mut to_check: HashSet<usize> = HashSet::new();
        to_check.insert(start);
        while ! to_check.is_empty() {
            let pipe = *to_check.iter().nth(0).unwrap();
            if let Some(new_to_check) = self.pipes.get(&pipe).cloned() {
                group.insert(pipe);
                for p in new_to_check {
                    to_check.insert(p);
                }
                self.pipes.remove(&pipe);
            }
            to_check.remove(&pipe);
        }
        group
    }
}

pub fn group_size(s: &str) -> usize {
    let mut village = Village::from(s);
    village.take_group(0).len()
}

pub fn count_groups(s: &str) -> usize {
    let mut village = Village::from(s);
    let mut group_count = 0;
    while ! village.pipes.is_empty() {
        let first_pipe = *village.pipes.keys().nth(0).unwrap();
        village.take_group(first_pipe);
        group_count += 1;
    }
    group_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = 
"
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
";
        assert_eq!(group_size(input), 6);
        assert_eq!(count_groups(input), 2);
    }

    #[test]
    fn problem_1() {
        assert_eq!(group_size(include_str!("../input.txt")), 128);
    }

    #[test]
    fn problem_2() {
        assert_eq!(count_groups(include_str!("../input.txt")), 209);
    }
}
