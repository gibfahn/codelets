pub struct Firewall {
    layers: Vec<(usize, usize)>,
}

impl Firewall {
    pub fn from(layer_range: &str) -> Self {
        let mut layers: Vec<(usize, usize)> = Vec::new();
        for line in layer_range.trim().lines() {
            let mut words = line.split_whitespace();
            layers.push((
                words.next().unwrap().chars().filter(|&c| c != ':').collect::<String>().parse::<usize>().unwrap(),
                words.next().unwrap().parse::<usize>().unwrap()
                ));
        }
        Firewall { layers }
    }
    pub fn severity(&self) -> usize {
        self.layers.iter().fold(0, |severity, &(layer, range)| {
            severity + if range == 1 || layer % ((range - 1) * 2) == 0 { layer * range } else { 0 }
        })
    }

    fn caught(&self, delay: usize) -> bool {
        self.layers.iter().any(|&(layer, range)| range == 1 || (layer + delay) % ((range - 1) * 2) == 0)
    }

    pub fn min_delay(&self) -> usize {
        (0..std::usize::MAX).find(|&d| !self.caught(d)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_1() {
        let input =
"0: 3
1: 2
4: 4
6: 4";
        assert_eq!(Firewall::from(input).severity(), 24);
    }

    #[test]
    fn problem_1() {
        let input = include_str!("../input.txt");
        assert_eq!(Firewall::from(input).severity(), 1504);
    }

    #[test]
    fn example_1_2() {
        let input =
"0: 3
1: 2
4: 4
6: 4";
        assert_eq!(Firewall::from(input).min_delay(), 10);
    }

    #[test]
    fn problem_2() {
        let input = include_str!("../input.txt");
        assert_eq!(Firewall::from(input).min_delay(), 3823370);
    }
}
