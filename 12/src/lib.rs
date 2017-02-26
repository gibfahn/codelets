use std::error::Error;
use std::str::FromStr;

pub fn puzzle(input: &str) -> i32 {
    let program: Vec<Instruction> = input.lines()
                                         .map(|line| line.trim())
                                         //.inspect(|x| println!("about to parse: {:?}", x))
                                         .map(|line| line.parse().expect("Parse error"))
                                         //.inspect(|x| println!("finished parsing: {:?}", x))
                                         .collect();
    let mut machine = Machine::new();
    machine.execute(&program);
    machine.value_of(Register::A)
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Instruction {
    Copy(FromLocation, Register),
    Increment(Register),
    Decrement(Register),
    JumpNonZero(FromLocation, i32),
}

impl FromStr for Instruction {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        match words.next() {
            Some("cpy") => Ok(Instruction::Copy(
                    words.next().ok_or("FromLocation not found")?.parse()?,
                    words.next().ok_or("Register not found")?.parse()?)),
            Some("inc") => Ok(Instruction::Increment(
                    words.next().ok_or("Register not found")?.parse()?)),
            Some("dec") => Ok(Instruction::Decrement(
                    words.next().ok_or("Register not found")?.parse()?)),
            Some("jnz") => Ok(Instruction::JumpNonZero(
                    words.next().ok_or("Register not found")?.parse()?,
                    words.next().ok_or("Offset not found")?.parse()?)),
            Some(other) => Err(format!("Unknown instruction: {}", other).into()),
            None => Err("Instruction missing".into()),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum FromLocation {
    Integer(i32),
    Register(Register),
}

impl FromStr for FromLocation {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(FromLocation::Integer)
            .or_else(|_| s.parse().map(FromLocation::Register))
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Register {
    A,B,C,D,
}

impl FromStr for Register {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Register::*;
        match s {
            "a" => Ok(A),
            "b" => Ok(B),
            "c" => Ok(C),
            "d" => Ok(D),
            _ => Err(format!("Unknown register {}", s).into()),
        }
    }
}

#[derive(Debug)]
pub struct Machine {
    registers: [i32; 4],
    program: Vec<Instruction>,
    prog_count: usize,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            registers: [0; 4],
            program: Vec::new(),
            prog_count: 0,
        }
    }

    fn execute(&mut self, instructions: &[Instruction]) {
        self.program.extend_from_slice(instructions);

        while self.prog_count < self.program.len() {
            //println!("Machine: Program Count: {:?}, Registers: {:?}", self.prog_count, self.registers);
            match self.program[self.prog_count] {
                Instruction::Copy(FromLocation::Integer(from), to) => {
                    self.registers[Machine::register_index(to)] = from;
                },
                Instruction::Copy(FromLocation::Register(from), to) => {
                    self.registers[Machine::register_index(to)] = self.registers[Machine::register_index(from)];
                },
                Instruction::Increment(register) => {
                    self.registers[Machine::register_index(register)] += 1;
                },
                Instruction::Decrement(register) => {
                    self.registers[Machine::register_index(register)] -= 1;
                },
                Instruction::JumpNonZero(condition, offset) => {
                    let condition = match condition {
                        FromLocation::Integer(x) => x,
                        FromLocation::Register(x) => self.registers[Machine::register_index(x)],
                    };
                    if condition != 0 {
                        self.prog_count = (self.prog_count as i32 + offset -1) as usize;
                    }
                },
            }
            self.prog_count += 1;
        }
    }

    fn register_index(register: Register) -> usize {
        use Register::*;
        match register {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
        }
    }

    fn value_of(&self, register: Register) -> i32 {
        self.registers[Machine::register_index(register)]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
     * This module can access your library's private functions. You shouldn't need to test these
     * functions, but if you want to, this is the place to do it.
     */

    #[test]
    fn run_cpy() {
        let mut machine = Machine::new();
        machine.execute(&[Instruction::Copy(FromLocation::Integer(5), Register::A)]);
        assert_eq!(machine.value_of(Register::A), 5);
        machine.execute(
            &[Instruction::Copy(FromLocation::Register(Register::A), Register::D)]
        );
        assert_eq!(machine.value_of(Register::D), 5);
    }

    #[test]
    fn run_inc_dec() {
        let mut machine = Machine::new();
        machine.execute(&[Instruction::Decrement(Register::B)]);
        assert_eq!(machine.value_of(Register::B), -1);

        machine.execute(&[Instruction::Copy(FromLocation::Integer(5), Register::B)]);
        machine.execute(&[Instruction::Increment(Register::B)]);
        assert_eq!(machine.value_of(Register::B), 6);
    }

    #[test]
    fn run_jnz_jump_of_zero() {
        let mut machine = Machine::new();
        machine.execute(&[
                        Instruction::Increment(Register::A),
                        Instruction::Increment(Register::D),
                        Instruction::JumpNonZero(FromLocation::Register(Register::D), 0),
                        Instruction::Increment(Register::A),
        ]);
        assert_eq!(machine.value_of(Register::A), 2);
    }

    #[test]
    fn run_jnz_zero_condition_jump() {
        let mut machine = Machine::new();
        machine.execute(&[
                        Instruction::Increment(Register::A),
                        Instruction::JumpNonZero(FromLocation::Register(Register::D), -1),
                        Instruction::Increment(Register::A),
        ]);
        assert_eq!(machine.value_of(Register::A), 2);
    }

    #[test]
    fn run_jnz_jump_zero() {
        let mut machine = Machine::new();
        machine.execute(&[
                        Instruction::Increment(Register::C),
                        Instruction::JumpNonZero(FromLocation::Register(Register::C), 0),
                        Instruction::Increment(Register::D),
                        Instruction::Increment(Register::C),
        ]);
        assert_eq!(machine.value_of(Register::C), 2);
        assert_eq!(machine.value_of(Register::D), 1);
    }

    #[test]
    fn run_jnz_forward_jump_one() {
        let mut machine = Machine::new();
        machine.execute(&[
                        Instruction::Increment(Register::C),
                        Instruction::JumpNonZero(FromLocation::Register(Register::C), 1),
                        Instruction::Increment(Register::D),
                        Instruction::Increment(Register::C),
        ]);
        assert_eq!(machine.value_of(Register::C), 2);
        assert_eq!(machine.value_of(Register::D), 0);
    }

    #[test]
    fn run_jnz_forward_jump_two() {
        let mut machine = Machine::new();
        machine.execute(&[
                        Instruction::Increment(Register::C),
                        Instruction::JumpNonZero(FromLocation::Register(Register::C), 2),
                        Instruction::Increment(Register::D),
                        Instruction::Increment(Register::C),
        ]);
        assert_eq!(machine.value_of(Register::C), 1);
        assert_eq!(machine.value_of(Register::D), 0);
    }
}
