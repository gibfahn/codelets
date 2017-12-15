pub fn count_matches(seed_a: usize, seed_b: usize, iterations: usize) -> usize {
    let mut count = 0;
    let mut val_a = seed_a;
    let mut val_b = seed_b;
    for _ in 0..iterations {
        val_a = (val_a * 16807) % 2147483647;
        val_b = (val_b * 48271) % 2147483647;
        if val_a & 0xFFFF == val_b & 0xFFFF {
            // println!("a: {}, b: {}", val_a, val_b);
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(count_matches(65, 8921, 5), 1);
        assert_eq!(count_matches(65, 8921, 40_000_000), 588);
    }

    #[test]
    fn problem_1() {
        assert_eq!(count_matches(289, 629, 40_000_000), 638);
    }
}
