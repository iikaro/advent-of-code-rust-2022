use crate::utils::utils::*;

pub fn find_pairs_that_overlap() {
    let file_path = "src/challenges/four_input.txt";
    let contents = read_file(file_path);
    let pairs: Vec<&str> = contents.split("\r\n").collect();

    let mut completely_overlapping_sections = 0;
    let mut overlapping_sections = 0;

    for pair in pairs {
        let sections: Vec<&str> = pair.split(",").collect();
        let first: Vec<i32> = get_start_and_end_section_as_int(sections[0].split("-").collect());
        let second: Vec<i32> = get_start_and_end_section_as_int(sections[1].split("-").collect());

        completely_overlapping_sections += check_complete_overlap(&first, &second) as i32;
        overlapping_sections += check_overlap(&first, &second) as i32;
    }
    println!("A total of {} sections overlap completely, and {}, partially", completely_overlapping_sections, overlapping_sections)
}

fn get_start_and_end_section_as_int(sections: Vec<&str>) -> Vec<i32> {
    let start: i32 = sections[0].parse().unwrap();
    let end: i32 = sections[1].parse().unwrap();
    vec![start, end]
}

fn check_complete_overlap(first: &Vec<i32>, second: &Vec<i32>) -> bool {
    let first_array: Vec<i32> = create_range(first);
    let second_array: Vec<i32> = create_range(second);
    let result_first = second_array.iter().all(|element| first_array.contains(element));
    let result_second = first_array.iter().all(|element| second_array.contains(element));
    
    if result_first == true{
        return true
    } else if result_second == true {
        return true
    } else {
        return false
    }
}

fn check_overlap(first: &Vec<i32>, second: &Vec<i32>) -> bool {
    let first_array: Vec<i32> = create_range(first);
    let second_array: Vec<i32> = create_range(second);
    let result_first = second_array.iter().any(|element| first_array.contains(element));
    let result_second = first_array.iter().any(|element| second_array.contains(element));
    
    if result_first == true{
        return true
    } else if result_second == true {
        return true
    } else {
        return false
    }
}

fn create_range(data:&Vec<i32>)->Vec<i32>{
    if data[0] == data[1]{
        return vec![data[0]]
    }else {
        return (data[0]..data[1]+1).collect();
    }
}
