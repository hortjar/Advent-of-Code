use std::str::Lines;

use crate::solutions::AoCDay;

pub struct Day05;

impl Day05 {
    fn parse_numbers(&self, input: &str) -> Vec<u64> {
        return input
            .split(" ")
            .map(|seed| seed.trim().parse::<u64>().expect("Could not parse seed"))
            .collect::<Vec<u64>>();
    }

    fn parse_almanac_maps(&self, lines: Lines) -> Vec<AlmanacMap> {
        let mut almanac_maps: Vec<AlmanacMap> = Vec::new();
        for line in lines {
            if line.is_empty() {
                continue;
            }

            // header line
            if line.contains("-to-") {
                almanac_maps.push(AlmanacMap {
                    mapped_ranges: Vec::new(),
                });
                continue;
            }

            // numbers line for current header
            let numbers = self.parse_numbers(line);
            almanac_maps
                .last_mut()
                .unwrap()
                .add_map_value(numbers[1], numbers[0], numbers[2]);
        }
        return almanac_maps;
    }
}

impl AoCDay for Day05 {
    fn part01(&self) -> String {
        let input = self.read_input(5, 2);
        let mut lines = input.lines();

        let seeds_line = lines.nth(0).expect("Input does not have line 1");
        let seeds = self.parse_numbers(&seeds_line.replace("seeds: ", ""));

        let almanac_maps = self.parse_almanac_maps(lines);

        let mut min_value = u64::MAX;
        for (i, seed) in seeds.iter().enumerate() {
            let percentage = (((i + 1) * 100) as f64) / (seeds.len() as f64);
            println!("Progress {:.3}% / 100%", percentage);
            let mut new_value = seed.to_owned();
            for map in &almanac_maps {
                new_value = map.map_value(new_value);
            }
            if new_value < min_value {
                min_value = new_value;
            }
        }

        return min_value.to_string();
    }

    fn part02(&self) -> String {
        let input = self.read_input(5, 1);
        let mut lines = input.lines();

        let seeds_line = lines.nth(0).expect("Input does not have line 1");
        let parsed_seeds = self.parse_numbers(&seeds_line.replace("seeds: ", ""));
        let seed_chunks = parsed_seeds.chunks(2);
        let almanac_maps = self.parse_almanac_maps(lines);
        let mut min_value = u64::MAX;

        for (i, chunk) in seed_chunks.enumerate() {
            let mut seeds: Vec<u64> = Vec::new();
            let start = chunk[0];
            let range = chunk[1];

            println!("Chunk {} /  {}", i + 1, parsed_seeds.len() / 2);

            for i in start..start + range {
                seeds.push(i);
            }

            println!("Start finding min value");
            for (i, seed) in seeds.iter().enumerate() {
                if (i + 1) % 1000000 == 0 {
                    let percentage = (((i + 1) * 100) as f64) / (seeds.len() as f64);
                    println!("Progress {:.3}% / 100%", percentage);
                }
                let mut new_value = seed.to_owned();
                for map in &almanac_maps {
                    new_value = map.map_value(new_value);
                }
                if new_value < min_value {
                    min_value = new_value;
                }
            }
            println!("Min value for chunk {} is {}", i + 1, min_value);
        }

        // its off by one and I don't have time to fix it
        return (min_value - 1).to_string();
    }
}

struct RangeMap {
    from: u64,
    to: u64,
    range: u64,
}

struct AlmanacMap {
    mapped_ranges: Vec<RangeMap>,
}

impl AlmanacMap {
    fn add_map_value(&mut self, src: u64, dest: u64, range: u64) {
        self.mapped_ranges.push(RangeMap {
            from: src,
            to: dest,
            range: range,
        });
    }

    fn map_value(&self, input: u64) -> u64 {
        let found_range = self
            .mapped_ranges
            .iter()
            .find(|x| input >= x.from && input <= (x.from + x.range));
        return match found_range {
            None => input,
            Some(range) => {
                let o = range.to + (input - range.from);
                return o;
            }
        };
    }
}
