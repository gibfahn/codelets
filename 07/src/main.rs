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
    let mut total_weights: HashMap<String, usize> = children.difference(&parents)
        .map(|leaf| (leaf.clone(), *individual_weights.get(leaf).unwrap())).collect();
    let mut families: HashMap<String, Vec<String>> = s.lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            (
                words.next().unwrap().to_owned(),
                words.skip(2).map(|word| word.chars().filter(|&c| c != ',').collect::<String>()
                                  ).collect(),
            )})
        .collect();
    for (name, _) in total_weights.iter() {
        children.remove(name);
        families.remove(name);
    }
    'outer: loop {
        // println!("bottoms: {:?}\n", bottoms);
        // println!("families: {:?}\n", families);
        // println!("total weights: {:?}\n", total_weights);
        // println!("individual_weights: {:?}\n", individual_weights);
        // println!("parents: {:?}\n", parents);
        // println!("children: {:?}\n", children);
        let to_check: Vec<(String, Vec<String>)> = families.iter()
            // .inspect(|&(parent, children)| {
            //     println!("CHILDREN: {:?} ({})", children, parent);
            //     println!("total weights: {:?}\n", total_weights);
            //     for child in children {
            //         println!("{}: {}", child, total_weights.contains_key(child));
            //     }
            // })
            .filter(|&(parent, children)| children.iter().all(|child| total_weights.contains_key(child)))
            .map(|(parent, children)| (parent.to_owned(), children.clone()))
            .collect();
        // println!("to_check: {:#?}\n", to_check);
        for &(ref parent, ref children) in to_check.iter() {
            let mut expected_weight = *total_weights.get(&children[0]).unwrap();
            // println!("Weights: {:?}, {:?}, {:?}", total_weights.get(&children[0]), total_weights.get(&children[1]), total_weights.get(&children[2]));
            // println!("A: {:?}, B: {:?}", total_weights.get(&children[0]) != total_weights.get(&children[1]), total_weights.get(&children[1]) == total_weights.get(&children[2]));
            if total_weights.get(&children[0]) != total_weights.get(&children[1]) && 
                total_weights.get(&children[1]) == total_weights.get(&children[2]) {
                    expected_weight = *total_weights.get(&children[1]).unwrap();
                }
            // println!("expected_weight: {:?}\n", expected_weight);

            if let Some(unbalanced) = children.iter().find(|&child| *total_weights.get(child).unwrap() != expected_weight) {
                assert!(children.len() > 2);
                return *individual_weights.get(unbalanced).unwrap() - (*total_weights.get(unbalanced).unwrap() - expected_weight);
            } else {
                total_weights.insert(parent.clone(), individual_weights.get(parent).unwrap() + children.len() * expected_weight);
                families.remove(parent);
            }
        }
    }
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

#[test]
fn problem_2() {
    let input = include_str!("../input.txt");
    assert_eq!(get_right_weight(input), 1505);
}

