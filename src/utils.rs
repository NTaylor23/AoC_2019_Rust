use std::fs::{read_to_string, File};
use std::io::BufRead;
use std::io::BufReader;

pub fn parse_and_split(file: &File, sep: char) -> Vec<Vec<String>> {
    let reader = BufReader::new(file);    
    let mut content: Vec<Vec<String>> = Vec::new();
    for ln in reader.lines() {
        match ln {
            Ok(s) => content.push(s.split(sep).map(str::to_string).collect()),
            Err(e) => println!("Could not read string: {}", e),
        }
    }
    content
}

pub fn read_strings(file: &File) -> Vec<String> {
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|ln| ln.unwrap())
        .collect()
}

pub fn read_numbers(file: &File) -> Vec<i32> {
    let reader = BufReader::new(file);

    let numbers: Vec<i32> = reader
        .lines()
        .map(|ln| ln.unwrap().parse::<i32>().unwrap())
        .collect();

    numbers
}

pub fn read_digits(path: &str) -> Vec<u32> {
    read_to_string(path)
        .unwrap()
        .chars()
        .map(|c| c as u32 - '0' as u32)
        .collect()
}

pub fn read_csv_numbers(path: &str) -> Vec<i32> {
    let numbers: Vec<i32> = read_to_string(path)
        .expect("File not found.")
        .split(',')
        .map(|el| el.parse::<i32>().unwrap())
        .collect();

    numbers
}

pub fn intcode_compiler(input: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut program = input.clone();
    program[1] = noun;
    program[2] = verb;

    for i in (0..program.len()).step_by(4) {
        let opcode: i32 = program[i];
        let idx1: usize = program[i + 1] as usize;
        let idx2: usize = program[i + 2] as usize;
        let loc: usize = program[i + 3] as usize;

        match opcode {
            1 => program[loc] = program[idx1] + program[idx2],
            2 => program[loc] = program[idx1] * program[idx2],
            99 => break,
            _ => {
                println!("Something went wrong.");
                break;
            }
        }
    }

    program[0]
}
