pub mod day01;
use std::fs;

pub trait AoCDay {
    fn part01(&self) -> String;
    fn part02(&self) -> String;

    fn run(&self) {
        let sol01 = self.part01();
        println!("{}", sol01);

        let sol02 = self.part02();
        println!("{}", sol02);
    }

    fn read_input(&self, day: i8, part: i8) -> String {
        let daystr = format!("{:02}", day);
        let partstr = format!("{:02}", part);
        let file_path = format!("src/solutions/day{}/input/part{}.txt", daystr, partstr);

        let content = fs::read_to_string(file_path).expect("Cannot read file");

        return content;
    }
}
