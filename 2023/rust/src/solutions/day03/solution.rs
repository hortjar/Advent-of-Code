use crate::solutions::AoCDay;

pub struct Day03;

impl Day03 {
    fn get_line(&self, lines: &Vec<&str>, index: usize) -> String {
        return match lines.get(index) {
            None => ".".repeat(140),
            Some(x) => x.to_string(),
        };
    }
    fn is_special_char(&self, input: char) -> bool {
        input != '.' && !input.is_numeric()
    }
    fn parse_num_from_pos(&self, line: &str, pos: usize, special_char: char) -> u32 {
        let sep_pos = line
            .chars()
            .skip(pos)
            .position(|x| x == '.' || x == special_char)
            .unwrap_or(line.len() - pos);
        let substr_l = &line[0..pos - 1]
            .chars()
            .rev()
            .take_while(|x| x.is_numeric())
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();
        let substr_r = &line[pos - 1..pos + sep_pos];
        let mut string_num: String;
        if (substr_r.starts_with('.') || substr_r.starts_with(special_char)) && substr_l != "" {
            string_num = format!("{}{}", "", substr_r);
        } else {
            string_num = format!("{}{}", substr_l, substr_r);
        }
        string_num = string_num.replace(".", "").replace(special_char, "");
        return string_num.parse::<u32>().unwrap_or_default();
    }
    fn get_numbers_on_lines(
        &self,
        a: &str,
        b: &str,
        c: &str,
        index: usize,
        char: char,
    ) -> [[u32; 3]; 3] {
        let chars_a: Vec<char> = a.chars().collect();
        let chars_b: Vec<char> = b.chars().collect();
        let chars_c: Vec<char> = c.chars().collect();

        let diag_left_above = chars_a[index - 1];
        let above = chars_a[index];
        let diag_right_above = chars_a[index + 1];

        let mut numbers = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
        if diag_left_above.is_numeric() {
            numbers[0][0] = self.parse_num_from_pos(a, index - 1, char);
        }
        if above.is_numeric() {
            let new_num = self.parse_num_from_pos(a, index, char);
            if !numbers[0].contains(&new_num) {
                numbers[0][1] = new_num;
            }
        }
        if diag_right_above.is_numeric() {
            let new_num = self.parse_num_from_pos(a, index + 1, char);
            if !numbers[0].contains(&new_num) {
                numbers[0][2] = new_num;
            }
        }

        let left = chars_b[index - 1];
        let right = chars_b[index + 1];

        if left.is_numeric() {
            let new_num = self.parse_num_from_pos(b, index - 1, char);
            if !numbers[1].contains(&new_num) {
                numbers[1][0] = new_num;
            }
        }
        if right.is_numeric() {
            let new_num = self.parse_num_from_pos(b, index + 1, char);
            if !numbers[1].contains(&new_num) {
                numbers[1][1] = new_num;
            }
        }

        let diag_left_below = chars_c[index - 1];
        let below = chars_c[index];
        let diag_right_below = chars_c[index + 1];

        if diag_left_below.is_numeric() {
            numbers[2][0] = self.parse_num_from_pos(c, index - 1, char);
        }
        if below.is_numeric() {
            let new_num = self.parse_num_from_pos(c, index, char);
            if !numbers[2].contains(&new_num) {
                numbers[2][1] = new_num;
            }
        }
        if diag_right_below.is_numeric() {
            let new_num = self.parse_num_from_pos(c, index + 1, char);
            if !numbers[2].contains(&new_num) {
                numbers[2][2] = new_num;
            }
        }

        return numbers;
    }
}

impl AoCDay for Day03 {
    fn part01(&self) -> String {
        let input = self.read_input(3, 1);
        let new_lines: Vec<&str> = input.lines().collect();

        let mut chunk = [
            self.get_line(&new_lines, 0),
            self.get_line(&new_lines, 1),
            self.get_line(&new_lines, 2),
        ];
        let mut sum: u32 = 0;

        for i in 0..new_lines.len() {
            let indices: Vec<(usize, char)> = chunk[1]
                .char_indices()
                .filter(|x| self.is_special_char(x.1))
                .collect();
            for c in indices {
                self.get_numbers_on_lines(&chunk[0], &chunk[1], &chunk[2], c.0, c.1)
                    .iter()
                    .flatten()
                    .filter(|x| **x > 0)
                    .for_each(|x| sum += x);
            }

            // next chunk
            chunk = [
                self.get_line(&new_lines, i + 1),
                self.get_line(&new_lines, i + 2),
                self.get_line(&new_lines, i + 3),
            ];
        }

        return sum.to_string();
    }

    fn part02(&self) -> String {
        let input = self.read_input(3, 2);
        let new_lines: Vec<&str> = input.lines().collect();

        let mut chunk = [
            self.get_line(&new_lines, 0),
            self.get_line(&new_lines, 1),
            self.get_line(&new_lines, 2),
        ];

        let mut sum: u32 = 0;

        for i in 0..new_lines.len() {
            let indices: Vec<(usize, char)> =
                chunk[1].char_indices().filter(|x| x.1 == '*').collect();
            for c in indices {
                let numbers = self.get_numbers_on_lines(&chunk[0], &chunk[1], &chunk[2], c.0, c.1);
                let non_zero = numbers
                    .iter()
                    .flatten()
                    .filter(|x| **x > 0)
                    .collect::<Vec<&u32>>();
                if non_zero.len() == 2 {
                    let mut mul = 1;
                    for part in non_zero {
                        mul *= *part;
                    }
                    sum += mul;
                }
            }

            // next chunk
            chunk = [
                self.get_line(&new_lines, i + 1),
                self.get_line(&new_lines, i + 2),
                self.get_line(&new_lines, i + 3),
            ];
        }

        return sum.to_string();
    }
}
