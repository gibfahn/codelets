fn main() {
    println!("Hello, world!");
}

pub enum Problem {
    First,
    Second,
}

/// Takes a list of memory banks, and tries to redistribute them as much as possible. Returns
/// the number of redistributions until it starts to cycle if Problem::First passed, or the
/// number of iterations since the loop if Problem::Second passed.
pub fn redist_count(s: &str, p: Problem) -> usize {
    let mut banks: Vec<usize> = s.split_whitespace().map(|word| word.parse().unwrap()).collect();
    let bank_len = banks.len();
    let mut bank_history: Vec<Vec<usize>> = Vec::with_capacity(100);
    while ! bank_history.iter().any(|bank| bank.iter().zip(banks.iter()).all(|(a,b)| a == b)) {
        bank_history.push(banks.clone());
        let mut counter = banks.iter()
            .enumerate().max_by_key(|&(a, b)| (b, bank_len - a)).unwrap().0;
        let mut max = banks[counter];
        banks[counter] = 0;
        while max > 0 {
            counter = (counter + 1) % bank_len;
            banks[counter] += 1;
            max -= 1;
        }
    }

    match p {
        Problem::First => bank_history.len(),
        Problem::Second => {
            // TODO(gib): We could avoid this calculation by storing the result from the while
            // loop test.
            let (i, _) = bank_history.iter().enumerate()
                .filter(|&(_, bank)| bank.iter().zip(banks.iter()).all(|(a,b)| a == b)).nth(0).unwrap();
            bank_history.len() - i
        }
    }
}

#[test]
fn example_1() {
    let input = "0 2 7 0";
    assert_eq!(redist_count(input, Problem::First), 5);
}

#[test]
fn problem_1() {
    let input = include_str!("../input.txt");
    assert_eq!(redist_count(input, Problem::First), 7864);
}

#[test]
fn example_2() {
    let input = "0 2 7 0";
    assert_eq!(redist_count(input, Problem::Second), 4);
}

#[test]
fn problem_2() {
    let input = include_str!("../input.txt");
    assert_eq!(redist_count(input, Problem::Second), 1695);
}
