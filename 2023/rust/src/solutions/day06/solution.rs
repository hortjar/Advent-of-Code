use crate::solutions::AoCDay;

pub struct Day06;

impl Day06 {
    fn parse_line(&self, line: &str) -> Vec<u32> {
        return line
            .replace("Distance:", "")
            .replace("Time:", "")
            .trim()
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
    }
    fn parse_line_part2(&self, line: &str) -> u64 {
        return line
            .replace("Distance:", "")
            .replace("Time:", "")
            .replace(" ", "")
            .trim()
            .parse::<u64>()
            .unwrap();
    }
}

impl AoCDay for Day06 {
    fn part01(&self) -> String {
        let input = self.read_input(6, 1);
        let mut lines = input.lines();

        let times = self.parse_line(&lines.nth(0).expect("No line 0"));
        let distances = self.parse_line(&lines.nth(0).expect("No line 1"));

        let mut mul = 1;
        for i in 0..times.len() {
            let time = times[i];
            let distance = distances[i];
            let mut num_of_valid_options: u32 = 0;
            for j in 1..time {
                if j * (time - j) > distance {
                    num_of_valid_options += 1;
                }
            }
            if num_of_valid_options > 0 {
                mul *= num_of_valid_options;
            }
        }
        return mul.to_string();
    }

    fn part02(&self) -> String {
        let input = self.read_input(6, 1);
        let mut lines = input.lines();

        let time = self.parse_line_part2(&lines.nth(0).expect("No line 0"));
        let distance = self.parse_line_part2(&lines.nth(0).expect("No line 1"));

        let mut mul = 1;
        let mut num_of_valid_options: u64 = 0;
        for j in 1..time {
            if j * (time - j) > distance {
                num_of_valid_options += 1;
            }
        }
        if num_of_valid_options > 0 {
            mul *= num_of_valid_options;
        }
        return mul.to_string();
    }
}
