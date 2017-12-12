use std::collections::HashMap;
use std::collections::HashSet;

pub fn group_size(s: &str) -> usize {
    let mut pipes: HashMap<usize, Vec<usize>> = HashMap::new();
    for line in s.trim().lines() {
        let mut words = line.split_whitespace();
        pipes.insert(words.next().unwrap().parse::<usize>().unwrap(),
            words.skip(1).map(|word| word.chars().filter(|&c| c != ',').collect::<String>().parse::<usize>().unwrap()).collect::<Vec<usize>>()
        );
    }
    println!("HashMap: {:#?}", pipes);
    let mut group: HashSet<usize> = HashSet::new();
    let mut to_check: HashSet<usize> = HashSet::new();
    to_check.insert(0);
    while ! to_check.is_empty() {
        let pipe = *to_check.iter().nth(0).unwrap();
        if let Some(new_to_check) = pipes.get(&pipe).cloned() {
            group.insert(pipe);
            for p in new_to_check {
                to_check.insert(p);
            }
            pipes.remove(&pipe);
        }
        to_check.remove(&pipe);
    }
    // println!("group: {:?}", group);
    // println!("len: {}", group.len());
    group.len()
}

pub fn count_groups(s: &str) -> usize {
    let mut pipes: HashMap<usize, Vec<usize>> = HashMap::new();
    for line in s.trim().lines() {
        let mut words = line.split_whitespace();
        pipes.insert(words.next().unwrap().parse::<usize>().unwrap(),
            words.skip(1).map(|word| word.chars().filter(|&c| c != ',').collect::<String>().parse::<usize>().unwrap()).collect::<Vec<usize>>()
        );
    }
    println!("HashMap: {:#?}", pipes);
    let mut group_count = 0;
    while ! pipes.is_empty() {
        let mut group: HashSet<usize> = HashSet::new();
        let mut to_check: HashSet<usize> = HashSet::new();
        to_check.insert(*pipes.keys().nth(0).unwrap());
        while ! to_check.is_empty() {
            let pipe = *to_check.iter().nth(0).unwrap();
            if let Some(new_to_check) = pipes.get(&pipe).cloned() {
                group.insert(pipe);
                for p in new_to_check {
                    to_check.insert(p);
                }
                pipes.remove(&pipe);
            }
            to_check.remove(&pipe);
        }
        group_count += 1;
    }
    // println!("group: {:?}", group);
    // println!("len: {}", group.len());
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
