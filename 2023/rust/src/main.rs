mod solutions;
use solutions::AoCDay;
use std::env;
use std::io;
use std::io::Write;

fn main() {
    let choice: i32;
    let mut input = String::new();

    if env::args().len() > 1 {
        input = match env::args().nth(1) {
            Some(x) => x,
            None => "0".to_string(),
        }
    } else {
        print!("Select day (1-23): ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut input).expect("Read failed!");
    }

    let trimmed = input.trim();
    choice = match trimmed.parse::<i32>() {
        Ok(num) => num,
        Err(..) => 0,
    };

    if choice <= 0 {
        println!("Wrong choice! Use numbers between 1 and 23");
        return;
    }

    match choice {
        1 => solutions::day01::solution::Day01.run(),
        2 => solutions::day02::solution::Day02.run(),
        3 => solutions::day03::solution::Day03.run(),
        _ => println!("Day not yet added!"),
    }
}
