use snafu::{OptionExt, ResultExt, Snafu};

const INPUT: &str = include_str!("input");

#[derive(Debug, Snafu, PartialEq)]
pub enum Error {
    #[snafu(display("Invalid opcode: {}.", opcode))]
    InvalidOpcode { opcode: i32 },
    #[snafu(display("Out of range: {}.", position))]
    OutOfRange { position: usize },
    #[snafu(display("Invalid input: {}.", source))]
    InvalidInput { source: std::num::ParseIntError },
    #[snafu(display("Expected output was not found."))]
    OutputNotFound,
}

pub fn answer() -> (String, String) {
    (
        first(INPUT).unwrap().to_string(),
        second(INPUT, 19_690_720).unwrap().to_string(),
    )
}

trait GetChecked {
    fn get_error(&self, i: usize) -> Result<i32, Error>;
    fn get_mut_error(&mut self, i: usize) -> Result<&mut i32, Error>;
}

impl GetChecked for Vec<i32> {
    fn get_error(&self, i: usize) -> Result<i32, Error> {
        self.get(i).map(|i| *i).context(OutOfRange { position: i })
    }

    fn get_mut_error(&mut self, i: usize) -> Result<&mut i32, Error> {
        self.get_mut(i).context(OutOfRange { position: i })
    }
}

fn first(input: &str) -> Result<i32, Error> {
    let mut int_codes = input
        .trim()
        .split_terminator(',')
        .map(|i| i.parse::<i32>().context(InvalidInput))
        .collect::<Result<Vec<_>, Error>>()?;

    *int_codes.get_mut_error(1)? = 12;
    *int_codes.get_mut_error(2)? = 2;

    run_program(&mut int_codes)?;
    int_codes.get_error(0)
}

fn second(input: &str, output: i32) -> Result<i32, Error> {
    let int_codes = input
        .trim()
        .split_terminator(',')
        .map(|i| i.parse::<i32>().context(InvalidInput))
        .collect::<Result<Vec<_>, Error>>()?;

    for i in 0..100 {
        for j in 0..100 {
            let mut int_codes = int_codes.clone();
            *int_codes.get_mut_error(1)? = i;
            *int_codes.get_mut_error(2)? = j;
            run_program(&mut int_codes)?;
            if output == int_codes.get_error(0)? {
                return Ok(int_codes.get_error(1)? * 100 + int_codes.get_error(2)?);
            }
        }
    }
    OutputNotFound.fail()
}

fn run_program(int_codes: &mut Vec<i32>) -> Result<(), Error> {
    let mut program_counter = 0;

    while let Some(opcode) = int_codes.get(program_counter) {
        match opcode {
            // Add: `1 2 3 4` -> add 2 and 3 and write to position 4.
            1 => {
                let a = int_codes.get_error(int_codes.get_error(program_counter + 1)? as usize)?;
                let b = int_codes.get_error(int_codes.get_error(program_counter + 2)? as usize)?;
                let position = int_codes.get_error(program_counter + 3)? as usize;
                int_codes[position] = a + b;
                program_counter += 4;
            }
            // Multiply: `2 3 4 5` -> multiply 3 and 4 and write to position 5.
            2 => {
                let a = int_codes.get_error(int_codes.get_error(program_counter + 1)? as usize)?;
                let b = int_codes.get_error(int_codes.get_error(program_counter + 2)? as usize)?;
                let position = int_codes.get_error(program_counter + 3)? as usize;
                int_codes[position] = a * b;
                program_counter += 4;
            }
            99 => {
                break;
            }
            _ => InvalidOpcode { opcode: *opcode }.fail()?,
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_examples() {
        let mut input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        run_program(&mut input).unwrap();
        assert_eq!(input, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],);

        let mut input = vec![1, 0, 0, 0, 99];
        run_program(&mut input).unwrap();
        assert_eq!(input, vec![2, 0, 0, 0, 99],);

        let mut input = vec![2, 3, 0, 3, 99];
        run_program(&mut input).unwrap();
        assert_eq!(input, vec![2, 3, 0, 6, 99],);

        let mut input = vec![2, 4, 4, 5, 99, 0];
        run_program(&mut input).unwrap();
        assert_eq!(input, vec![2, 4, 4, 5, 99, 9801],);

        let mut input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        run_program(&mut input).unwrap();
        assert_eq!(input, vec![30, 1, 1, 4, 2, 5, 6, 0, 99],);
    }

    #[test]
    fn first_real() {
        assert_eq!(first(INPUT), Ok(2_890_696));
    }

    #[test]
    fn second_real() {
        assert_eq!(second(INPUT, 19_690_720), Ok(8226));
    }
}
