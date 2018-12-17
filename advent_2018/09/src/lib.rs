#![feature(external_doc)]
#![doc(include = "../Question.md")]

use std::collections::VecDeque;

const INPUT: &str = include_str!("../input");

pub fn answer() -> (String, String) {
    let mut game = Game::from(INPUT);
    (
        game.high_score().to_string(),
        game.hundred_game_score().to_string(),
    )
}

struct Game {
    player_count: usize,
    last_marble: usize,
}

impl Game {
    fn from(s: &str) -> Self {
        let mut words = s.split_whitespace();
        Game {
            player_count: words.next().unwrap().parse::<usize>().unwrap(),
            last_marble: words.nth(5).unwrap().parse::<usize>().unwrap(),
        }
    }

    fn hundred_game_score(&mut self) -> usize {
        self.last_marble *= 100;
        self.high_score()
    }

    fn high_score(&self) -> usize {
        let mut board = VecDeque::with_capacity(self.last_marble);
        board.push_front(0);
        let mut scores = vec![0; self.player_count];

        for marble in 1..=self.last_marble {
            if marble % 23 == 0 {
                for _ in 0..7 {
                    let m = board.pop_back().unwrap();
                    board.push_front(m);
                }
                scores[marble % self.player_count] += marble + board.pop_front().unwrap();
            } else {
                for _ in 0..2 {
                    let m = board.pop_front().unwrap();
                    board.push_back(m);
                }
                board.push_front(marble);
            }
        }
        *scores.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        let inputs = [
            "9 players; last marble is worth 25 points",
            "10 players; last marble is worth 1618 points",
            "13 players; last marble is worth 7999 points",
            "17 players; last marble is worth 1104 points",
            "21 players; last marble is worth 6111 points",
            "30 players; last marble is worth 5807 points",
        ];
        let high_scores = [32, 8317, 146373, 2764, 54718, 37305];
        for (input, score) in inputs.iter().zip(&high_scores) {
            assert_eq!(Game::from(input).high_score(), *score);
        }
    }

    #[test]
    fn test_answer() {
        assert_eq!(
            answer(),
            (String::from("398242"), String::from("3273842452"))
        );
    }
}
