#![feature(external_doc)]
#![doc(include = "../Question.md")]

use std::collections::VecDeque;
use std::mem;

const INPUT: &str = include_str!("../input");

pub fn answer() -> (String, String) {
    let license_tree = LicenseTree::from(INPUT);
    (
        license_tree.metadata_sum().to_string(),
        license_tree.root_value().to_string(),
    )
}

#[derive(Debug, Clone)]
struct LicenseTree {
    root: LicenseNode,
}

#[derive(Debug, Clone, Default)]
struct LicenseNode {
    child_count: u32,
    metadata_count: u32,
    children: VecDeque<LicenseNode>,
    metadata: VecDeque<u32>,
}

impl LicenseTree {
    fn from(s: &str) -> Self {
        let mut input: VecDeque<u32> = s
            .split_whitespace()
            .map(|w| w.parse::<u32>().unwrap())
            .collect();

        LicenseTree {
            root: LicenseNode::from(&mut input),
        }
    }

    fn metadata_sum(&self) -> u32 {
        self.root.metadata_sum()
    }

    fn root_value(&self) -> u32 {
        self.root.value()
    }
}

/// 2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2 <- All
/// 2 3                           1 1 2 <- A
///     0 3 10 11 12 1 1 0 1 99 2       <- A Children
///     0 3 10 11 12                    <- B
///                  1 1        2       <- C
///                      0 1 99         <- C children
///                      0 1 99         <- D
impl LicenseNode {
    fn from(input: &mut VecDeque<u32>) -> Self {
        let child_count = input.pop_front().unwrap();
        let metadata_count = input.pop_front().unwrap();

        let mut children = VecDeque::new();

        let mut children_left = child_count;
        while children_left > 0 {
            children.push_back(Self::from(input));
            children_left -= 1;
        }

        let remaining = input.split_off(metadata_count as usize);
        let metadata = mem::replace(input, remaining);

        LicenseNode {
            child_count,
            metadata_count,
            metadata,
            children,
        }
    }

    fn metadata_sum(&self) -> u32 {
        self.metadata.iter().sum::<u32>()
            + self.children.iter().map(Self::metadata_sum).sum::<u32>()
    }

    fn value(&self) -> u32 {
        assert_eq!(self.child_count as usize, self.children.len());
        if self.child_count == 0 {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                .map(|&n| {
                    if n == 0 {
                        0
                    } else if let Some(child) = self.children.get(n as usize - 1) {
                        child.value()
                    } else {
                        0
                    }
                })
                .sum()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let license_tree = LicenseTree::from(input);
        assert_eq!(license_tree.metadata_sum(), 138);
        assert_eq!(license_tree.root_value(), 66);
    }

    #[test]
    fn test_answer() {
        assert_eq!(answer(), (String::from("43996"), String::from("35189")));
    }
}
