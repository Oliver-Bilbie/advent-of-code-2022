use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};

enum LineValue {
    Number(u32),
    Empty,
}

fn main() {
    task_1();
    task_2();
}

fn task_1() {
    let file = File::open("input.txt").expect("File not found");
    let lines = BufReader::new(file).lines();

    let mut current_sum = 0 as u32;
    let mut largest_sum = 0 as u32;

    for (line_number, line_content) in lines.enumerate() {
        let line_value = read_line(line_number, line_content);
        match line_value {
            LineValue::Number(number) => {
                current_sum += number;
            }
            LineValue::Empty => {
                if current_sum > largest_sum {
                    largest_sum = current_sum;
                }
                current_sum = 0
            }
        }
    }
    if current_sum > largest_sum {
        largest_sum = current_sum;
    }

    println!(
        "The elf carrying the most calories is carrying {} calories.",
        largest_sum
    );
}

fn task_2() {
    let file = File::open("input.txt").expect("File not found");
    let lines = BufReader::new(file).lines();

    let mut current_sum = 0 as u32;
    let mut largest_sums: [u32; 3] = [0, 0, 0];

    for (line_number, line_content) in lines.enumerate() {
        let line_value = read_line(line_number, line_content);
        match line_value {
            LineValue::Number(number) => {
                current_sum += number;
            }
            LineValue::Empty => {
                if current_sum > largest_sums[0] {
                    largest_sums[0] = current_sum;
                    largest_sums.sort();
                }
                current_sum = 0
            }
        }
    }
    if current_sum > largest_sums[0] {
        largest_sums[0] = current_sum;
        largest_sums.sort();
    }

    println!(
        "The three elves carrying the most calories are carrying {}, {}, and {} calories.",
        largest_sums[2], largest_sums[1], largest_sums[0]
    );
    println!(
        "That gives us a total of {} calories.",
        largest_sums.iter().sum::<u32>()
    );
}

fn read_line(line_number: usize, line_content: Result<String, Error>) -> LineValue {
    match line_content {
        Ok(content) => {
            if content.is_empty() {
                LineValue::Empty
            } else {
                let number = content.parse::<u32>();
                match number {
                    Ok(number) => LineValue::Number(number),
                    Err(_) => panic!("The value on line {} is not an integer", line_number),
                }
            }
        }
        Err(e) => panic!("Error reading line {}: {:?}", line_number, e),
    }
}
