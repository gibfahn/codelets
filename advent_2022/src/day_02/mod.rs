use color_eyre::eyre::bail;
use color_eyre::Result;

const INPUT: &str = include_str!("./input");

#[derive(Debug, PartialEq, Copy, Clone)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl RockPaperScissors {
    fn from_char(c: char) -> Result<Self> {
        match c {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => bail!("Unknown input character {c}"),
        }
    }

    fn from_line(line: &str) -> Result<(Self, Self)> {
        let mut chars = line.chars();
        let c = chars.next().unwrap();
        let opponent = match c {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            _ => bail!("Unknown input character {c}"),
        };

        let c = chars.skip(1).next().unwrap();
        let yours = Outcome::from_char(c)
            .unwrap()
            .choice_to_get_output(opponent);
        Ok((opponent, yours))
    }

    fn score_against(self, opponent: Self) -> u64 {
        self.score() + self.outcome(opponent).score()
    }

    fn score(self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn outcome(self, opponent: Self) -> Outcome {
        match (self, opponent) {
            (Self::Rock, Self::Paper) => Outcome::Loss,
            (Self::Rock, Self::Scissors) => Outcome::Win,
            (Self::Paper, Self::Rock) => Outcome::Win,
            (Self::Paper, Self::Scissors) => Outcome::Loss,
            (Self::Scissors, Self::Rock) => Outcome::Loss,
            (Self::Scissors, Self::Paper) => Outcome::Win,
            (_, _) => Outcome::Draw,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Outcome {
    Win,
    Draw,
    Loss,
}
impl Outcome {
    fn score(&self) -> u64 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }

    fn from_char(c: char) -> Result<Self> {
        match c {
            'X' => Ok(Self::Loss),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => bail!("Unknown input character {c}"),
        }
    }

    fn choice_to_get_output(self, opponent: RockPaperScissors) -> RockPaperScissors {
        match (self, opponent) {
            (Outcome::Draw, _) => opponent,
            (Outcome::Win, RockPaperScissors::Rock) => RockPaperScissors::Paper,
            (Outcome::Win, RockPaperScissors::Paper) => RockPaperScissors::Scissors,
            (Outcome::Win, RockPaperScissors::Scissors) => RockPaperScissors::Rock,
            (Outcome::Loss, RockPaperScissors::Rock) => RockPaperScissors::Scissors,
            (Outcome::Loss, RockPaperScissors::Paper) => RockPaperScissors::Rock,
            (Outcome::Loss, RockPaperScissors::Scissors) => RockPaperScissors::Paper,
        }
    }
}

pub fn first() -> String {
    INPUT
        .lines()
        .map(|line| {
            let mut iter = line
                .chars()
                .filter(|c| c.is_ascii_uppercase())
                .map(|c| RockPaperScissors::from_char(c).unwrap());
            let opponent = iter.next().unwrap();
            let yours = iter.next().unwrap();
            yours.score_against(opponent)
        })
        .sum::<u64>()
        .to_string()
}

pub fn second() -> String {
    INPUT
        .lines()
        .map(|line| {
            let (opponent, yours) = RockPaperScissors::from_line(line).unwrap();
            yours.score_against(opponent)
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        assert_eq!(
            dbg!(first()),
            advent_of_code::solve(2022, 2, 1, INPUT).unwrap()
        );
        assert_eq!(
            dbg!(second()),
            advent_of_code::solve(2022, 2, 2, INPUT).unwrap()
        );
    }
}
