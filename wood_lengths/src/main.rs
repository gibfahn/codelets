//! Wooden Board Usage Calculator
//!
//! Works out the number of boards you need for a project, based on the pieces of wood you need to buy.
//!
//! Modify the constants at the top to fit your project.

/// Types of board you will need.
enum Board {
    Slat,
    Post,
}

impl Board {
    /// Lengths of board you can buy.
    fn length(&self) -> usize {
        match self {
            Board::Slat => 2400,
            Board::Post => 2400,
        }
    }

    /// Number of screws needed (on average) for each wood type.
    fn screws(&self) -> usize {
        match self {
            Board::Slat => 4,
            Board::Post => 2,
        }
    }
}

fn main() {
    println!("{}", get_board_output());
}

#[test]
fn test_project() {
    let expected = "slat;: 19 (segments: 122 | wastage: 2800 | screws: 488)
  2350 => [950, 950, 450] - 50
  2350 => [950, 950, 450] - 50
  2350 => [950, 950, 450] - 50
  2350 => [950, 950, 450] - 50
  2250 => [450, 450, 450, 450, 450] - 150
  2250 => [450, 450, 450, 450, 450] - 150
  2300 => [450, 450, 350, 350, 350, 350] - 100
  2300 => [350, 350, 350, 350, 350, 350, 200] - 100
  2300 => [350, 350, 350, 350, 350, 350, 200] - 100
  2300 => [350, 350, 350, 350, 350, 350, 200] - 100
  2200 => [350, 350, 300, 300, 300, 300, 300] - 200
  2300 => [300, 300, 300, 300, 300, 300, 300, 200] - 100
  2300 => [300, 300, 300, 300, 300, 300, 300, 200] - 100
  2300 => [300, 300, 300, 300, 300, 300, 300, 200] - 100
  2300 => [300, 300, 300, 300, 300, 300, 300, 200] - 100
  2300 => [300, 300, 300, 300, 300, 300, 300, 200] - 100
  2300 => [300, 300, 300, 300, 300, 300, 300, 200] - 100
  2300 => [300, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200] - 100
  1400 => [200, 200, 200, 200, 200, 200, 200] - 1000
post: 7 (segments: 30 | wastage: 2700 | screws: 60)
  2350 => [1000, 1000, 350] - 50
  2350 => [1000, 1000, 350] - 50
  2150 => [850, 850, 450] - 250
  2350 => [450, 450, 450, 350, 350, 300] - 50
  2100 => [350, 350, 350, 350, 350, 350] - 300
  2200 => [350, 350, 300, 300, 300, 300, 300] - 200
  600 => [300, 300] - 1800
";
    assert_eq!(expected, get_board_output());
}

fn get_board_output() -> String {
    let output = String::new();
    output
}
