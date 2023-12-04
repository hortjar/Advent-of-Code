use std::collections::HashMap;

use crate::solutions::AoCDay;

pub struct Day04;

impl Day04 {
    fn parse_numbers(&self, input: &str) -> Vec<u32> {
        return input
            .trim()
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|num| num.trim().parse::<u32>().unwrap_or_default())
            .collect::<Vec<u32>>();
    }
    fn parse_cards(&self, input: &str) -> Vec<Card> {
        return input
            .lines()
            .map(|line| {
                let l = line.replace("Card ", "");
                let split = l.split(|c| c == ':' || c == '|').collect::<Vec<&str>>();

                let card = &split
                    .get(0)
                    .expect("No card on index 0")
                    .trim()
                    .parse::<u32>()
                    .unwrap_or_default();

                let winning_nums =
                    self.parse_numbers(&split.get(1).expect("Mo numbers on index 1"));
                let drawn_nums = self.parse_numbers(&split.get(2).expect("Mo numbers on index 2"));

                return Card {
                    card: *card,
                    winning_numbers: winning_nums.to_owned(),
                    drawn_numbers: drawn_nums.to_owned(),
                };
            })
            .collect::<Vec<Card>>();
    }
}

impl AoCDay for Day04 {
    fn part01(&self) -> String {
        let input = self.read_input(4, 1);

        let sum = self
            .parse_cards(&input)
            .iter()
            .map(|card| card.score())
            .sum::<u32>();

        return sum.to_string();
    }

    fn part02(&self) -> String {
        let input = self.read_input(4, 2);
        let mut sum = 0;

        let parsed = self.parse_cards(&input);
        let cards = parsed.iter().map(|card| {
            let size = card.matches().len();
            let mut hm: HashMap<u32, Vec<usize>> = HashMap::new();
            hm.insert(card.card, vec![size]);
            return hm;
        });

        for card_map in cards {}

        return sum.to_string();
    }
}

struct Card {
    card: u32,
    winning_numbers: Vec<u32>,
    drawn_numbers: Vec<u32>,
}

impl Card {
    fn matches(&self) -> Vec<u32> {
        return self
            .drawn_numbers
            .iter()
            .filter(|x| self.winning_numbers.contains(x))
            .map(|f| f.to_owned())
            .collect::<Vec<u32>>();
    }

    fn score(&self) -> u32 {
        let _matches = self.matches();
        let mut score = if _matches.len() > 0 { 1 } else { 0 };
        _matches.iter().skip(1).for_each(|_| score *= 2);
        return score;
    }
}

struct Card {
    game: u32,
    winning_numbers: Vec<u32>,
    drawn_numbers: Vec<u32>,
}
