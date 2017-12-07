fn main() {
    println!("Hello, world!");
}

/// Takes a list of memory banks, and tries to redistribute them as much as possible. Returns
/// the number of redistributions until it starts to cycle.
pub fn redist_count(s: &str) -> usize {
    let mut banks: Vec<usize> = s.split_whitespace().map(|word| word.parse().unwrap()).collect();
    let bank_len = banks.len();
    let mut bank_history: Vec<Vec<usize>> = Vec::new();
    let mut iterations = 0;
    while ! bank_history.iter().any(|bank| bank.iter().zip(banks.iter()).all(|(a,b)| a == b)) {
        bank_history.push(banks.clone());
        iterations += 1;
        // println!("Banks: {:?}", banks);
        // println!("Bank history: {:?}", bank_history);
        // TODO(gib): Fix unnecessary clone (wish we had NLL).
        let mut counter;
        let mut max;
        {
            let (counter2, max2) = banks.iter()
                .enumerate().max_by_key(|&(a, b)| (b, bank_len - a)).unwrap();
            max = *max2;
            counter = counter2;

        }
        banks[counter] = 0;
        while max > 0 {
            counter = (counter + 1) % bank_len;
            banks[counter] += 1;
            max -= 1;
        }
    }
    iterations
}

#[test]
fn example_1() {
    let input = "0 2 7 0";
    assert_eq!(redist_count(input), 5);
}

#[test]
fn problem_1() {
    let input = include_str!("../input.txt");
    assert_eq!(redist_count(input), 7864);
}
