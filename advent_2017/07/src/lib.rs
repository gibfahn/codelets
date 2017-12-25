use std::collections::{HashSet, HashMap};

/// Get the root of a tree of nodes, given a list of nodes and their children.
pub fn get_root(s: &str) -> String {
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
    assert!(root.len() == 1);
    root[0].clone()
}

/// Given an unbalanced tree of nodes, where changing the weight of one node will balance the tree,
/// work out what the weight of that node should be.
pub fn get_right_weight(s: &str) -> usize {
    let individual_weights: HashMap<String, usize> = s.lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            (
                words.next().unwrap().to_owned(),
                words.next().unwrap().chars().filter(|&c| c.is_digit(10))
                    .collect::<String>().parse::<usize>().unwrap()
            )})
        .collect();
    let mut families: HashMap<String, Vec<String>> = s.lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            (
                words.next().unwrap().to_owned(),
                words.skip(2).map(|word| word.chars().filter(|&c| c != ',').collect::<String>()).collect(),
            )})
        .collect();
    let mut total_weights: HashMap<String, usize> =
        individual_weights.keys().map(|key| key.to_owned()).collect::<HashSet<_>>()
            .difference(
                &families.iter().filter(|&(_, children)| ! children.is_empty()) .map(|(a, _)| a.to_owned()).collect()
                        )
            .map(|leaf| (leaf.clone(), individual_weights[leaf])).collect();
    for name in total_weights.keys() { families.remove(name); }
    loop {
        let to_check: Vec<(String, Vec<String>)> = families.iter()
            .filter(|&(_, children)| children.iter().all(|child| total_weights.contains_key(child)))
            .map(|(parent, children)| (parent.to_owned(), children.clone()))
            .collect();
        for &(ref parent, ref children) in &to_check {
            let expected_weight =
                if total_weights.get(&children[0]) != total_weights.get(&children[1]) &&
                    total_weights.get(&children[1]) == total_weights.get(&children[2])
                {
                    total_weights[&children[1]]
                } else {
                    total_weights[&children[0]]
                };
            if let Some(unbalanced) = children.iter()
                .find(|&child| total_weights[child] != expected_weight)
            {
                assert!(children.len() > 2);
                return individual_weights[unbalanced] - (total_weights[unbalanced] - expected_weight);
            } else {
                total_weights.insert(parent.clone(),
                                     individual_weights[parent] + children.len() * expected_weight);
                families.remove(parent);
            }
        }
    }
}

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

#[test] fn problem_1() {
    let input = include_str!("../input");
    assert_eq!(get_root(input), String::from("hlhomy"));
}

#[test]
fn problem_2() {
    let input = include_str!("../input");
    assert_eq!(get_right_weight(input), 1505);
}
