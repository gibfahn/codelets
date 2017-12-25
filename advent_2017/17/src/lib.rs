/// Fill a circular buffer with `0..=max_len` values, stepping `step_size` forward each time.
/// Returns the value after the last value inserted.
pub fn fill_buffer_last(max_len: usize, step_size: usize) -> usize {
    let mut buffer = Vec::with_capacity(max_len+1);
    let mut index = 0;
    buffer.push(0);
    for i in 1..max_len+1 {
        index = (index + step_size) % i + 1;
        buffer.insert(index, i);
    }
    buffer[(index + 1) % (max_len + 1)]
}

/// Fill a circular buffer with `0..=max_len` values, stepping `step_size` forward each time.
/// Returns the second value in the buffer (first is always 0).
pub fn fill_buffer_second(max_len: usize, step_size: usize) -> usize {
    let mut index = 0;
    let mut output = 0;
    for i in 1..max_len+1 {
        index = (index + step_size) % i + 1;
        if index == 1 { output = i; }
    }
    output
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(fill_buffer_last(2017, 3), 638);
    }

    #[test]
    fn problem_1() {
        assert_eq!(fill_buffer_last(2017, 316), 180);
    }

    #[test]
    fn example_2() {
        assert_eq!(fill_buffer_second(50_000_000, 354), 10242889);
        assert_eq!(fill_buffer_second(0, 354), 0);
    }

    #[test]
    fn problem_2() {
        assert_eq!(fill_buffer_second(50_000_000, 316), 13326437);
    }
}
