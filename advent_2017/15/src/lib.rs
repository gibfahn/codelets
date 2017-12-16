pub fn count_matches_1(mut a: usize, mut b: usize, iterations: usize) -> usize {
    (0..iterations).filter(|_| {
        a = a * 16_807 % 2_147_483_647;
        b = b * 48_271 % 2_147_483_647;
        a & 0xFFFF == b & 0xFFFF
    }).count()
}

pub fn count_matches_2(mut a: usize, mut b: usize, iterations: usize) -> usize {
    let mut matched_a = Vec::new();
    let mut matched_b = Vec::new();

    while std::cmp::min(matched_a.len(), matched_b.len()) < iterations {
        a = a * 16_807 % 2_147_483_647;
        b = b * 48_271 % 2_147_483_647;
        if a % 4 == 0 { matched_a.push(a); }
        if b % 8 == 0 { matched_b.push(b); }
    }

    matched_a.iter().zip(matched_b).filter(|&(a, b)| a & 0xFFFF == b & 0xFFFF).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(count_matches_1(65, 8921, 5), 1);
        assert_eq!(count_matches_1(65, 8921, 40_000_000), 588);
        assert_eq!(count_matches_2(65, 8921, 5_000_000), 309);
    }

    #[test]
    fn problem_1() {
        assert_eq!(count_matches_1(289, 629, 40_000_000), 638);
    }

    #[test]
    fn problem_2() {
        assert_eq!(count_matches_2(289, 629, 5_000_000), 343);
    }
}
