use std::collections::HashSet;

fn find_complement_numbers(input: &[i32], expected_result: &i32) -> Option<(i32, i32)> {
    let mut numbers = HashSet::new();

    for number in input {
        let complement_number = expected_result - number;

        if numbers.contains(&complement_number) {
            return Some((*number, complement_number));
        }

        numbers.insert(number);
    }

    return None;
}

fn main() {
    let expected_result = 2020;

    let input: Vec<i32> = include_str!("task1.txt")
        .split("\n")
        .map(|line| line.parse().expect("input should contains numbers"))
        .collect();

    // First star
    let (number, complement_number) = find_complement_numbers(&input, &expected_result).expect("answer not found");
    println!("{} * {} = {}", number, complement_number, number * complement_number);

    // Second star
    for (i, another_number) in input.iter().enumerate() {
        let expected_result = expected_result - another_number;

        match find_complement_numbers(&input[i..], &expected_result) {
            None => (),
            Some((number, complement_number)) => {
                println!("{} * {} * {} = {}", another_number, number, complement_number, another_number * number * complement_number);
                return;
            }
        }
    }
}
