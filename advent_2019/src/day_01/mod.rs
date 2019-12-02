const INPUT: &str = include_str!("input");

pub fn answer() -> (String, String) {
    (first(INPUT).to_string(), second(INPUT).to_string())
}

/// Specifically, to find the fuel required for a module, take its mass, divide by three, round
/// down, and subtract 2. individually calculate the fuel needed for the mass of each module (your
/// puzzle input), then add together all the fuel values.
fn first(input: &str) -> i32 {
    input
        .split_terminator('\n')
        .map(|i| i.parse::<i32>().unwrap() / 3 - 2)
        .sum()
}

///Fuel itself requires fuel just like a module - take its mass, divide by three, round down, and subtract 2. However, that fuel also requires fuel, and that fuel requires fuel, and so on. Any mass that would require negative fuel should instead be treated as if it requires zero fuel; the remaining mass, if any, is instead handled by wishing really hard, which has no mass and is outside the scope of this calculation.
///
///So, for each module mass, calculate its fuel and add it to the total. Then, treat the fuel amount you just calculated as the input mass and repeat the process, continuing until a fuel requirement is zero or negative. For example:
fn second(input: &str) -> i32 {
    input
        .split_terminator('\n')
        .map(|i| {
            let mut total = 0;
            let mut x = i.parse::<i32>().unwrap();
            while {
                x = x / 3 - 2;
                x > 0
            } {
                total += x;
            }
            total
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
    /// For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
    /// For a mass of 1969, the fuel required is 654.
    /// For a mass of 100756, the fuel required is 33583.
    #[test]
    fn first_examples() {
        assert_eq!(first("12"), 2);
        assert_eq!(first("14"), 2);
        assert_eq!(first("1969"), 654);
        assert_eq!(first("100756"), 33583);
    }

    #[test]
    fn first_real() {
        assert_eq!(first(INPUT), 3345909);
    }

    // A module of mass 14 requires 2 fuel. This fuel requires no further fuel (2 divided by 3 and rounded down is 0, which would call for a negative fuel), so the total fuel required is still just 2.
    // At first, a module of mass 1969 requires 654 fuel. Then, this fuel requires 216 more fuel (654 / 3 - 2). 216 then requires 70 more fuel, which requires 21 fuel, which requires 5 fuel, which requires no further fuel. So, the total fuel required for a module of mass 1969 is 654 + 216 + 70 + 21 + 5 = 966.
    // The fuel required by a module of mass 100756 and its fuel is: 33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.
    #[test]
    fn second_examples() {
        assert_eq!(second("14"), 2);
        assert_eq!(second("1969"), 966);
        assert_eq!(second("100756"), 50346);
    }

    #[test]
    fn second_real() {
        assert_eq!(second(INPUT), 5015983)
    }
}
