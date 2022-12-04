use crate::utils::utils::*;

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
        }
    }
    println!("The maximum calories is: {}", maximum_calories);
}

pub fn find_top_three_with_most_calories() {
    print_current_directory();
    let file_path = "src/challenges/one_input.txt";
    let contents = read_file(file_path);
    let elves: Vec<&str> = contents.split("\r\n\r\n").collect();
    let mut maximum_calories: Vec<i32> = vec![0; elves.len()];

    for i in 0..elves.len() {
        let calories: Vec<i32> = elves[i]
            .split("\r\n")
            .map(|string| string.parse::<i32>().unwrap())
            .collect();

        maximum_calories[i] = calories.iter().sum();
    }

    maximum_calories.sort();
    maximum_calories.reverse();

    println!(
        "The sum of the top three is: {}, {} and {}",
        maximum_calories[0], maximum_calories[1], maximum_calories[2]
    );
}
