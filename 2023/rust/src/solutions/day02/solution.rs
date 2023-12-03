use crate::solutions::AoCDay;

pub struct Day02;

impl Day02 {
    fn parse_game(&self, input: &str) -> Game {
        let new_input = input.replace("Game", "");
        let game_sep_index = new_input.find(":").expect("Game separator not found!");
        let game_index = &new_input[0..game_sep_index]
            .trim()
            .parse::<u32>()
            .expect("Could not parse game index");
        let draw = &new_input[game_sep_index + 1..new_input.len()].trim();

        let bag_draw = self.parse_bag_draw(draw);

        return Game {
            index: *game_index,
            draws: bag_draw,
        };
    }

    fn parse_bag_draw(&self, input: &str) -> Vec<CubeSet> {
        let mut bag: Vec<CubeSet> = Vec::new();
        for draw in input.split(";") {
            let mut bag_draw = CubeSet {
                blue: 0,
                green: 0,
                red: 0,
            };
            for split in draw.split(",") {
                let item: Vec<&str> = split.trim().split(" ").collect();

                if item.len() < 2 {
                    panic!("Drawn item is of wrong format!\n{}", draw);
                }

                let count = item[0]
                    .trim()
                    .parse::<u32>()
                    .expect("Could not parse count");
                let color = item[1].trim();

                match color {
                    "blue" => bag_draw.blue = count,
                    "red" => bag_draw.red = count,
                    "green" => bag_draw.green = count,
                    _ => panic!("Invalid color {}", color),
                };
            }
            bag.push(bag_draw);
        }
        return bag;
    }
}

impl AoCDay for Day02 {
    fn part01(&self) -> String {
        let input = self.read_input(2, 1);

        let max_cubes = CubeSet {
            red: 12,
            green: 13,
            blue: 14,
        };

        let mut possible_sum = 0;
        for line in input.lines() {
            let game = self.parse_game(line);

            let has_invalid = game.draws.iter().any(|bag| {
                bag.blue > max_cubes.blue || bag.red > max_cubes.red || bag.green > max_cubes.green
            });
            if !has_invalid {
                possible_sum += game.index;
            }
        }

        return possible_sum.to_string();
    }

    fn part02(&self) -> String {
        let input = self.read_input(2, 2);

        let mut sum = 0;
        for line in input.lines() {
            let game = self.parse_game(line);

            // 1 for multiply
            let mut max_cubeset = CubeSet {
                blue: 1,
                green: 1,
                red: 1,
            };

            for draw in game.draws {
                if draw.blue > max_cubeset.blue {
                    max_cubeset.blue = draw.blue;
                }
                if draw.red > max_cubeset.red {
                    max_cubeset.red = draw.red;
                }
                if draw.green > max_cubeset.green {
                    max_cubeset.green = draw.green;
                }
            }

            sum += max_cubeset.blue * max_cubeset.green * max_cubeset.red;
        }

        return sum.to_string();
    }
}

struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    index: u32,
    draws: Vec<CubeSet>,
}
