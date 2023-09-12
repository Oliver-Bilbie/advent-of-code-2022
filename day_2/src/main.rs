use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};

struct LineValues {
    opponent_char: char,
    my_char: char,
}

fn read_line(line_number: usize, line_content: Result<String, Error>) -> LineValues {
    match line_content {
        Ok(content) => LineValues {
            opponent_char: content.chars().nth(0).unwrap(),
            my_char: content.chars().nth(2).unwrap(),
        },
        Err(e) => panic!("Error reading line {}: {:?}", line_number, e),
    }
}

enum GameOptions {
    Rock,
    Paper,
    Scissors,
}

impl GameOptions {
    fn from_opponent_char(input_character: char) -> GameOptions {
        match input_character {
            'A' => GameOptions::Rock,
            'B' => GameOptions::Paper,
            'C' => GameOptions::Scissors,
            _ => panic!("Invalid opponent input character: {}", input_character),
        }
    }

    fn from_my_char(input_character: char) -> GameOptions {
        match input_character {
            'X' => GameOptions::Rock,
            'Y' => GameOptions::Paper,
            'Z' => GameOptions::Scissors,
            _ => panic!("Invalid input character: {}", input_character),
        }
    }

    fn from_my_outcome(opponent_choice: GameOptions, outcome: GameOutcomes) -> GameOptions {
        match opponent_choice {
            GameOptions::Rock => match outcome {
                GameOutcomes::Win => GameOptions::Paper,
                GameOutcomes::Draw => GameOptions::Rock,
                GameOutcomes::Lose => GameOptions::Scissors,
            },
            GameOptions::Paper => match outcome {
                GameOutcomes::Win => GameOptions::Scissors,
                GameOutcomes::Draw => GameOptions::Paper,
                GameOutcomes::Lose => GameOptions::Rock,
            },
            GameOptions::Scissors => match outcome {
                GameOutcomes::Win => GameOptions::Rock,
                GameOutcomes::Draw => GameOptions::Scissors,
                GameOutcomes::Lose => GameOptions::Paper,
            },
        }
    }

    fn get_points(&self) -> u32 {
        match self {
            GameOptions::Rock => 1,
            GameOptions::Paper => 2,
            GameOptions::Scissors => 3,
        }
    }
}

enum GameOutcomes {
    Win,
    Draw,
    Lose,
}

impl GameOutcomes {
    fn from_my_char(input_character: char) -> GameOutcomes {
        match input_character {
            'X' => GameOutcomes::Lose,
            'Y' => GameOutcomes::Draw,
            'Z' => GameOutcomes::Win,
            _ => panic!("Invalid input character: {}", input_character),
        }
    }

    fn get_points(&self) -> u32 {
        match self {
            GameOutcomes::Win => 6,
            GameOutcomes::Draw => 3,
            GameOutcomes::Lose => 0,
        }
    }
}

struct RoundState {
    opponent_choice: GameOptions,
    my_choice: GameOptions,
}

impl RoundState {
    fn get_my_outcome(&self) -> GameOutcomes {
        match self.my_choice {
            GameOptions::Rock => match self.opponent_choice {
                GameOptions::Rock => GameOutcomes::Draw,
                GameOptions::Paper => GameOutcomes::Lose,
                GameOptions::Scissors => GameOutcomes::Win,
            },
            GameOptions::Paper => match self.opponent_choice {
                GameOptions::Rock => GameOutcomes::Win,
                GameOptions::Paper => GameOutcomes::Draw,
                GameOptions::Scissors => GameOutcomes::Lose,
            },
            GameOptions::Scissors => match self.opponent_choice {
                GameOptions::Rock => GameOutcomes::Lose,
                GameOptions::Paper => GameOutcomes::Win,
                GameOptions::Scissors => GameOutcomes::Draw,
            },
        }
    }

    fn get_my_score(&self) -> u32 {
        self.get_my_outcome().get_points() + self.my_choice.get_points()
    }
}

fn task_1() {
    let file = File::open("input.txt").expect("File not found");
    let lines = BufReader::new(file).lines();

    let mut my_total_score = 0 as u32;

    for (line_number, line_content) in lines.enumerate() {
        let current_line_values = read_line(line_number, line_content);
        let current_round_state = RoundState {
            opponent_choice: GameOptions::from_opponent_char(current_line_values.opponent_char),
            my_choice: GameOptions::from_my_char(current_line_values.my_char),
        };
        my_total_score += current_round_state.get_my_score();
    }

    println!("My total score is {}.", my_total_score);
}

fn task_2() {
    let file = File::open("input.txt").expect("File not found");
    let lines = BufReader::new(file).lines();

    let mut my_total_score = 0 as u32;

    for (line_number, line_content) in lines.enumerate() {
        let current_line_values = read_line(line_number, line_content);
        let current_round_state = RoundState {
            opponent_choice: GameOptions::from_opponent_char(current_line_values.opponent_char),
            my_choice: GameOptions::from_my_outcome(
                GameOptions::from_opponent_char(current_line_values.opponent_char),
                GameOutcomes::from_my_char(current_line_values.my_char),
            ),
        };
        my_total_score += current_round_state.get_my_score();
    }

    println!("My total score is {}.", my_total_score);
}

fn main() {
    task_1();
    task_2();
}
