use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;

pub fn read_file(file_path: &str) -> String {
    let mut file = File::open(file_path).expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    return data;
}

pub fn print_current_directory() -> String {
    let current_directory = current_dir().unwrap().display().to_string();
    println!("{}", current_directory);
    return current_directory;
}

pub fn resolve_rock_paper_scissors(opponent: &str, response: &str) -> i32 {
    match opponent {
        //Rock
        "A" => match response {
            "X" => 1 + 3, //Rock
            "Y" => 2 + 6, //Paper
            "Z" => 3 + 0, //Scissors
            _ => 0,
        },
        //Paper
        "B" => match response {
            "X" => 1 + 0, //Rock
            "Y" => 2 + 3, //Paper
            "Z" => 3 + 6, //Scissors
            _ => 0,
        },
        //Scissors
        "C" => match response {
            "X" => 1 + 6, //Rock
            "Y" => 2 + 0, //Paper
            "Z" => 3 + 3, //Scissors
            _ => 0,
        },
        _ => 0,
    }
}

pub fn resolve_rock_paper_scissors_alternative(opponent: &str, response: &str) -> i32 {
    match opponent {
        //Rock
        "A" => match response {
            "X" => 3 + 0, //Paper, lose
            "Y" => 1 + 3, //Rock, draw
            "Z" => 2 + 6, //Scissors, win
            _ => 0,
        },
        //Paper
        "B" => match response {
            "X" => 1 + 0, //Rock, lose
            "Y" => 2 + 3, //Paper, draw
            "Z" => 3 + 6, //Scissors, win
            _ => 0,
        },
        //Scissors
        "C" => match response {
            "X" => 2 + 0, //Paper, lose
            "Y" => 3 + 3, //Scissors, draw
            "Z" => 1 + 6, //Rock, win
            _ => 0,
        },
        _ => 0,
    }
}
