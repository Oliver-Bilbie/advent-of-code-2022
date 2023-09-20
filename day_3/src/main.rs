use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};

struct RucksackContents {
    first_compartment: String,
    second_compartment: String,
}

impl RucksackContents {
    fn find_duplicate_item(self) -> char {
        for item in self.first_compartment.chars() {
            if self.second_compartment.contains(item) {
                return item;
            }
        }
        panic!("No duplicate item found in rucksack");
    }
}

fn read_line_to_rucksack(
    line_number: usize,
    line_content: Result<String, Error>,
) -> RucksackContents {
    match line_content {
        Ok(content) => {
            let items_per_compartment = content.len() / 2;
            RucksackContents {
                first_compartment: content.chars().take(items_per_compartment).collect(),
                second_compartment: content.chars().skip(items_per_compartment).collect(),
            }
        }
        Err(e) => panic!("Error reading line {}: {:?}", line_number, e),
    }
}

fn read_line_to_string(line_number: usize, line_content: Result<String, Error>) -> String {
    match line_content {
        Ok(content) => content.to_string(),
        Err(e) => panic!("Error reading line {}: {:?}", line_number, e),
    }
}

fn find_duplicate_in_group(group_items: &[String; 3]) -> char {
    for item in group_items[0].chars() {
        if group_items[1].contains(item) {
            if group_items[2].contains(item) {
                return item;
            }
        }
    }
    panic!("No duplicate item found in group");
}

fn calculate_priority(item: u8) -> u8 {
    // Convert a character (as a u8) to a priority value where a-z = 1-26 and A-Z = 27-52
    if item >= 97 && item <= 122 {
        item - 96
    } else if item >= 65 && item <= 90 {
        item - 38
    } else {
        panic!("Unexpected item in bagging area: {}", item)
    }
}

fn task_1() {
    let file = File::open("input.txt").expect("File not found");
    let lines = BufReader::new(file).lines();

    let mut total_priority = 0 as u32;

    for (line_number, line_content) in lines.enumerate() {
        let current_sack_contents = read_line_to_rucksack(line_number, line_content);
        let duplicate_item = current_sack_contents.find_duplicate_item();
        let priority = calculate_priority(duplicate_item as u8);
        total_priority += priority as u32;
    }

    println!("Total priority: {}", total_priority);
}

fn task_2() {
    let file = File::open("input.txt").expect("File not found");
    let lines = BufReader::new(file).lines();

    let mut total_priority = 0 as u32;
    let mut group_member_number: usize;
    let mut group_sack_contents: [String; 3] = [String::new(), String::new(), String::new()];

    for (line_number, line_content) in lines.into_iter().enumerate() {
        group_member_number = (line_number + 1) % 3;
        group_sack_contents[group_member_number as usize] =
            read_line_to_string(line_number, line_content);
        if group_member_number == 0 {
            let duplicate_item = find_duplicate_in_group(&group_sack_contents);
            let priority = calculate_priority(duplicate_item as u8);
            total_priority += priority as u32;
        }
    }

    println!("Total priority: {}", total_priority);
}

fn main() {
    task_1();
    task_2();
}
