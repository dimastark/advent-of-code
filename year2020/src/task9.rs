use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Sub;

const WINDOW_SIZE: usize = 25;

fn find_complement_numbers<T>(input: &[T], expected_result: T) -> Option<(T, T)>
where
    T: Copy + Eq + Hash + Sub<T, Output = T>,
{
    let mut numbers = HashSet::new();

    for number in input {
        let complement_number = expected_result - *number;

        if numbers.contains(&complement_number) {
            return Some((*number, complement_number));
        }

        numbers.insert(number);
    }

    return None;
}

fn main() {
    let input: Vec<i64> = include_str!("task9.txt")
        .lines()
        .map(|line| line.parse().expect("input should contains numbers"))
        .collect();

    // First star
    let mut weakness = 0;
    for (i, number) in input[WINDOW_SIZE..].iter().enumerate() {
        if find_complement_numbers(&input[i..i + WINDOW_SIZE], *number).is_none() {
            weakness = *number;
            break;
        }
    }

    println!("{}", weakness);

    // Second star
    let mut l = 0;
    let mut r = 0;
    let mut sum = 0;
    while sum != weakness {
        if r - l < 2 || sum < weakness {
            sum += input[r];
            r += 1;
            continue;
        }

        if sum > weakness {
            sum -= input[l];
            l += 1;
            continue;
        }
    }

    println!(
        "{}",
        (&input[l..r]).iter().min().unwrap() + (&input[l..r]).iter().max().unwrap()
    );
}
