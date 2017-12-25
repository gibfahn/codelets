use std::str::FromStr;
use std::error::Error;

#[derive(Debug)]
pub enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl FromStr for Move {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.chars().nth(0).ok_or_else(|| "Missing move type".to_string())? {
            's' => Move::Spin(s.chars().skip(1).collect::<String>().parse::<usize>()?),
            'x' => Move::Exchange(s.chars().skip(1).take_while(|&c| c != '/').collect::<String>().parse()?,
                            s.chars().skip_while(|&c| c != '/').skip(1).collect::<String>().parse()?),
            'p' => Move::Partner(s.chars().skip(1).take(1).next().ok_or_else(|| "Missing move from char".to_string())?,
                            s.chars().skip_while(|&c| c != '/').skip(1).take(1).next().ok_or_else(|| "Missing move to char".to_string())?),
            x  => return Err(format!("Unrecognised move: {}", x).into()),
        })
    }
}

/// Returns the resulting program order after executing a series of dance moves (`moves`), on a
/// set of programs from a->z (limited by `length`), a number of times (iterations).
pub fn dance(moves: &str, length: usize, iterations: usize) -> String {
    debug_assert!(length <= 26);
    if length == 1 { return String::from("a") }
    let mut programs = "abcdefghijklmnopqrstuvwxyz".chars().take(length).collect::<Vec<char>>();
    let mut cycles = Vec::with_capacity(length);
    cycles.push(programs.clone());
    let moves: Vec<_> = moves.trim().split(',').map(|word| word.parse::<Move>().unwrap()).collect();
    for _ in 0..iterations {
        for m in &moves {
            match *m {
                Move::Spin(n) => { if n == length || n == 0 { continue; }
                    programs.reverse();
                    let (a, b) = programs.split_at_mut(n);
                    a.reverse(); b.reverse();
                },
                Move::Exchange(i,j) => {programs.swap(i,j);},
                Move::Partner(a,b) => {
                    let i = programs.iter().position(|&c| c == a).unwrap();
                    let j = programs.iter().position(|&c| c == b).unwrap();
                    programs.swap(i,j);
                },
            }
        }
        if cycles.iter().any(|cycle| cycle == &programs) {
            break;
        } else {
            cycles.push(programs.clone());
        }
    }
    let cycle_length = cycles.len();
    if cycle_length < iterations {
        cycles[iterations % cycle_length].clone().into_iter().collect()
    } else {
        programs.into_iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(dance("s1,x3/4,pe/b", 5, 1), String::from("baedc"));
    }

    #[test]
    fn problem_1() {
        assert_eq!(dance(include_str!("../input"), 16, 1), String::from("doeaimlbnpjchfkg"));
    }

    #[test]
    fn problem_2() {
        assert_eq!(dance(include_str!("../input"), 16, 1_000_000_000), String::from("agndefjhibklmocp"));
    }
}

