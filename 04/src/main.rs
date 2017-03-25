#![feature(field_init_shorthand)]
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

fn main() {
    let mut file = File::open("./input").expect("Could not open input file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Could not read input");
    let puzzle1: usize = input.lines()
        .map(|s| s.parse::<Room>().expect(&format!("Couldn't parse {} into Room", s)))
        .filter(|r| r.check_checksum())
        .map(|r| r.sector_id)
        .sum();
    println!("First part puzzle output: {}", puzzle1);
    let puzzle2 = input.lines()
        .map(|s| s.parse::<Room>().expect(&format!("Couldn't parse {} into Room", s)))
        .filter(|r| r.check_checksum())
        .map(|mut r| {
                 r.shift_cipher();
                 r
             })
        .filter(|r| r.name.contains("north") || r.name.contains("pole"))
        .collect::<Vec<_>>();
    println!("Second part puzzle output: {:?}", puzzle2);
}

#[derive(Debug, PartialEq)]
pub struct Room {
    name: String,
    checksum: String,
    sector_id: usize,
}

impl Room {
    pub fn shift_cipher(&mut self) {
        let shift_width = (self.sector_id % 26) as u8;
        self.name = self.name
            .chars()
            .map(|mut c| if c == '-' { ' ' } else { c.shift(shift_width) })
            .collect();
    }

    pub fn calc_checksum(&self) -> String {
        use std::collections::HashMap;
        let mut tally: HashMap<char, u32> = HashMap::with_capacity(26);
        for c in self.name.chars().filter(|&c| c != '-') {
            *tally.entry(c).or_insert(0) += 1;
        }
        let mut tally: Vec<(char, u32)> = tally.into_iter().collect();
        tally.sort_by_key(|el| el.0);
        tally.sort_by(|a, b| b.1.cmp(&a.1));
        tally.into_iter()
            .map(|(letter, _)| letter)
            .take(5)
            .collect()
    }

    pub fn check_checksum(&self) -> bool {
        self.calc_checksum() == self.checksum
    }
}

impl FromStr for Room {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rest) = s.split_at(s.rfind('-').ok_or("Couldn't find -")?);
        let name = String::from(name);
        let (sector_id, checksum) = rest.split_at(rest.rfind('[').ok_or("Couldn't find [")?);
        let checksum = checksum.chars().filter(|&c| c != '[' && c != ']').collect();
        let sector_id = sector_id.chars().skip(1).collect::<String>();
        let sector_id = sector_id.parse::<usize>()?;
        Ok(Room {
               name,
               checksum,
               sector_id,
           })
    }
}

trait Shift {
    fn shift(&mut self, u8) -> Self;
}

impl Shift for char {
    fn shift(&mut self, shift: u8) -> char {
        let mut out = *self as u8 + shift;
        while out > 'z' as u8 {
            out -= 26;
        }
        //println!("{} ({}) => {} ({})", *self, *self as u8, out, out as char);
        out as char
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn parse_room(s: &str) -> Room {
        s.parse::<Room>().expect(&format!("Couldn't parse {} into Room", s))
    }

    #[test]
    fn room_parsing_and_checksums() {
        let input = parse_room("aaaaa-bbb-z-y-x-123[abxyz]");
        assert_eq!(input,
                   Room {
                       name: String::from("aaaaa-bbb-z-y-x"),
                       checksum: String::from("abxyz"),
                       sector_id: 123,
                   });
        assert!(input.check_checksum());

        let input = parse_room("a-b-c-d-e-f-g-h-987[abcde]");
        assert_eq!(input,
                   Room {
                       name: String::from("a-b-c-d-e-f-g-h"),
                       checksum: String::from("abcde"),
                       sector_id: 987,
                   });
        assert!(input.check_checksum());

        let input = parse_room("not-a-real-room-404[oarel]");
        assert_eq!(input,
                   Room {
                       name: String::from("not-a-real-room"),
                       checksum: String::from("oarel"),
                       sector_id: 404,
                   });
        assert!(input.check_checksum());

        let input = parse_room("totally-real-room-200[decoy]");
        assert_eq!(input,
                   Room {
                       name: String::from("totally-real-room"),
                       checksum: String::from("decoy"),
                       sector_id: 200,
                   });
        assert!(!input.check_checksum());
    }

    #[test]
    fn shift_chars() {
        println!();
        assert_eq!('a'.shift(2), 'c');
        assert_eq!('z'.shift(2), 'b');
    }

}
