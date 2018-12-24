#![feature(external_doc)]
#![doc(include = "../Question.md")]

use failure::Error;

const INPUT: &str = include_str!("../input");

pub fn answer() -> (String, String) {
    let answer_1 = largest_total_power(INPUT).unwrap();
    (format!("{},{}", answer_1.0, answer_1.1), String::from(""))
}

/// ((x + 10) * y + s) * (x + 10)
/// (x + 10)²y + (x + 10)s
/// (xy + 10y + s) * (x + 10)
/// (xxy + 10xy + xs + 10xy + 100y + 10s)
/// (xxy + 20xy + 100y + xs + 10s)
fn power_level((x, y): (i32, i32), serial_number: i32) -> i8 {
    let s = serial_number;

    let pwr = x * x * y + 20 * x * y + 100 * y + x * s + 10 * s;
    ((pwr % 1000 - pwr % 100) / 100) as i8 - 5
}

fn largest_total_power(s: &str) -> Result<(usize, usize), Error> {
    let serial_number = s.trim_end().parse::<i32>()?;
    // let points: Vec<i8> = (0..=300).map(|i| Point{x: i, y: 1}.power_level(serial_number)).collect();
    let points: Vec<Vec<i8>> = (0..=300)
        .map(|y| {
            (0..=300)
                .map(|x| power_level((x, y), serial_number))
                .collect()
        })
        .collect();
    let total_power =
        (0..=298)
            .flat_map(|y| (0..=298).map(move |x| (x, y)))
            .fold(((0, 0), 0), |acc, (x, y)| {
                let total_power = points[y][x]
                    + points[y][x + 1]
                    + points[y][x + 2]
                    + points[y + 1][x]
                    + points[y + 1][x + 1]
                    + points[y + 1][x + 2]
                    + points[y + 2][x]
                    + points[y + 2][x + 1]
                    + points[y + 2][x + 2];
                if total_power >= acc.1 {
                    ((x, y), total_power)
                } else {
                    acc
                }
            });
    println!("{:?}", total_power);
    Ok(total_power.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        assert_eq!(power_level((3, 5), 8), 4);
        assert_eq!(largest_total_power("18").unwrap(), (33, 45));
        assert_eq!(largest_total_power("42").unwrap(), (21, 61));
    }

    #[test]
    fn test_answer() {
        assert_eq!(answer(), (String::from("21,61"), String::from("")));
    }

    #[test]
    fn second_example() {}
}
