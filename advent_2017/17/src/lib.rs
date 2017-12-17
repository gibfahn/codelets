pub fn fill_buffer(max_len: usize, step_size: usize) -> usize {
    let mut buffer = Vec::with_capacity(max_len+1);
    buffer.push(0);
    let mut index = 0;
    for i in 1..max_len+1 {
        index = (index + step_size) % i + 1;
        // println!("index: {}", index);
        buffer.insert(index, i);
    }
    // println!("buffer: {:?}", buffer);
    // println!("index: {}, max_len: {}, buffer[index]: {}", index, max_len, buffer[index]);
    buffer[(index + 1) % (max_len + 1)]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(fill_buffer(2017, 3), 638);
    }

    #[test]
    fn problem_1() {
        assert_eq!(fill_buffer(2017, 316), 0);
    }
}
