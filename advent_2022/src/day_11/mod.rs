use color_eyre::eyre::eyre;
use std::str::FromStr;

pub const DAY: u8 = 11;
pub const INPUT: &str = include_str!("./input");

#[derive(Debug)]
struct Monkeys(Vec<Monkey>);

impl FromStr for Monkeys {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let monkeys = s
            .split("\n\n")
            .map(|block| block.parse())
            .collect::<Result<_, _>>()?;
        Ok(Self(monkeys))
    }
}

#[derive(Debug, Copy, Clone)]
enum Operator {
    Multiply,
    Add,
}

impl FromStr for Operator {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Self::Multiply),
            "+" => Ok(Self::Add),
            _ => Err(eyre!("Unexpected input {s}")),
        }
    }
}

/// ```text
/// Monkey 0:
///   Starting items: 79, 98
///   Operation: new = old * 19
///   Test: divisible by 23
///     If true: throw to monkey 2
///     If false: throw to monkey 3
/// ```
#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation_operator: Operator,
    operation_amount: usize,
    test_divisible_by: usize,
    throw_if_true: usize,
    throw_if_false: usize,
}

impl FromStr for Monkey {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let _ = lines
            .next()
            .ok_or_else(|| eyre!("Expected monkey ID line."))?; // Ignore Monkey ID.

        let line = lines
            .next()
            .ok_or_else(|| eyre!("Expected starting items line."))?;
        let items = line
            .split_whitespace()
            .skip(2)
            .map(|word| word.replace(',', "").parse::<usize>())
            .collect::<Result<_, _>>()?;

        let line = lines
            .next()
            .ok_or_else(|| eyre!("Expected operation line."))?;
        let mut words = line.split_whitespace().skip(4);
        let operation_operator = words
            .next()
            .ok_or_else(|| eyre!("Expected operation operator."))?
            .parse::<Operator>()?;
        let operation_amount = words
            .next()
            .ok_or_else(|| eyre!("Expected operation amount."))?
            .parse::<usize>()?;

        let line = lines.next().ok_or_else(|| eyre!("Expected test line."))?;
        let test_divisible_by = line
            .split_whitespace()
            .last()
            .ok_or_else(|| eyre!("Expected test divisible by amount."))?
            .parse::<usize>()?;

        let line = lines
            .next()
            .ok_or_else(|| eyre!("Expected throw_if_true line."))?;
        let throw_if_true = line
            .split_whitespace()
            .last()
            .ok_or_else(|| eyre!("Expected throw_if_true."))?
            .parse::<usize>()?;

        let line = lines
            .next()
            .ok_or_else(|| eyre!("Expected throw_if_false line."))?;
        let throw_if_false = line
            .split_whitespace()
            .last()
            .ok_or_else(|| eyre!("Expected throw_if_false."))?
            .parse::<usize>()?;

        Ok(Self {
            items,
            operation_operator,
            operation_amount,
            test_divisible_by,
            throw_if_true,
            throw_if_false,
        })
    }
}

pub fn first(input: &str) -> String {
    let mut monkeys: Monkeys = input.parse().unwrap();
    todo!()
}

pub fn second(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[ignore]
    #[test]
    fn check() {
        assert_eq!(
            dbg!(first(INPUT)),
            advent_of_code::solve(2022, DAY, 1, INPUT).unwrap()
        );

        assert_eq!(
            dbg!(second(INPUT)),
            advent_of_code::solve(2022, DAY, 2, INPUT).unwrap()
        );
    }

    #[test]
    fn examples() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

        assert_eq!(dbg!(first(input)), "10605");
        // assert_eq!(dbg!(second(input)), "");
    }
}
