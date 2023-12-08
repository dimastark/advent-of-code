use onig::*;
use std::collections::HashMap;

fn find_number(s: &str, re: &Regex) -> i32 {
    let num_map: HashMap<&str, i32> = HashMap::from([
        ("1", 1), ("o", 1),
        ("2", 2), ("tw", 2),
        ("3", 3), ("th", 3),
        ("4", 4), ("fo", 4),
        ("5", 5), ("fi", 5),
        ("6", 6), ("si", 6),
        ("7", 7), ("se", 7),
        ("8", 8), ("e", 8),
        ("9", 9), ("n", 9),
    ]);

    let captures: Vec<&str> = re
        .captures_iter(s)
        .map(|capture| capture.at(1).unwrap())
        .collect();

    let first = num_map[captures.first().unwrap()];
    let last = num_map[captures.last().unwrap()];

    10 * first + last
}

fn main() {
    let char_regex = Regex::new(r"([1-9])").unwrap();
    let word_regex = Regex::new(r"([1-9]|o(?=ne)|tw(?=o)|th(?=ree)|fo(?=ur)|fi(?=ve)|si(?=x)|se(?=ven)|e(?=ight)|n(?=ine))").unwrap();

    assert!(find_number("1abc2", &char_regex) == 12);
    assert!(find_number("pqr3stu8vwx", &char_regex) == 38);
    assert!(find_number("a1b2c3d4e5f", &char_regex) == 15);
    assert!(find_number("treb7uchet", &char_regex) == 77);

    let first_star: i32 = include_str!("task1.txt")
        .lines()
        .map(|line| find_number(line, &char_regex))
        .sum();

    println!("{}", first_star);

    assert!(find_number("two1nine", &word_regex) == 29);
    assert!(find_number("eightwothree", &word_regex) == 83);
    assert!(find_number("abcone2threexyz", &word_regex) == 13); 
    assert!(find_number("xtwone3four", &word_regex) == 24);
    assert!(find_number("4nineeightseven2", &word_regex) == 42);
    assert!(find_number("zoneight234", &word_regex) == 14);
    assert!(find_number("7pqrstsixteen", &word_regex) == 76);
    assert!(find_number("8nine37bpkmtghhnc2hnreightwohvs", &word_regex) == 82);

    let second_star: i32 = include_str!("task1.txt")
        .lines()
        .map(|line| find_number(line, &word_regex))
        .sum();

    println!("{}", second_star);
}
