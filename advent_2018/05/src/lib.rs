#![feature(external_doc)]
#![doc(include = "../Question.md")]
#![feature(const_fn)]
#![feature(test)]

extern crate test;

use rayon::prelude::*;

const INPUT: &str = include_str!("../input");

pub fn answer() -> (String, String) {
    let polymer = Polymer::from(INPUT);
    (
        polymer.clone().react().to_string(),
        polymer.remove_react().to_string(),
    )
}

#[derive(Debug, PartialEq, Clone)]
struct Polymer {
    units: Vec<char>,
}

impl Polymer {
    fn from(s: &str) -> Self {
        Polymer {
            units: s.trim().chars().collect(),
        }
    }

    fn remove_react(&self) -> usize {
        const ALPHABET: [(char, char); 26] = [
            ('A', 'a'),
            ('B', 'b'),
            ('C', 'c'),
            ('D', 'd'),
            ('E', 'e'),
            ('F', 'f'),
            ('G', 'g'),
            ('H', 'h'),
            ('I', 'i'),
            ('J', 'j'),
            ('K', 'k'),
            ('L', 'l'),
            ('M', 'm'),
            ('N', 'n'),
            ('O', 'o'),
            ('P', 'p'),
            ('Q', 'q'),
            ('R', 'r'),
            ('S', 's'),
            ('T', 't'),
            ('U', 'u'),
            ('V', 'v'),
            ('W', 'w'),
            ('X', 'x'),
            ('Y', 'y'),
            ('Z', 'z'),
        ];

        ALPHABET
            .par_iter()
            .map(|(lower, upper)| {
                let mut polymer = self.clone();
                polymer.units.retain(|&c| c != *lower && c != *upper);
                polymer.react()
            })
            .min()
            .unwrap()
    }

    fn react(mut self) -> usize {
        let mut i = 0;
        while i < self.units.len() - 1 {
            if self.units[i] != self.units[i + 1]
                && self.units[i].to_lowercase().to_string()
                    == self.units[i + 1].to_lowercase().to_string()
            {
                self.units.remove(i + 1);
                self.units.remove(i);
                i = i.saturating_sub(1);
            } else {
                i += 1;
            }
        }
        self.units.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn examples() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!(Polymer::from(input).react(), 10);
        assert_eq!(Polymer::from(input).remove_react(), 4);
    }

    #[test]
    fn test_answer() {
        assert_eq!(answer(), (String::from("10598"), String::from("5312")));
    }

    #[bench]
    fn bench_first_example(b: &mut Bencher) {
        let input = "dabAcCaCBAcCcaDA";
        b.iter(|| {
            Polymer::from(input).react()
        })
    }

    #[bench]
    fn bench_second_example(b: &mut Bencher) {
        let input = "dabAcCaCBAcCcaDA";
        b.iter(|| {
            Polymer::from(input).remove_react()
        })
    }

    #[bench]
    fn bench_first_answer(b: &mut Bencher) {
        b.iter(|| {
            Polymer::from(INPUT).react()
        })
    }

    #[bench]
    fn bench_second_answer(b: &mut Bencher) {
        b.iter(|| {
            Polymer::from(INPUT).remove_react()
        })
    }

}
