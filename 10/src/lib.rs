pub struct KnotHash {
    lengths: Vec<usize>,
    list: Vec<usize>,
    list_size: usize,
    position: usize,
    skip_size: usize,
}

impl KnotHash {
    pub fn from_simple(lengths: &str, list_size: usize) -> Self {
        KnotHash {
            lengths: lengths.trim().split(',')
                .map(|length| length.parse::<usize>().expect(&format!("Couldn't parse {}", length))).collect(),
            list: (0..list_size).collect(),
            list_size,
            position: 0,
            skip_size: 0,
        }
    }
    pub fn from_complex(lengths: &str, list_size: usize) -> Self {
        let mut new_lengths = lengths.trim().chars().map(|c| c as u8 as usize).collect::<Vec<usize>>();
        new_lengths.append(&mut vec![17, 31, 73, 47, 23]);
        KnotHash {
            lengths: new_lengths,
            list: (0..list_size).collect(),
            list_size,
            position: 0,
            skip_size: 0,
        }
    }

    fn knot_hash(&mut self) {
        for &length in &self.lengths {
            if length > 1 && length <= self.list_size {
                let mut start_pos = self.position;
                let mut end_pos = (self.position + length - 1) % self.list_size;
                let mut swap_length = length;
                while swap_length > 1 {
                    self.list.swap(start_pos, end_pos);
                    start_pos = (start_pos + 1) % self.list_size;
                    end_pos = if end_pos == 0 { self.list_size - 1 } else { end_pos - 1 };
                    swap_length -= 2;
                }
            }
            self.position = (self.position + self.skip_size + length) % self.list_size;
            self.skip_size += 1;
        }
    }
    pub fn simple_product(&mut self) -> usize {
        self.knot_hash();
        self.list[0] * self.list[1]
    }

    pub fn complex_product(&mut self) -> String {
        for _ in 0..64 { self.knot_hash(); }
        self.list.chunks(16).map(|chunk|
            format!("{:02x}", chunk.iter().fold(0, |acc, n| acc ^ n))
        ).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_1() {
        assert_eq!(KnotHash::from_simple("3,4,1,5", 5).simple_product(), 12);
    }

    #[test]
    fn example_1_2() {
        assert_eq!(KnotHash::from_simple("5,1,2", 6).simple_product(), 8);
    }

    #[test]
    fn problem_1() {
        assert_eq!(KnotHash::from_simple(include_str!("../input.txt"), 256).simple_product(), 37230);
    }

    #[test]
    fn example_2_1() {
        assert_eq!( KnotHash::from_complex("1,2,3", 256).complex_product(), String::from("3efbe78a8d82f29979031a4aa0b16a9d"));
        assert_eq!( KnotHash::from_complex("AoC 2017", 256).complex_product(), String::from("33efeb34ea91902bb2f59c9920caa6cd"));
        assert_eq!( KnotHash::from_complex("1,2,3", 256).complex_product(), String::from("3efbe78a8d82f29979031a4aa0b16a9d"));
        assert_eq!( KnotHash::from_complex("1,2,4", 256).complex_product(), String::from("63960835bcdc130f0b66d7ff4f6a5a8e"));
    }

    #[test]
    fn problem_2() {
        let input = include_str!("../input.txt");
        assert_eq!( KnotHash::from_complex(input, 256).complex_product(), String::from("70b856a24d586194331398c7fcfa0aaf"));
    }
}
