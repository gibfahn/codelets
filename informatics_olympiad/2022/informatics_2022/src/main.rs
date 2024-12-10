use std::{array, cmp::Ordering};

fn main() {
    let r = 9;
    let b = 3;
    let s = 3;
    let f = 1;

    let (red, blue) = play_game_of_drones(r, b, s, f);
    println!("{}", red);
    println!("{}", blue);
}

fn play_game_of_drones(r: usize, b: usize, s: usize, f: usize) -> (usize, usize) {
    let mut hive = Hive::from(r, b, s, f);
    hive.skirmishes();
    hive.feuds();

    hive.hexagons_controlled()
}

#[derive(Debug)]
struct Hive {
    hexagons: [Hexagon; 25],
    drones: [Drone; 2],
    /// Number of skirmishes to do
    skirmishes: usize,
    /// Number of feuds to do
    feuds: usize,
}

#[derive(Debug, Default)]
struct Hexagon {
    edges: [Edge; 6],
}

#[derive(Debug, Default)]
struct Edge {
    owner: Option<Colony>,
}

#[derive(Debug, Clone, Copy)]
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
                self.edge = (self.edge + 1) % 6;
            }
            // 60 degrees anticlockwise
            Colony::Blue => {
                self.edge = (self.edge + 5) % 6;
            }
        }
    }

    fn jump(&mut self) {
        self.hexagon = (self.hexagon + self.hexagons_jumped) % 25;
    }
}

impl Hive {
    fn from(
        red_hexagons_jumped: usize,
        blue_hexagons_jumped: usize,
        skirmishes: usize,
        feuds: usize,
    ) -> Self {
        let drones = [
            Drone {
                hexagon: 1,
                edge: 1,
                colony: Colony::Red,
                hexagons_jumped: red_hexagons_jumped,
            },
            Drone {
                hexagon: 25,
                edge: 6,
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

    fn skirmishes(&mut self) {
        for _ in 1..=self.skirmishes {
            self.skirmish(0);
            self.skirmish(1);
        }
    }

    fn skirmish(&mut self, drone_index: usize) {
        let drone = &self.drones[drone_index];
        self.take_ownership(drone.hexagon, drone.edge, drone.colony);

        self.drones[drone_index].rotate();
        self.drones[drone_index].jump();
    }

    fn take_ownership(&mut self, hexagon: usize, edge: usize, owner: Colony) {
        self.hexagons[hexagon].edges[edge].owner = Some(owner);
        if let Some((other_hexagon, other_edge)) = Hive::find_other_hexagon_edge(hexagon, edge) {
            self.hexagons[other_hexagon].edges[other_edge].owner = Some(owner);
        }
    }

    fn feuds(&mut self) {
        for _ in 1..=self.feuds {
            self.feud(0);
            self.feud(1);
        }
    }

    fn feud(&mut self, drone_index: usize) {
        let drone = &self.drones[drone_index];
        let (hexagon, edge) = self.edge_to_take(drone.colony);
        self.hexagons[hexagon].edges[edge].owner = Some(drone.colony);
    }

    fn edge_to_take(&self, colony: Colony) -> (usize, usize) {
        let gives_control = todo!();
        todo!()
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

impl Hexagon {
    fn controlled_by(&self) -> Option<Colony> {
        let (red_edges, blue_edges) = self.edge_counts();
        match red_edges.cmp(&blue_edges) {
            Ordering::Less => Some(Colony::Blue),
            Ordering::Equal => None,
            Ordering::Greater => Some(Colony::Red),
        }
    }

    /// Return `(red_edges, blue_edges)`.
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
}

#[cfg(test)]
mod tests {
    use crate::Hive;

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
