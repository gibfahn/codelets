use std::collections::{HashSet, HashMap};

fn main() {
    println!("Hello, world!");
}

/// Get the root of a tree of nodes, given a list of nodes and their children.
fn get_root(s: &str) -> String {
    let bottoms: Vec<Vec<String>> = s.lines()
        .filter_map(|line| match line.split_whitespace().nth(2) {
            Some(_) => Some(line.split_whitespace()
                .map(|word| word.chars().filter(|&c| c != ',').collect())
                .collect()),
            None => None,
            }).collect();
    let parents: HashSet<String> = bottoms.iter().map(|row| row[0].clone()).collect();
    let children: HashSet<String> = bottoms.iter()
        .flat_map(|row| row.clone().into_iter().skip(3)).collect();
    let root: Vec<_> = parents.difference(&children).collect();
    if root.len() != 1 { panic!("There can only be one root!"); }
    root[0].clone()
}

fn get_right_weight(s: &str) -> usize {
    let root = get_root(s);
    let bottoms: Vec<Vec<String>> = s.lines()
        .filter_map(|line| match line.split_whitespace().nth(2) {
            Some(_) => Some(line.split_whitespace()
                .map(|word| word.chars().filter(|&c| c != ',').collect())
                .collect()),
            None => None,
            }).collect();
    let parents: HashSet<String> = bottoms.iter().map(|row| row[0].clone()).collect();
    let mut children: HashSet<String> = bottoms.iter()
        .flat_map(|row| row.clone().into_iter().skip(3)).collect();
    let individual_weights: HashMap<String, usize> = s.lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            (
                words.next().unwrap().to_owned(),
                words.next().unwrap().chars().filter(|&c| c.is_digit(10))
                    .collect::<String>().parse::<usize>().unwrap()
            )})
        .collect();
    let total_weights: HashMap<String, usize> = children.difference(&parents)
        .map(|leaf| (leaf.clone(), *individual_weights.get(leaf).unwrap())).collect();
    for (name, _) in total_weights.iter() {
        children.remove(name);
    }
    println!("bottoms: {:?}", bottoms);
    60
}


    // ugml + (gyxo + ebii + jptl) = 68 + (61 + 61 + 61) = 251
    // padx + (pbga + havc + qoyq) = 45 + (66 + 66 + 66) = 243
    // fwft + (ktlj + cntj + xhth) = 72 + (57 + 57 + 57) = 243

#[test]
fn example_1() {
    let input = "
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)".trim();
    assert_eq!(get_root(input), String::from("tknk"));
    assert_eq!(get_right_weight(input), 60);
}

#[test]
fn problem_1() {
    let input = include_str!("../input.txt");
    assert_eq!(get_root(input), String::from("hlhomy"));
}
