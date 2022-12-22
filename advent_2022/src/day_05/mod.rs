use color_eyre::{eyre::eyre, Result};

const INPUT: &str = include_str!("./input");

#[derive(Debug)]
struct Stacks(Vec<Vec<char>>);

/**

35 chars per line
8 whitespace
27 (3 x 9) chars per stack (`[A]`)

- Throw 1 char
- Take char if not space
- throw 3 chars

```text
[J]             [F] [M]
[Z] [F]     [G] [Q] [F]
[G] [P]     [H] [Z] [S] [Q]
[V] [W] [Z] [P] [D] [G] [P]
[T] [D] [S] [Z] [N] [W] [B] [N]
[D] [M] [R] [J] [J] [P] [V] [P] [J]
[B] [R] [C] [T] [C] [V] [C] [B] [P]
[N] [S] [V] [R] [T] [N] [G] [Z] [W]
 1   2   3   4   5   6   7   8   9
```
*/
impl Stacks {
    fn from_str(stacks_str: &str) -> Result<Self> {
        let mut stacks = Self(vec![Vec::new(); 9]);
        for line in stacks_str.lines() {
            let mut chars = line.chars().peekable();
            if chars.next() != Some('[') {
                continue;
            }
            let mut stack = 0;
            loop {
                let Some(c) = chars.next() else {
                    break;
                };
                if !c.is_whitespace() {
                    stacks.0[stack].push(c);
                }
                stack += 1;
                let _ = chars.next();
                let _ = chars.next();
                let _ = chars.next();
            }
        }
        for stack in &mut stacks.0 {
            stack.reverse();
        }
        Ok(stacks)
    }

    /**
    6 words
    2nd word (index 1) number of crates
    4th word (index 3) starting stack
    6th word (index 5) ending stack

    ```text
    move 2 from 4 to 6
    ```
    */
    fn rearrange(&mut self, procedure_str: &str, model: Model) -> Result<()> {
        for line in procedure_str.lines() {
            let words: Vec<_> = line.split_whitespace().collect();
            self.move_crates(
                model,
                words[1].parse::<usize>()?,
                words[3].parse::<usize>()? - 1,
                words[5].parse::<usize>()? - 1,
            )?;
        }
        Ok(())
    }

    fn move_crates(&mut self, model: Model, count: usize, from: usize, to: usize) -> Result<()> {
        match model {
            Model::CrateMover9000 => {
                for _ in 0..count {
                    let value = self.0[from]
                        .pop()
                        .ok_or_else(|| eyre!("Stack {from} is empty."))?;
                    self.0[to].push(value);
                }
            }
            Model::CrateMover9001 => {
                let from_len = self.0[from].len();
                let elements: Vec<_> = self.0[from].drain(from_len - count..).collect();
                self.0[to].extend(elements);
            }
        }
        Ok(())
    }

    fn top_crates(&self) -> Result<String> {
        let mut output = String::new();
        for stack in &self.0 {
            if let Some(top) = stack.last() {
                output.push(*top);
            }
        }
        Ok(output)
    }
}

#[derive(Copy, Clone, Debug)]
enum Model {
    CrateMover9000,
    CrateMover9001,
}

pub fn first() -> String {
    let (stacks_str, procedure_str) = INPUT.split_once("\n\n").unwrap();

    let mut stacks = Stacks::from_str(stacks_str).unwrap();

    stacks
        .rearrange(procedure_str, Model::CrateMover9000)
        .unwrap();
    stacks.top_crates().unwrap()
}

pub fn second() -> String {
    let (stacks_str, procedure_str) = INPUT.split_once("\n\n").unwrap();

    let mut stacks = Stacks::from_str(stacks_str).unwrap();

    stacks
        .rearrange(procedure_str, Model::CrateMover9001)
        .unwrap();
    stacks.top_crates().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        assert_eq!(
            dbg!(first()),
            advent_of_code::solve(2022, 5, 1, INPUT).unwrap()
        );
        assert_eq!(
            dbg!(second()),
            advent_of_code::solve(2022, 5, 2, INPUT).unwrap()
        );
    }
}
