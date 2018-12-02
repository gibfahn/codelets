#![feature(external_doc)]
#![doc(include = "../../README.md")]

fn main() {
    println!("\nAdvent of Code 2018 Answers:\n");

    type StringFnPair = Vec<(fn() -> String, fn() -> String)>;

    let day_fns: StringFnPair = vec![(one::first, one::second), (two::first, two::second)];

    for (n, (first, second)) in day_fns.iter().enumerate() {
        println!("{:02}: {}, {}", n + 1, first(), second());
    }
}
