#![feature(external_doc)]
#![doc(include = "../Question.md")]

const INPUT: &str = include_str!("../input");

pub fn answer() -> (String, String) {
    (String::from(""), String::from(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {}

    #[test]
    fn test_answer() {
        assert_eq!(answer(), (String::from(""), String::from("")));
    }

    #[test]
    fn second_example() {}
}
