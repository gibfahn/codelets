use std::collections::HashMap;
use std::collections::HashSet;

pub struct Village {
    pipes: HashMap<usize, Vec<usize>>,
}

impl Village {
    pub fn from(pipe_list: &str) -> Self {
        Village {
            pipes: pipe_list.trim().lines().map(|line| {
                let mut words = line.split_whitespace().map(|word| word.chars().filter(|&c| c != ',').collect::<String>()
                                    .parse::<usize>().unwrap_or(0));
                (words.next().unwrap(), words.skip(1).collect::<Vec<usize>>())
            }).collect::<HashMap<usize, Vec<usize>>>(),
        }
    }

    pub fn take_group(&mut self, start: usize) -> HashSet<usize> {
        let mut group: HashSet<usize> = HashSet::new();
        let mut to_check: HashSet<usize> = HashSet::new();
        to_check.insert(start);
        while ! to_check.is_empty() {
            let pipe = *to_check.iter().nth(0).unwrap();
            if let Some(new_to_check) = self.pipes.get(&pipe).cloned() {
                group.insert(pipe);
                for p in new_to_check { to_check.insert(p); }
                self.pipes.remove(&pipe);
            }
            to_check.remove(&pipe);
        }
        group
    }

    pub fn first_group(&mut self) -> usize {
        self.take_group(0).len()
    }

    pub fn count_groups(&mut self) -> usize {
        let mut group_count = 0;
        while ! self.pipes.is_empty() {
            let first_pipe = *self.pipes.keys().nth(0).unwrap();
            self.take_group(first_pipe);
            group_count += 1;
        }
        group_count
    }
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
        assert_eq!(Village::from(input).first_group(), 6);
        assert_eq!(Village::from(input).count_groups(), 2);
    }

    #[test]
    fn problem_1() {
        assert_eq!(Village::from(include_str!("../input.txt")).first_group(), 128);
    }

    #[test]
    fn problem_2() {
        assert_eq!(Village::from(include_str!("../input.txt")).count_groups(), 209);
    }
}
