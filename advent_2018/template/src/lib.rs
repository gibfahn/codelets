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
    fn first_answer() {
        assert_eq!(&first(), "");
    }

    #[test]
    fn second_example() {}

    #[test]
    fn second_answer() {
        assert_eq!(&second(), "");
    }
}
