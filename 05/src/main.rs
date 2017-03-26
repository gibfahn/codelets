#![feature(rustc_private)]
use std::fs::File;
use std::io::prelude::Read;
extern crate crypto;
extern crate rustc_serialize;

fn main() {
    let mut file = File::open("./input").expect("Could not open input file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Could not read input file");
    println!("Input: {}", input);
    println!("Puzzle 1: {}", puzzle(&input));
}

pub fn puzzle(id: &str) -> String {
    use crypto::md5::Md5;
    use crypto::digest::Digest;
    use rustc_serialize::hex::ToHex;

    let mut md5 = Md5::new();
    println!("id: {}", id);
    let seed = id.trim().as_bytes();
    println!("seed: {:?}", std::str::from_utf8(seed));

    let mut output = String::new();
    for i in 0..std::u64::MAX {
        md5.input(seed);
        md5.input(i.to_string().as_bytes()); // Concatenate index with seed.
        let mut hash = [0; 16];
        md5.result(&mut hash);
        if hash[..2] == [0,0] && hash[2] <= 0x0F {
            println!();
            println!("i: {}", i);
            println!("Hash: {:?}", hash.to_vec());
            println!("Interesting: {}", hash[2]);
            output.push([hash[2]].to_hex().chars().skip(1).next().unwrap());
            println!("Output: {}", output);
            println!();
            if output.len() == 8 { break; }
        }
        md5.reset();
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn get_door_password() {
        println!();
        let id = "abc";
        let hash = "18f47a30";
        assert_eq!(puzzle(id), hash);
    }
}
