use std::collections::BTreeMap;

use crate::solutions::AoCDay;

pub struct Day01;

impl Day01 {
    fn convert_string_to_stringdigit(&self, digit: &str) -> String {
        match digit {
            "one" => "1".to_string(),
            "two" => "2".to_string(),
            "three" => "3".to_string(),
            "four" => "4".to_string(),
            "five" => "5".to_string(),
            "six" => "6".to_string(),
            "seven" => "7".to_string(),
            "eight" => "8".to_string(),
            "nine" => "9".to_string(),
            _ => digit.to_string(),
        }
    }
}

impl AoCDay for Day01 {
    fn part01(&self) -> String {
        let input = self.read_input(1, 1);

        let mut calibrations: Vec<u32> = Vec::new();
        for line in input.lines() {
            let digits: Vec<char> = line.chars().filter(|x| x.is_numeric()).collect();

            let first = digits.first().expect("No first digit!");
            let last = digits.last().expect("No last digit!");
            let combined = format!("{}{}", first, last);

            let number = combined.parse::<u32>().expect("Could not parse!");

            calibrations.push(number);
        }

        let result: u32 = calibrations.iter().sum();
        result.to_string()
    }

    fn part02(&self) -> String {
        let input = self.read_input(1, 1);

        let valid_digits = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3",
            "4", "5", "6", "7", "8", "9",
        ];
        let mut calibrations: Vec<u32> = Vec::new();
        let mut found: BTreeMap<usize, &str> = BTreeMap::new();
        // let mut i = 1;
        for line in input.lines() {
            found.clear();

            // println!("{};------------------\n{}", i, line);
            for digit in valid_digits {
                let results: Vec<(usize, &str)> = line.match_indices(digit).collect();

                for result in results {
                    found.insert(result.0, result.1);
                }
            }

            // for item in found.iter() {
            //     println!("k: {}; v: {}", item.0, item.1);
            // }

            let first = match found.first_key_value() {
                Some(x) => self.convert_string_to_stringdigit(x.1),
                None => "".to_string(),
            };

            let last = match found.last_key_value() {
                Some(x) => self.convert_string_to_stringdigit(x.1),
                None => "".to_string(),
            };

            let combined = format!("{}{}", first, last);
            let number = combined.parse::<u32>().expect("Could not parse!");
            // println!("{}, {}, ={}", first, last, number);

            calibrations.push(number);
            // i += 1;
        }

        let result: u32 = calibrations.iter().sum();
        result.to_string()
    }
}
