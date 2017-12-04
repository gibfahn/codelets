//! Advent of Code 2017 - Day 3
//! http://adventofcode.com/2017/day/3

#![feature(iterator_step_by)]

fn main() {
    let input = 361527;
    println!("First answer: {}", distance(input));
    println!("Second answer: {}", get_first_value_larger_than(361527));
}

/// Calls get_next_val() until the value is larger than the provided threshold, at which point it
/// returns that value.
pub fn get_first_value_larger_than(threshold: usize) -> usize {
    let mut spiral = Vec::with_capacity(100);
    let mut next_ring = 1;
    let mut next_ring_start = 1;
    spiral.push(1);
    let mut n = 1;
    while spiral[n-1] < threshold {
        n += 1;
        // println!("\n{:?}, n: {}", spiral, n);
        let next_val = get_next_val(&spiral, n, &mut next_ring, &mut next_ring_start);
        spiral.push(next_val);
    }
    // println!("\n{:?}", spiral);
    spiral[n-1]
}

/// Calls get_next_val() n times and returns the resulting Vector.
pub fn get_n_values(max: usize) -> Vec<usize> {
    let mut spiral = Vec::with_capacity(max);
    let mut next_ring = 1;
    let mut next_ring_start = 1;
    spiral.push(1);
    let mut n = 1;
    while n < max {
        n += 1;
        // println!("\n{:?}, n: {}", spiral, n);
        let next_val = get_next_val(&spiral, n, &mut next_ring, &mut next_ring_start);
        spiral.push(next_val);
    }
    // println!("\n{:?}", spiral);
    spiral
}

/// Helper function for the second problem.
/// Given a vector representing a spiral where each result is the sum of the already populated
/// values adjacent to it, for example:
///
/// ```
/// 147  142  133  122   59
/// 304    5    4    2   57
/// 330   10    1    1   54
/// 351   11   23   25   26
/// 362  747  806  880  931
///
/// Which corresponds to:
/// [1, 1, 2, 4, 5, 10, 11, 23, 25, 26, 54, 57, 59, 122, 133, 142, 147, 304, 330, 351, 362, 747, 806, 880, 931];
///
/// Work out the next number in the sequence.
pub fn get_next_val(spiral: &Vec<usize>, n: usize, next_ring: &mut usize, next_ring_start: &mut usize) -> usize {
    let mut next_val = spiral[n-2]; // Next val always contains the previous value.
    if n > *next_ring_start { // First side
        if n > 9 {
            let inside_corner = (*next_ring as usize - 2).pow(2) + 1;
            next_val += spiral[inside_corner - 1];
        }
        *next_ring += 2;
        *next_ring_start = (*next_ring).pow(2);
        return next_val;
    }
    let ring = *next_ring - 2;
    let ring_start = ring.pow(2);
    let side = ((ring + 2).pow(2) - ring_start) / 4; // Side length.
    let quadrant = (n - ring_start) / side;
    let side_middle = ring_start + quadrant * side;
    let offset = n - side_middle;
    if offset == 0 { // Other corner
        let inside_corner = if n <= 9 { 1 } else {
            let last_side = side - 2;
            let prev_ring_start = (ring - 2).pow(2);
            prev_ring_start + quadrant * last_side
        };
        next_val += spiral[inside_corner - 1];
        if n == *next_ring_start {
            let inside_side = ring_start + 1;
            next_val += spiral[inside_side - 1];
        }
        return next_val;
    } else { // Side
        let inside_side = if n <= 9 { 1 } else {
            let last_side = side - 2;
            let prev_ring_start = (ring - 2).pow(2);
            prev_ring_start + quadrant * last_side + offset - 1
        };
        next_val += spiral[inside_side - 1];
        let inside_side_prev = if offset == 1 || n == ring_start + 2 {
            n - 2
        } else { inside_side - 1 };
        next_val += spiral[inside_side_prev - 1];
        if offset != side - 1 {
            let inside_side_next = inside_side + 1;
            next_val += spiral[inside_side_next - 1];
        }
        if n == *next_ring_start - 1 {
            let inside_side_next = ring_start + 1;
            next_val += spiral[inside_side_next - 1];
        }
        return next_val;
    }
}


/// Solution to the first problem. Given a spiral grid populated like this:
///
/// ```
/// 65  64  63  62  61  60  59  58  57
/// 66  37  36  35  34  33  32  31  56
/// 67  38  17  16  15  14  13  30  55
/// 68  39  18   5   4   3  12  29  54
/// 69  40  19   6   1   2  11  28  53
/// 70  41  20   7   8   9  10  27  52
/// 71  42  21  22  23  24  25  26  51
/// 72  43  44  45  46  47  48  49  50
/// 73  74  75  76  77  78  79  80  81
/// ```
///
/// Work out the number of moves from tile n to the centre (tile 1).
pub fn distance(n: i64) -> i64 {
    if n == 1 { return 0; }
    let mut ring = 0; // Bottom right corner is square of an odd number. Last one before n.
    {
        let mut i = 1;
        while i * i < n {
            ring = i;
            i += 2;
        }
    }
    let ring_start = ring.pow(2);
    let min_distance = (ring + 1) / 2; // Distance from this ring if there's no offset.

    let side = ((ring + 2).pow(2) - ring_start) / 4; // Side length.
    let quadrant = (n - ring_start) / side;
    let side_middle = ring_start + quadrant * side + side / 2;
    let offset = (n - side_middle).abs();
    offset + min_distance
}

#[test]
fn example1_1() {
    assert_eq!(distance(1), 0);
}

#[test]
fn example1_2() {
    assert_eq!(distance(12), 3);
}

#[test]
fn example1_3() {
    assert_eq!(distance(23), 2);
}

#[test]
fn example1_4() {
    assert_eq!(distance(1024), 31);
}

#[test]
fn my_examples() {
    assert_eq!(distance(9), 2);
    assert_eq!(distance(28), 3);
    assert_eq!(distance(27), 4);
    assert_eq!(distance(26), 5);
}

#[test]
fn problem_1() {
    assert_eq!(distance(361527), 326);
}


#[test]
fn my_example_2() {
    let v = [1, 1, 2, 4, 5, 10, 11, 23, 25, 26, 54, 57, 59, 122, 133, 142, 147, 304, 330, 351, 362, 747, 806, 880, 931];
    assert_eq!(get_n_values(v.len()), v);
}

#[test]
fn problem_2() {
    assert_eq!(363010, get_first_value_larger_than(361527));
}
