use std::fs::File;

use std::io::{BufReader, BufRead};

fn main() {
    let lines = read_input();

    let nums: Vec<i32> = lines
        .iter()
        .map(|line: &String| {
            match line.parse::<i32>() {
                Err(line) => panic!("Couldn't parse line '{}' to i32", line),
                Ok(parsed_line) => {
                    parsed_line
                }
            }
        }).collect::<Vec<_>>();


    println!("sum: {}", nums.iter().sum::<i32>());

    find_first_duplicate(&nums);
}

fn find_first_duplicate(nums : &Vec<i32>) {
    let mut freqs: Vec<i32> = Vec::new();
    freqs.push(0);
    let mut matched = false;
    let mut total = 0;

    while !matched {
        for line in nums {
            total = total + line;

            match freqs.contains(&total) {
                true => {
                    println!("First duplicate frequency is '{}'!", total);
                    matched = true;
                    break;
                },
                false => {
                    freqs.push(total);
                }
            }


        }
    }
    ()
}

fn read_input() -> Vec<String> {
    let input: File = match File::open("./input.txt") {
        Err(e) => panic!("Error opening file 'input.txt': {}", e),
        Ok(f) => f,
    };

    let buf: BufReader<File> = BufReader::new(input);

    let mut lines: Vec<String> = Vec::new();

    for line in buf.lines() {
        lines.push(line.unwrap());
    }

    lines
}