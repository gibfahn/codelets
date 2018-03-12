extern crate twelve;
use twelve::*;

//! Tests in tests/ are used for integration testing, they can't see your private functions.
//! Tests in lib.rs can, so they can be used for unit tests.

#[test]
fn basic() {
    assert_eq!(4, 4);
}

// Parse an instruction
fn instr_parse(s: &str) -> Instruction {
    s.parse().expect("Couldn't parse instruction")
}

#[test]
fn parse_cpy_integer_to_register() {
    assert_eq!(instr_parse("cpy 41 a"),
               Instruction::Copy(FromLocation::Integer(41), Register::A));
}

#[test]
fn parse_cpy_register_to_register() {
    assert_eq!(instr_parse("cpy a c"),
               Instruction::Copy(FromLocation::Register(Register::A), Register::C));
}

#[test]
fn parse_inc_register() {
    assert_eq!(instr_parse("inc c"), Instruction::Increment(Register::C));
}

#[test]
fn parse_dec_register() {
    assert_eq!(instr_parse("dec b"), Instruction::Decrement(Register::B));
}

#[test]
fn parse_jnz() {
    assert_eq!(instr_parse("jnz d 2"),
               Instruction::JumpNonZero(FromLocation::Register(Register::D), 2));
}

#[test]
fn run_their_example() {
    let input = "cpy 41 a
                 inc a
                 inc a
                 dec a
                 jnz a 2
                 dec a";
    assert_eq!(puzzle(input), 42);
}
