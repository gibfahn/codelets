#![feature(external_doc)]
#![doc(include = "../Question.md")]

pub fn first() -> String {
    String::from("")
}

pub fn second() -> String {
    String::from("")
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
