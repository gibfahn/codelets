extern crate ten; // See ../10/ .
extern crate bit_vec;

use ten::KnotHash;
use bit_vec::BitVec;

pub fn defrag(s: &str) -> u32 {
    (0..128).fold(0, |count, n| {
        KnotHash::from(&format!("{}-{}", s, n), 255).knot_hash_vec().iter()
            .fold(0, |c, &h| c + h.count_ones()) + count
    })
}

pub fn regions(s: &str) -> u32 {
    let mut grid = (0..128).map(|n| {
        BitVec::from_bytes(&KnotHash::from(&format!("{}-{}", s, n), 255).knot_hash_vec())
    }).collect::<Vec<_>>();
    let mut regions = 0;
    let mut to_check = Vec::new();
    let mut y = 0;
    while y < 128 {
        let mut x = 0;
        while x < 128 {
            if grid[y][x] {
                regions += 1;
                to_check.push((y,x));
                while let Some((y,x)) = to_check.pop() {
                    grid[y].set(x, false);
                    if y > 0   && grid[y-1][x] { to_check.push((y-1, x)); }
                    if y < 127 && grid[y+1][x] { to_check.push((y+1, x)); }
                    if x > 0   && grid[y][x-1] { to_check.push((y, x-1)); }
                    if x < 127 && grid[y][x+1] { to_check.push((y, x+1)); }
                }
            }
            x += 1;
        }
        y += 1;
    }
    regions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(defrag("flqrgnkx"), 8108);
        assert_eq!(defrag("ljoxqyyw"), 8316);
    }

    #[test]
    fn problem_1() {
        assert_eq!(defrag("amgozmfv"), 8222);
    }

    #[test]
    fn example_2() {
        assert_eq!(regions("flqrgnkx"), 1242);
        assert_eq!(defrag("ljoxqyyw"), 8316);
    }

    #[test]
    fn problem_2() {
        assert_eq!(regions("amgozmfv"), 1086);
    }
}
