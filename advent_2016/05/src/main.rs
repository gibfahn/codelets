#![feature(rustc_private)]
extern crate crypto;
extern crate rustc_serialize;
use crypto::md5::Md5;
use crypto::digest::Digest;
use rustc_serialize::hex::ToHex;
use std::fs::File;
use std::io::prelude::Read;

fn main() {
    let mut file = File::open("./input").expect("Could not open input file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Could not read input file");
    println!("Input: {}", input);
    println!("Puzzle 1: {}", puzzle(&input));
    println!("Puzzle 2: {}", puzzle2(&input));
}

pub fn puzzle2(id: &str) -> String {
    let mut md5 = Md5::new();
    let seed = id.trim().as_bytes();
    let mut output = [None; 8];
    for i in 0..std::u64::MAX {
        md5.input(seed);
        md5.input(i.to_string().as_bytes()); // Concatenate index with seed.
        let mut hash = [0; 16];
        md5.result(&mut hash);
        if hash[..2] == [0, 0] && hash[2] <= 0x07 {
            let pos = hash[2] as usize; // Password position.
            if output[pos] == None {
                output[pos] = Some([hash[3]]
                                       .to_hex()
                                       .chars()
                                       .next()
                                       .unwrap());
                if output.iter().all(|x| x.is_some()) {
                    break;
                }
            }
        }
        md5.reset();
    }
    output.iter().map(|x| x.unwrap()).collect()
}

pub fn puzzle(id: &str) -> String {
    let mut md5 = Md5::new();
    let seed = id.trim().as_bytes();
    let mut output = String::new();
    for i in 0..std::u64::MAX {
        md5.input(seed);
        md5.input(i.to_string().as_bytes()); // Concatenate index with seed.
        let mut hash = [0; 16];
        md5.result(&mut hash);
        if hash[..2] == [0, 0] && hash[2] <= 0x0F {
            output.push([hash[2]]
                            .to_hex()
                            .chars()
                            .skip(1)
                            .next()
                            .unwrap());
            if output.len() == 8 {
                break;
            }
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

    #[test]
    fn get_door_password2() {
        println!();
        let id = "abc";
        let hash = "05ace8e3";
        assert_eq!(puzzle2(id), hash);
    }
}
