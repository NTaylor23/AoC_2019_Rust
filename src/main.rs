#![allow(warnings)]

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

    // let day3in = utils::parse_and_split(&read_input("src/inputs/day03.txt"), ',');
    // let day3a: i32 = days::day03::day03a(&day3in);
    // let day3b: i32 = days::day03::day03b(&day3in);
    // println!("{} {}", day3a, day3b);

    // let day4in = "171309-643603";
    // let day4a: i32 = days::day04::day04a(day4in);
    // let day4b: i32 = days::day04::day04b(day4in);
    // println!("{} {}", day4a, day4b);

    // let day6in = utils::read_strings(&read_input("src/inputs/day06.txt"));
    // let day6a = days::day06::day06a(&day6in);
    // let day6b = days::day06::day06b(&day6in);
    // println!("{} {}", day6a, day6b);

    let day8in = utils::read_digits("src/inputs/day08.txt");
    let day8a = days::day08::day08a(&day8in);
    let day8b = days::day08::day08b(&day8in);
    println!("{}\n{}", day8a, day8b);
    
}
