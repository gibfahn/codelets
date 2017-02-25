use std::fs::File;
use std::io::prelude::*;

extern crate twelve;

fn main() {
    let mut file = File::open("./input.txt")
                         .expect("Could not open input.txt");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Could not read input.txt");

    let answer = twelve::puzzle(&input);
    println!("The answer is {}", answer);
}
