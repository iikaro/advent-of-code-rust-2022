use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;

pub fn find_elf_with_most_calories() {
    print_current_directory();
    let file_path = "src/challenges/one_input.txt";
    let contents = read_file(file_path);
    let elves: Vec<&str> = contents.split("\r\n\r\n").collect();
    let mut maximum_calories: f32 = 0.;
    for elf in elves {
        let calories: Vec<f32> = elf
            .split("\r\n")
            .map(|string| string.parse::<f32>().unwrap())
            .collect();
        let total_calories = calories.iter().sum();
        if total_calories > maximum_calories {
            maximum_calories = total_calories;
            println!("{}", maximum_calories);
        }
    }
}

fn read_file(file_path: &str) -> String {
    let mut file = File::open(file_path).expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    //println!("{}", data);
    return data;
}

fn print_current_directory() -> String {
    let current_directory = current_dir().unwrap().display().to_string();
    println!("{}", current_directory);
    return current_directory;
}
