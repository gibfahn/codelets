pub const DAY: u8 = 10;
pub const INPUT: &str = include_str!("./input");

const SCREEN_SIZE: usize = 40 * 6;

#[derive(Debug)]
struct Cpu {
    counter: usize,
    x: i64,
    interesting_signal_strengths: i64,
    screen: [bool; SCREEN_SIZE],
}

impl Cpu {
    fn new() -> Self {
        Self {
            x: 1,
            screen: [false; SCREEN_SIZE],
            counter: 0,
            interesting_signal_strengths: 0,
        }
    }

    fn run_program(&mut self, input: &str) {
        for line in input.lines() {
            self.tick();
            let mut words = line.split_whitespace();
            match words.next().unwrap() {
                "addx" => self.run_addx(words.next().unwrap().parse().unwrap()),
                "noop" => (),
                x => panic!("Unexpected input instruction {x}"),
            };
        }
    }

    fn tick(&mut self) {
        self.screen[self.counter] = (self.x - (self.counter as i64) % 40).abs() <= 1;
        self.counter += 1;
        if (self.counter + 20) % 40 == 0 {
            self.interesting_signal_strengths += self.counter as i64 * self.x;
        }
    }

    fn run_addx(&mut self, increment: i64) {
        self.tick();
        self.x += increment;
    }

    fn print_image(&self) -> String {
        let mut output = String::new();
        for (i, pixel) in self.screen.iter().enumerate() {
            if i != 0 && i % 40 == 0 {
                output.push('\n');
            }
            if *pixel {
                output.push('#');
            } else {
                output.push('.');
            }
        }
        eprintln!("Screen:\n{output}");
        output
    }
}

pub fn first(input: &str) -> String {
    let mut cpu = Cpu::new();
    cpu.run_program(input);
    cpu.interesting_signal_strengths.to_string()
}

pub fn second(input: &str) -> String {
    let mut cpu = Cpu::new();
    cpu.run_program(input);
    cpu.print_image()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        assert_eq!(
            dbg!(first(INPUT)),
            advent_of_code::solve(2022, DAY, 1, INPUT).unwrap()
        );

        let output = "####..##..####.#..#.####..##..#....###..
#....#..#....#.#..#....#.#..#.#....#..#.
###..#......#..#..#...#..#..#.#....#..#.
#....#.....#...#..#..#...####.#....###..
#....#..#.#....#..#.#....#..#.#....#.#..
####..##..####..##..####.#..#.####.#..#.";
        assert_eq!(
            dbg!(second(INPUT)),
            output,
            // advent_of_code::solve(2022, DAY, 2, INPUT).unwrap()
        );
    }

    #[test]
    fn examples() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

        let second_result = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

        assert_eq!(dbg!(first(input)), "13140");
        assert_eq!(dbg!(second(input)), second_result);
    }
}
