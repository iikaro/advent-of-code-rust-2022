use crate::utils::utils::*;

pub fn compute_sum_of_priority_of_repeated_items() {
    let file_path = "src/challenges/three_input.txt";
    let contents = read_file(file_path);
    let rucksacks: Vec<&str> = contents.split("\r\n").collect();

    let mut item_priority_sum = 0;

    //First part
    for rucksack in rucksacks {
        let total_items = rucksack.len();
        let first = &rucksack[0..total_items / 2].to_owned();
        let second = &rucksack[total_items / 2..].to_owned();
        let repeated_item = look_for_item_in_both_compartments(first, second);
        let item_priority = find_item_priority(repeated_item);
        item_priority_sum += item_priority
    }

    println!(
        "The sum of the priority of the misplaced items is: {}",
        item_priority_sum
    );

    //Second part
    for i in 1..=300 {
        let mut elves_rucksacks: Vec<&str> = ["", "", ""].to_vec();

        if i % 3 == 0 {}
    }
}

pub fn look_for_item_in_both_compartments(
    first_compartment: &String,
    second_compartment: &String,
) -> char {
    for item in first_compartment.chars() {
        for other_item in second_compartment.chars() {
            if item == other_item {
                let repeated_item = other_item;
                return repeated_item;
            }
        }
    }
    let place_holder = '_';
    return place_holder;
}

pub fn find_item_priority(item: char) -> i32 {
    const ASCII_VALUE_UPPER_CASE: i32 = 65;
    const ASCII_VALUE_LOWER_CASE: i32 = 97;
    let mut priority = 0;
    if item.is_lowercase() {
        priority = item as i32 % ASCII_VALUE_LOWER_CASE + 1;
    }

    if item.is_uppercase() {
        priority = item as i32 % ASCII_VALUE_UPPER_CASE + 27;
    }
    return priority;
}
