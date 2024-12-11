/*!
Solution for question 2 of the 2022 Informatics Olympiad paper.

Has no package dependencies, and is a single file.

## Run online

You can copy this into <https://play.rust-lang.org> and run the tests or run the main code itself.

Here's a permalink to the current iteration of the code: <https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=640ec3747cd552292264428754a8e43a>
And a link to the code as a gist for easier online browsing: <https://gist.github.com/rust-play/640ec3747cd552292264428754a8e43a>

## Run locally

```shell
# Run code
cargo run

# Run code with debug logging:
DEBUG=1 cargo run

# Run tests to check the solution works for the exam questions:
cargo test

# Debug a specific failure without having to manually type input:
DEBUG=1 c r <<<"25 15
3 13"
```
*/

use std::{array, cmp::Ordering, fmt::Display, io};

/// Number of hexagons in the hive.
const HEXAGONS: usize = 25;
/// Number of edges in each hexagon.
const EDGES: usize = 6;

/**
Write a program that plays the Game of Drones.
Your program should first input a line containing two integers, r (1 ≤ r ≤ 25) then b
(1 ≤ b ≤ 25), indicating the number of hexagons the red and blue drones jump by
during each of the skirmishes. This will be followed by a line containing two
integers, s (0 ≤ s ≤ 1000) then f (0 ≤ f ≤ 40), indicating the number of skirmishes and
feuds respectively.
*/
fn main() {
    let mut input_raw = String::new();
    let stdin = io::stdin();
    eprint!("Enter two integers r and b: ");
    stdin
        .read_line(&mut input_raw)
        .expect("error: unable to read user input line 1");
    eprint!("\nEnter two integers s and f: ");
    stdin
        .read_line(&mut input_raw)
        .expect("error: unable to read user input line 2");
    let input: Vec<usize> = input_raw
        .split_whitespace()
        .map(|word| {
            word.parse::<usize>()
                .expect("Failed to parse user input '{word}'")
        })
        .collect();
    assert_eq!(4, input.len(), "User input should be 4 integers.");

    let (red, blue) = play_game_and_return_controlled(input[0], input[1], input[2], input[3]);
    println!("{}", red);
    println!("{}", blue);
}

/**
Plays a game of drones and:

You should output two lines, the first containing the number of hexagons controlled
by the red colony and the second the number controlled by the blue colony.
*/
fn play_game_and_return_controlled(r: usize, b: usize, s: usize, f: usize) -> (usize, usize) {
    let mut hive = Hive::from(r, b, s, f);
    hive.run_game();

    hive.hexagons_controlled()
}

#[derive(Debug)]
struct Hive {
    hexagons: [Hexagon; HEXAGONS],
    drones: [Drone; 2],
    /// Number of skirmishes to do
    skirmishes: usize,
    /// Number of feuds to do
    feuds: usize,
}

#[derive(Debug, Default, Clone)]
struct Hexagon {
    edges: [Edge; EDGES],
}

#[derive(Debug, Default, Clone)]
struct Edge {
    owner: Option<Colony>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Colony {
    Red,
    Blue,
}

#[derive(Debug)]
struct Drone {
    colony: Colony,
    hexagon: usize,
    edge: usize,
    hexagons_jumped: usize,
}

impl Drone {
    fn rotate(&mut self) {
        match self.colony {
            // 60 degrees clockwise
            Colony::Red => {
                self.edge = (self.edge + 1) % EDGES;
            }
            // 60 degrees anticlockwise
            Colony::Blue => {
                self.edge = (self.edge + 5) % EDGES;
            }
        }
    }

    fn jump(&mut self) {
        self.hexagon = (self.hexagon + self.hexagons_jumped) % HEXAGONS;
    }
}

impl Display for Drone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Drone: {colony:?}, ({hexagon}, {edge})",
            colony = self.colony,
            hexagon = self.hexagon,
            edge = self.edge
        )
    }
}

impl Hive {
    /**
    Create a hive given the 4 parameters.

    The hive is represented by a 5x5 array of hexagons, numbered as in the diagram to the
    right. The six edges of each hexagon are numbered, as in the diagram below. Some
    edges are owned by either the red colony or the blue colony. If a colony owns more
    edges of a hexagon than another colony, that colony controls the hexagon. Initially,
    no edges are owned by either colony and hence no hexagons controlled.
    Two drones are in the hive: a red drone on hexagon 1 facing edge 1 and a blue drone
    on hexagon 25 facing edge 6. Drones jump between hexagons in numeric order,
    returning to 1 after hexagon 25. Whilst jumping, drones do not change the direction
    they are facing.
    */
    fn from(
        red_hexagons_jumped: usize,
        blue_hexagons_jumped: usize,
        skirmishes: usize,
        feuds: usize,
    ) -> Self {
        let drones = [
            Drone {
                hexagon: 0,
                edge: 0,
                colony: Colony::Red,
                hexagons_jumped: red_hexagons_jumped,
            },
            Drone {
                hexagon: 24,
                edge: 5,
                colony: Colony::Blue,
                hexagons_jumped: blue_hexagons_jumped,
            },
        ];

        let hexagons = array::from_fn(|_i| Hexagon::default());

        Hive {
            drones,
            skirmishes,
            feuds,
            hexagons,
        }
    }

    /// The game consists of a number of skirmishes followed by a number of feuds.
    fn run_game(&mut self) {
        self.skirmishes();
        self.feuds();
    }

    /**
    Run the skirmishes.

    In each skirmish:
    • The red drone takes ownership (for the red colony) of the edge it is facing, it then rotates 60°
    clockwise to face a new edge and finally it jumps r hexagons along the hive.
    • The blue drone similarly takes ownership of the edge it is facing, it then rotates 60° anti-clockwise to
    face a new edge before finally jumping b hexagons.
    */
    fn skirmishes(&mut self) {
        for skirmish in 1..=self.skirmishes {
            debug!("Skirmish {skirmish}");
            self.skirmish(0);
            self.skirmish(1);
            debug!("{self}");
        }
    }

    fn skirmish(&mut self, drone_index: usize) {
        let drone = &self.drones[drone_index];
        debug!("Before: {}", &self.drones[drone_index]);
        self.take_ownership(drone.hexagon, drone.edge, drone.colony);

        self.drones[drone_index].rotate();
        self.drones[drone_index].jump();
        debug!("After: {}", &self.drones[drone_index]);
    }

    fn take_ownership(&mut self, hexagon: usize, edge: usize, owner: Colony) {
        self.hexagons[hexagon].edges[edge].owner = Some(owner);
        debug!(
            "{owner:?} took ownership of ({hexagon}, {edge}): Hexagon {h}, edge {e}",
            h = hexagon + 1,
            e = edge + 1
        );
        if let Some((other_hexagon, other_edge)) = Hive::find_other_hexagon_edge(hexagon, edge) {
            self.hexagons[other_hexagon].edges[other_edge].owner = Some(owner);
            debug!(
                "{owner:?} also took ownership of ({other_hexagon}, {other_edge}): Hexagon {h}, edge {e}",
                h = other_hexagon + 1,
                e = other_edge + 1
            );
        }
    }

    /**
    Run the feuds.

    In each feud:
    • The red colony will take ownership of their preferred un-owned edge, followed by the blue colony
    doing likewise.
    • When selecting an edge a colony prefers edges that gain them control over the most hexagons.
    Between edges that give them the same amount of control they prefer those that take away the most
    control from the other colony. After that, preference is based on hexagon number; lowest for the red
    colony and highest for the blue colony. After that, preference is based on direction number; lowest
    for the red colony and highest for the blue colony.
    */
    fn feuds(&mut self) {
        for feud in 1..=self.feuds {
            debug!("Feud {feud}");
            self.feud(0);
            self.feud(1);
            debug!("{self}");
        }
    }

    fn feud(&mut self, drone_index: usize) {
        let drone = &self.drones[drone_index];
        if let Some((hexagon, edge)) = self.edge_to_take(drone.colony) {
            self.take_ownership(hexagon, edge, drone.colony);
        }
    }

    /**
    Choose the edge to select according to the following algorithm.

    When selecting an edge a colony prefers edges that gain them control over the most hexagons.
    Between edges that give them the same amount of control they prefer those that take away the most
    control from the other colony. After that, preference is based on hexagon number; lowest for the red
    colony and highest for the blue colony. After that, preference is based on direction number; lowest
    for the red colony and highest for the blue colony.

    If we can't find an edge to take (because all edges are taken) then we return `None`.
    */
    fn edge_to_take(&self, colony: Colony) -> Option<(usize, usize)> {
        // Possibilities in order of preference and their indexes in the array:
        // - [N/A] We gain 2 (e.g. take an edge between two empty hexagons, or two with equal red and blue edges). Not present because if we find it we early return.
        // - [4] We gain 1, they lose 1 (e.g. take an edge between an empty hexagon and a balanced hexagon).
        // - [3] We gain 1 (e.g. take an edge in an empty hexagon, or one with equal red and blue edges).
        // - [2] They lose 2 (e.g. we go from 2 them, 1 us; to 2 them, 2 us in two hexagons)
        // - [1] They lose 1 (e.g. we go from 2 them, 1 us; to 2 them, 2 us in one hexagons)
        // - [0] No change (e.g. we go from 3 them, 1 us; to 3 them, 2 us in one or two hexagons)
        let mut our_gain_their_loss: [Option<(usize, usize)>; 5] = [None; 5];

        for mut hexagon_index in 0..HEXAGONS {
            // Reverse the ordering if blue.
            if colony == Colony::Blue {
                hexagon_index = HEXAGONS - hexagon_index - 1;
            }

            for mut edge_index in 0..EDGES {
                // Reverse the ordering if blue.
                if colony == Colony::Blue {
                    edge_index = EDGES - edge_index - 1;
                }

                // We only consider unowned edges.
                if self.hexagons[hexagon_index].edges[edge_index]
                    .owner
                    .is_some()
                {
                    continue;
                }

                // Work out who owned it before and after.
                let hexagon_change = self.hexagons[hexagon_index].get_before_after(colony);

                // Work out who owned the other hexagon touching this edge (if it exists) before
                // and after.
                let other_change = if let Some((other_hexagon, other_edge)) =
                    Self::find_other_hexagon_edge(hexagon_index, edge_index)
                {
                    assert!(
                        self.hexagons[other_hexagon].edges[other_edge]
                            .owner
                            .is_none(),
                        "Something went wrong, this edge is owned but the other side of it isn't."
                    );
                    self.hexagons[other_hexagon].get_before_after(colony)
                } else {
                    HexagonChange::NoChange
                };

                let mut our_gain = 0;
                let mut their_loss = 0;

                for change in [hexagon_change, other_change] {
                    match change {
                        HexagonChange::OurGain => our_gain += 1,
                        HexagonChange::TheirLoss => their_loss += 1,
                        HexagonChange::NoChange => {}
                    }
                }

                if our_gain == 2 {
                    // Best result, so immediately return.
                    return Some((hexagon_index, edge_index));
                }

                // our_gain can be 0 or 1 here.
                let index = their_loss + 3 * our_gain;

                our_gain_their_loss[index].get_or_insert((hexagon_index, edge_index));
            }
        }

        // Start at the end of the array and find the first non-empty value.
        our_gain_their_loss
            .into_iter()
            .rev()
            .find(|o| o.is_some())
            .flatten()
    }

    fn hexagons_controlled(&self) -> (usize, usize) {
        let mut red_hexagons = 0;
        let mut blue_hexagons = 0;

        for hexagon in &self.hexagons {
            match hexagon.controlled_by() {
                Some(Colony::Blue) => blue_hexagons += 1,
                Some(Colony::Red) => red_hexagons += 1,
                None => {}
            }
        }

        (red_hexagons, blue_hexagons)
    }

    fn find_other_hexagon_edge(hexagon: usize, edge: usize) -> Option<(usize, usize)> {
        match edge {
            0 => {
                if hexagon <= 4 || hexagon == 9 || hexagon == 19 {
                    None
                } else if (5..=9).contains(&hexagon) || (15..=19).contains(&hexagon) {
                    Some((hexagon - 4, 3))
                } else {
                    Some((hexagon - 5, 3))
                }
            }
            1 => {
                if (hexagon + 1) % 5 == 0 {
                    None
                } else {
                    Some((hexagon + 1, 4))
                }
            }
            2 => {
                if hexagon == 9 || hexagon == 19 || hexagon >= 20 {
                    None
                } else if (0..=4).contains(&hexagon) || (10..=14).contains(&hexagon) {
                    Some((hexagon + 5, 5))
                } else {
                    Some((hexagon + 6, 5))
                }
            }
            3 => {
                if hexagon == 0 || hexagon == 10 || hexagon >= 20 {
                    None
                } else if (0..=4).contains(&hexagon) || (10..=14).contains(&hexagon) {
                    Some((hexagon + 4, 0))
                } else {
                    Some((hexagon + 5, 0))
                }
            }
            4 => {
                if hexagon % 5 == 0 {
                    None
                } else {
                    Some((hexagon - 1, 1))
                }
            }
            5 => {
                if hexagon <= 4 || hexagon == 10 || hexagon == 20 {
                    None
                } else if (5..=9).contains(&hexagon) || (15..=19).contains(&hexagon) {
                    Some((hexagon - 5, 2))
                } else {
                    Some((hexagon - 6, 2))
                }
            }
            _ => panic!("Unexpected edge"),
        }
    }
}

impl Display for Hive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Hive:")?;
        writeln!(f, "     123456 C")?;
        for (i, hexagon) in self.hexagons.iter().enumerate() {
            writeln!(f, " {j:>2}: {hexagon}", j = i + 1)?;
        }
        Ok(())
    }
}

impl Hexagon {
    fn controlled_by(&self) -> Option<Colony> {
        let (red_edges, blue_edges) = self.edge_counts();
        match red_edges.cmp(&blue_edges) {
            Ordering::Less => Some(Colony::Blue),
            Ordering::Equal => None,
            Ordering::Greater => Some(Colony::Red),
        }
    }

    /// Return `(red_edges, blue_edges)`, with number of edges owned by each colony..
    fn edge_counts(&self) -> (i32, i32) {
        let mut red_edges = 0;
        let mut blue_edges = 0;
        for edge in &self.edges {
            match edge.owner {
                Some(Colony::Red) => red_edges += 1,
                Some(Colony::Blue) => blue_edges += 1,
                None => {}
            }
        }
        (red_edges, blue_edges)
    }

    fn get_before_after(&self, colony: Colony) -> HexagonChange {
        let (mut red_edges, mut blue_edges) = self.edge_counts();
        let before = Self::cmp_edges(red_edges, blue_edges);

        match colony {
            Colony::Red => red_edges += 1,
            Colony::Blue => blue_edges += 1,
        }

        let after = Self::cmp_edges(red_edges, blue_edges);

        match (before, after) {
            (None, None) => panic!("Shouldn't be possible to have no-one controlling a hexagon before or after an edge is taken."),
            (None, Some(c)) => {
                assert_eq!(c, colony, "We took an edge, and the hexagon went from unowned to owned, so we should have been the ones to take it.");
                HexagonChange::OurGain},
            (Some(c), None) => {
                assert_ne!(c, colony, "We took an edge, and the hexagon went from owned to unowned, so they should have been the ones to lose it.");
                HexagonChange::TheirLoss},
            (Some(c1), Some(c2)) => {
                assert_eq!(c1, c2, "We took one edge, and the hexagon went from owned to owned, so it should have stayed owned by the same colony.");
                HexagonChange::NoChange
            },
        }
    }

    fn cmp_edges(red_edges: i32, blue_edges: i32) -> Option<Colony> {
        match red_edges.cmp(&blue_edges) {
            Ordering::Less => Some(Colony::Blue),
            Ordering::Equal => None,
            Ordering::Greater => Some(Colony::Red),
        }
    }
}

impl Display for Hexagon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for edge in &self.edges {
            match edge.owner {
                Some(Colony::Red) => write!(f, "R")?,
                Some(Colony::Blue) => write!(f, "B")?,
                None => write!(f, "·")?,
            }
        }
        write!(
            f,
            " {}",
            match self.controlled_by() {
                Some(Colony::Red) => 'R',
                Some(Colony::Blue) => 'B',
                None => '·',
            }
        )
    }
}

#[derive(Debug)]
enum HexagonChange {
    OurGain,
    TheirLoss,
    NoChange,
}

/// Expands to the current function path.
#[macro_export]
macro_rules! debug {
    // debug!("a {} event", "log")
    ($($arg:tt)+) => {{
        if std::env::var("DEBUG").is_ok() {
            std::eprintln!($($arg)+)
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
    Write a program that plays the Game of Drones.
    Your program should first input a line containing two integers, r (1 ≤ r ≤ 25) then b
    (1 ≤ b ≤ 25), indicating the number of hexagons the red and blue drones jump by
    during each of the skirmishes. This will be followed by a line containing two
    integers, s (0 ≤ s ≤ 1000) then f (0 ≤ f ≤ 40), indicating the number of skirmishes and
    feuds respectively.
    Marks are available for cases where there are no feuds
    You should output two lines, the first containing the number of hexagons controlled
    by the red colony and the second the number controlled by the blue colony.
    */
    #[test]
    fn test_question_2a() {
        let (red, blue) = play_game_and_return_controlled(9, 3, 3, 1);
        assert_eq!((6, 6), (red, blue));
    }

    /**
    There are 15 tests used to check 2(a). For each
    test you will need to type in two lines, each
    containing two integers.
    For each test you should see two lines output
    each with a single integer. Both integers need to
    be correct to score marks.
    Tests must terminate in 1 second to receive
    marks.
    */
    #[test]
    fn test_question_2a_markscheme() {
        // (expected_outputs, inputs)
        let test_cases = [
            ((6, 6), (9, 3, 3, 1)),
            ((0, 0), (2, 11, 0, 0)),
            ((1, 2), (1, 1, 1, 0)),
            ((1, 3), (1, 1, 4, 0)),
            ((9, 8), (2, 23, 28, 0)),
            ((17, 7), (11, 5, 20, 0)),
            ((1, 24), (25, 24, 999, 0)),
            ((2, 2), (2, 11, 0, 1)),
            ((14, 9), (16, 25, 7, 3)),
            ((10, 13), (25, 15, 3, 13)),
            ((9, 13), (18, 6, 53, 3)),
            ((7, 16), (25, 24, 11, 3)),
            ((7, 7), (7, 1, 73, 3)),
            ((9, 2), (1, 2, 41, 15)),
            ((5, 3), (1, 14, 31, 19)),
        ];
        for (expected, (r, b, s, f)) in test_cases {
            assert_eq!(expected, play_game_and_return_controlled(r, b, s, f));
        }
    }

    /// Show the hive after 0 skirmishes and 7 feuds, making it clear which edges are owned by each colony.
    #[test]
    fn test_question_2b() {
        let mut hive = Hive::from(9, 3, 0, 7);
        hive.run_game();

        let hive_str = hive.to_string();
        let expected = "\
Hive:
     123456 C
  1: ·R···· R
  2: ····R· R
  3: ·R···· R
  4: ····R· R
  5: ··R··· R
  6: ·R···· R
  7: ····R· R
  8: ·R···· R
  9: ····R· R
 10: ·····R R
 11: ·RB··· ·
 12: ····R· R
 13: ·B···· B
 14: ····B· B
 15: ··B··· B
 16: ··BR·B B
 17: ··B··· B
 18: ··B··· B
 19: ··B··· B
 20: ·····B B
 21: R····· R
 22: ·····B B
 23: ·····B B
 24: ·····B B
 25: ·····B B
";

        assert_eq!(expected, hive_str);
    }

    /// Check that the other edge finding logic is correct.
    #[test]
    fn test_find_other_hexagon_edge() {
        let test_cases = [
            (0, 0, None),
            (0, 1, Some((1, 4))),
            (0, 2, Some((5, 5))),
            (0, 3, None),
            (0, 4, None),
            (0, 5, None),
            (9, 1, None),
            (10, 1, Some((11, 4))),
            (13, 2, Some((18, 5))),
            (15, 4, None),
            (19, 5, Some((14, 2))),
            (22, 4, Some((21, 1))),
            (24, 1, None),
            (24, 2, None),
        ];

        for (hexagon, edge, expected_other) in test_cases {
            assert_eq!(expected_other, Hive::find_other_hexagon_edge(hexagon, edge));
        }
    }
}
