use crate::solutions::AoCDay;

pub struct Day04;

impl Day04 {}

impl AoCDay for Day04 {
    fn part01(&self) -> String {
        let input = self.read_input(4, 1);

        let sum = input
            .lines()
            .map(|line| {
                let l = line.replace("Game ", "");
                let mut split = l.split(|c| c == ':' || c == '|');

                let game = &split
                    .to_owned()
                    .nth(0)
                    .unwrap_or_default()
                    .parse::<u32>()
                    .unwrap_or_default();

                let winning_nums = &split
                    .nth(1)
                    .unwrap_or_default()
                    .trim()
                    .split(" ")
                    .map(|num| num.trim().parse::<u32>().unwrap_or_default())
                    .collect::<Vec<u32>>();

                let drawn_nums = &split
                    .nth(0)
                    .unwrap_or_default()
                    .trim()
                    .split(" ")
                    .map(|num| num.parse::<u32>().unwrap_or_default())
                    .collect::<Vec<u32>>();

                println!("Winning:");
                winning_nums.iter().for_each(|x| print!("{}, ", x));
                println!("");

                println!("Drawn:");
                drawn_nums.iter().for_each(|x| print!("{}, ", x));
                println!("");

                return Card {
                    game: *game,
                    winning_numbers: winning_nums.to_owned(),
                    drawn_numbers: drawn_nums.to_owned(),
                };
            })
            .map(|card| {
                return card
                    .drawn_numbers
                    .iter()
                    .filter(|x| card.winning_numbers.contains(x))
                    .map(|f| f.to_owned())
                    .collect::<Vec<u32>>();
            })
            .map(|card| {
                let mut score = if card.len() > 0 { 1 } else { 0 };
                card.iter().skip(1).for_each(|_| score *= 2);
                println!("score {} card len {}", score, card.len());
                return score;
            })
            .sum::<u32>();

        return sum.to_string();
    }

    fn part02(&self) -> String {
        let input = self.read_input(4, 2);
        let mut sum = 0;

        return sum.to_string();
    }
}

struct Card {
    game: u32,
    winning_numbers: Vec<u32>,
    drawn_numbers: Vec<u32>,
}
