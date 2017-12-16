#![feature(inclusive_range_syntax)]

/// Struct used in construction of the Knot Hash.
pub struct KnotHash {
    lengths: Vec<u8>,
    list: Vec<u8>,
    list_max: u8,
    position: u8,
    skip_size: usize,
}

impl KnotHash {
    /// Converts a string to a list of lengths by converting characters to Ascii number,
    /// and appending the magic vector `[17,31,73,47,23]`.
    pub fn from(lengths: &str, list_max: u8) -> Self {
        let mut new_lengths: Vec<_> = lengths.trim().as_bytes().to_vec();
        new_lengths.append(&mut vec![17, 31, 73, 47, 23]);
        KnotHash {
            lengths: new_lengths,
            list: (0..=list_max).collect(),
            list_max,
            position: 0,
            skip_size: 0,
        }
    }

    /// Returns a vector of bytes (`u8`) rather than the string representation.
    pub fn knot_hash_vec(&mut self) -> Vec<u8> {
        for _ in 0..64 { self.hash(); }
        self.list.chunks(16).map(|chunk|
            chunk.iter().fold(0, |acc, n| acc ^ n)
        ).collect()
    }

    /// Returns a String representing the Knot Hash in Hex.
    pub fn knot_hash(&mut self) -> String {
        for _ in 0..64 { self.hash(); }
        self.list.chunks(16).map(|chunk|
            format!("{:02x}", chunk.iter().fold(0, |acc, n| acc ^ n))
        ).collect()
    }

    /// Converts a list of lengths to something that can be hashed.
    pub fn from_simple(lengths: &str, list_max: u8) -> Self {
        KnotHash {
            lengths: lengths.trim().split(',')
                .map(|length| length.parse::<u8>().expect(&format!("Couldn't parse {}", length))).collect(),
            list: (0..=list_max).collect(),
            list_max,
            position: 0,
            skip_size: 0,
        }
    }

    pub fn simple_hash(&mut self) -> usize {
        self.hash();
        self.list[0] as usize * self.list[1] as usize
    }

    pub fn hash(&mut self) {
        let list_len: usize = self.list_max as usize + 1;
        for &length in &self.lengths {
            if length > 1  && length <= self.list_max {
                let mut start_pos = self.position as usize;
                let mut end_pos = (self.position as usize + length as usize - 1) % list_len;
                let mut swap_length = length;
                while swap_length > 1 {
                    self.list.swap(start_pos, end_pos);
                    start_pos = (start_pos + 1) % list_len;
                    end_pos = if end_pos == 0 { self.list_max as usize } else { end_pos - 1 };
                    swap_length -= 2;
                }
            }
            self.position = ((self.position as usize + self.skip_size as usize + length as usize) % list_len) as u8;
            self.skip_size += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_1() {
        assert_eq!(KnotHash::from_simple("3,4,1,5", 4).simple_hash(), 12);
    }

    #[test]
    fn example_1_2() {
        assert_eq!(KnotHash::from_simple("5,1,2", 5).simple_hash(), 8);
    }

    #[test]
    fn problem_1() {
        assert_eq!(KnotHash::from_simple(include_str!("../input.txt"), 255).simple_hash(), 37230);
    }

    #[test]
    fn example_2_1() {
        assert_eq!(&KnotHash::from("1,2,3", 255).knot_hash(), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(&KnotHash::from("AoC 2017", 255).knot_hash(), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(&KnotHash::from("1,2,3", 255).knot_hash(), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(&KnotHash::from("1,2,4", 255).knot_hash(), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }

    #[test]
    fn problem_2() {
        let input = include_str!("../input.txt");
        assert_eq!(&KnotHash::from(input, 255).knot_hash(), "70b856a24d586194331398c7fcfa0aaf");
    }
}
