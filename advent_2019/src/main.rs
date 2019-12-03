mod day_01;
mod day_02;
mod day_03;

fn main() {
    println!("\nAdvent of Code 2019 Answers:\n");

    let day_fns: Vec<(fn() -> (String, String))> =
        vec![day_01::answer, day_02::answer, day_03::answer];

    for (n, answer) in day_fns.iter().enumerate() {
        let (first, second) = answer();
        println!("{:02}: ({:?}, {:?})", n + 1, first, second);
    }
}
