use crate::utils::utils::*;

pub fn compute_sum_of_priority_of_repeated_items() {
    let file_path = "src/challenges/three_input.txt";
    let contents = read_file(file_path);
    let rucksacks: Vec<&str> = contents.split("\r\n").collect();

    let all_my_rucksacks: Vec<String> = rucksacks.iter().map(|&s| s.into()).collect();

    let mut item_priority_sum = 0;
    let mut item_priority_sum_second_part = 0;

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

    let mut my_rucksacks: Vec<String> = vec![String::from(""); 3];

    //Second part
    for i in 0..=300 {
        if i % 3 == 0 && i > 0 {
            my_rucksacks[2].replace_range(.., &all_my_rucksacks[i - 1]);
            my_rucksacks[1].replace_range(.., &all_my_rucksacks[i - 2]);
            my_rucksacks[0].replace_range(.., &all_my_rucksacks[i - 3]);

            let repeated_item = look_for_item_in_three_compartments(
                &my_rucksacks[0],
                &my_rucksacks[1],
                &my_rucksacks[2],
            );

            let item_priority = find_item_priority(repeated_item);
            item_priority_sum_second_part += item_priority;
        }
    }
    println!(
        "The sum of the priority of part two items is: {}",
        item_priority_sum_second_part
    );
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

pub fn look_for_item_in_three_compartments(
    first_compartment: &String,
    second_compartment: &String,
    third_compartment: &String,
) -> char {
    for item in first_compartment.chars() {
        for other_item in second_compartment.chars() {
            for third_item in third_compartment.chars() {
                if item == other_item && item == third_item {
                    let repeated_item = item;
                    return repeated_item;
                }
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
