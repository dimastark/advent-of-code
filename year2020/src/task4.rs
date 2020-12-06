use regex::Regex;
use std::collections::HashMap;

fn rng_validator(from: i32, to: i32) -> impl Fn(&str) -> bool {
    move |s| rng_validate(from, to, s)
}

fn reg_validator(pattern: &str) -> impl Fn(&str) -> bool {
    let reg = Regex::new(pattern).unwrap();
    move |s| reg.is_match(s)
}

fn rng_validate(from: i32, to: i32, s: &str) -> bool {
    let d: i32 = s.parse().unwrap_or_default();
    from <= d && d <= to
}

fn hgt_validate(s: &str) -> bool {
    s.ends_with("cm") && rng_validate(150, 193, &s[..s.len() - 2])
        || s.ends_with("in") && rng_validate(59, 76, &s[..s.len() - 2])
}

fn main() {
    let records = include_str!("task4.txt").split("\n\n").map(|data| {
        data.split(char::is_whitespace)
            .map(|item| item.split_at(item.find(":").unwrap()))
            .map(|(key, value)| (key, &value[1..]))
            .collect::<HashMap<&str, &str>>()
    });

    let required_fields = vec![
        "byr", // (Birth Year)
        "iyr", // (Issue Year)
        "eyr", // (Expiration Year)
        "hgt", // (Height)
        "hcl", // (Hair Color)
        "ecl", // (Eye Color)
        "pid", // (Passport ID)
    ];

    let field_validators = {
        let mut h: HashMap<&str, Box<dyn Fn(&str) -> bool>> = HashMap::new();
        h.insert("byr", Box::new(rng_validator(1920, 2002)));
        h.insert("iyr", Box::new(rng_validator(2010, 2020)));
        h.insert("eyr", Box::new(rng_validator(2020, 2030)));
        h.insert("hgt", Box::new(hgt_validate));
        h.insert("hcl", Box::new(reg_validator(r"^#[0-9a-f]{6}$")));
        h.insert(
            "ecl",
            Box::new(reg_validator(r"^(amb|blu|brn|gry|grn|hzl|oth)$")),
        );
        h.insert("pid", Box::new(reg_validator(r"^\d{9}$")));
        h
    };

    let mut filled_records = 0;
    let mut valid_records = 0;
    'outer: for record in records {
        for required_field in &required_fields {
            if !record.contains_key(required_field) {
                continue 'outer;
            }
        }

        filled_records += 1;

        for (key, value) in record {
            if field_validators.contains_key(key) && !field_validators[key](value) {
                continue 'outer;
            }
        }

        valid_records += 1;
    }

    // First star
    println!("{}", filled_records);

    // Second star
    println!("{}", valid_records);
}
