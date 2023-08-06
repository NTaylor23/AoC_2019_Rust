pub mod days;
pub mod utils;

use std::fs::File;

fn read_input(path: &str) -> File {
    let file = File::open(path).expect("File not found.");
    file
}

fn main() {
    // let day1in: Vec<i32> = utils::read_numbers(&read_input("src/inputs/day01.txt"));
    // let day1a: i32 = days::day01::day01a(&day1in);
    // let day1b: i32 = days::day01::day01b(&day1in);
    // println!("{} {}", day1a, day1b);

    // let day2in = utils::read_csv_numbers("src/inputs/day02.txt");
    // let day2a: i32 = days::day02::day02a(&day2in);
    // let day2b: i32 = days::day02::day02b(&day2in);
    // println!("{} {}", day2a, day2b);

    let day3in = utils::parse_and_split_lines(&read_input("src/inputs/day03.txt"), ',');
    let day3a: i32 = days::day03::day03a(&day3in);
    let day3b: i32 = days::day03::day03b(&day3in);
    println!("{} {}", day3a, day3b);
}
