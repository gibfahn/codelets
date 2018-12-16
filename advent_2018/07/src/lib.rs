#![feature(external_doc)]
#![doc(include = "../Question.md")]
#![feature(vec_remove_item)]

use std::char;
use std::collections::HashMap;
use std::mem;

const INPUT: &str = include_str!("../input");

pub fn answer() -> (String, String) {
    let assembly = Assembly::from(INPUT);
    (
        assembly.clone().order(),
        assembly.work_time(5, 60).to_string(),
    )
}

/// Set of assembly instructions (list of which steps are required to be completed before a
/// given step).
#[derive(Debug, Clone)]
struct Assembly {
    requirements: HashMap<char, Vec<char>>,
}

/// Someone working on an assembly. `Worker::Busy` takes a step name (char) and a finishing time (time
/// when the step will be complete).
#[derive(Debug, Clone)]
enum Worker {
    Busy(char, u32),
    Idle,
}

impl Assembly {
    fn from(s: &str) -> Self {
        let mut requirements = HashMap::with_capacity(26);

        for line in s.lines() {
            let mut words = line.split_whitespace();
            let required = words.nth(1).unwrap().chars().next().unwrap();
            let step = words.nth(5).unwrap().chars().next().unwrap();
            requirements
                .entry(step)
                .or_insert_with(Vec::new)
                .push(required);
            requirements.entry(required).or_insert_with(Vec::new);
        }
        Assembly { requirements }
    }

    /// Return the order in which the instructions should be completed, assuming one
    /// person works through them in order.
    fn order(mut self) -> String {
        let mut out = String::new();

        while !self.requirements.is_empty() {
            let next = self.get_next().unwrap();
            self.requirements.remove(&next);
            out.push(next);

            self.remove_requirement(next);
        }
        out
    }

    /// Get the next instruction to be worked on if there is one. Returns the first
    /// instruction (alphabetically) without requirements.
    fn get_next(&self) -> Option<char> {
        self.requirements
            .iter()
            .fold(None, |first_empty, (c, reqs)| {
                if reqs.is_empty() && (first_empty == None || *c < first_empty.unwrap()) {
                    Some(*c)
                } else {
                    first_empty
                }
            })
    }

    /// Remove a completed requirement from anything which requires it.
    fn remove_requirement(&mut self, c: char) {
        for reqs in self.requirements.values_mut() {
            reqs.remove_item(&c);
        }
    }

    /// Work out how long `worker_count` workers would take to finish the assembly.
    fn work_time(mut self, worker_count: u32, job_delay: u32) -> u32 {
        let mut time: u32 = 0;
        let mut workers = vec![Worker::default(); worker_count as usize];
        while time == 0 || workers.iter().any(|w| w.is_busy()) {
            for worker in workers.iter_mut() {
                match worker {
                    Worker::Busy(c, t) if t == &time => {
                        self.remove_requirement(*c);
                        mem::replace(worker, Worker::Idle);
                    }
                    _ => {}
                }
            }

            for worker in workers.iter_mut() {
                if let Worker::Idle = worker {
                    if let Some(next) = self.get_next() {
                        mem::replace(
                            worker,
                            Worker::Busy(next, time + job_delay + next as u32 - 64),
                        );
                        self.requirements.remove(&next);
                    }
                }
            }
            time += 1;
        }
        time - 1
    }
}

impl Default for Worker {
    fn default() -> Self {
        Worker::Idle
    }
}

impl Worker {
    /// Whether the worker is `Worker::Busy`.
    fn is_busy(&self) -> bool {
        match self {
            Worker::Busy(_, _) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
                     Step C must be finished before step A can begin.\n\
                     Step C must be finished before step F can begin.\n\
                     Step A must be finished before step B can begin.\n\
                     Step A must be finished before step D can begin.\n\
                     Step B must be finished before step E can begin.\n\
                     Step D must be finished before step E can begin.\n\
                     Step F must be finished before step E can begin.\n\
                     ";

        let assembly = Assembly::from(input);
        assert_eq!(assembly.clone().order(), "CABDFE");
        assert_eq!(assembly.work_time(2, 0), 15);
    }

    #[test]
    fn test_answer() {
        assert_eq!(
            answer(),
            (
                String::from("JDEKPFABTUHOQSXVYMLZCNIGRW"),
                String::from("1048")
            )
        );
    }
}
