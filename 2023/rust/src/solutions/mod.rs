pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
use std::fs;

pub trait AoCDay {
    fn part01(&self) -> String;
    fn part02(&self) -> String;

    fn run(&self) {
        use std::time::Instant;

        let now1 = Instant::now();
        let sol01 = self.part01();
        let elapsed1 = now1.elapsed();

        println!("{}", sol01);

        let now2 = Instant::now();
        let sol02 = self.part02();
        let elapsed2 = now2.elapsed();

        println!("{}", sol02);

        println!("Time to solve part1: {:.2?}", elapsed1);
        println!("Time to solve part2: {:.2?}", elapsed2);
        println!("Time to solve all:   {:.2?}", elapsed1 + elapsed2);
    }

    fn read_input(&self, day: i8, part: i8) -> String {
        let daystr = format!("{:02}", day);
        let partstr = format!("{:02}", part);
        let file_path = format!("src/solutions/day{}/input/part{}.txt", daystr, partstr);

        let content = fs::read_to_string(file_path).expect("Cannot read file");

        return content;
    }
}
