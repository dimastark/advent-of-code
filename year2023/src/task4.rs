use std::collections::{HashMap, HashSet};
use std::str::FromStr;

struct Scratchcard {
    id: usize,
    winning_numbers: HashSet<i32>,
    numbers: HashSet<i32>,
}

impl Scratchcard {
    fn win_count(&self) -> u32 {
        self.numbers
            .intersection(&self.winning_numbers)
            .count()
            .try_into()
            .unwrap()
    }

    fn score(&self) -> i32 {
        let win_count = self.win_count();

        if win_count == 0 {
            return 0;
        }

        2_i32.pow(win_count - 1)
    }
}

impl FromStr for Scratchcard {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<i32> = s
            .split_terminator(&[' ', ':', '|'])
            .filter(|s| !s.is_empty())
            .filter(|s| s.chars().all(|c| c.is_ascii_digit()))
            .map(|s| i32::from_str(s).unwrap())
            .collect();

        let id = numbers[0].try_into().unwrap();
        let winning_numbers = HashSet::from_iter(numbers[1..11].iter().cloned());
        let numbers = HashSet::from_iter(numbers[11..].iter().cloned());

        Ok(Scratchcard {
            id,
            winning_numbers,
            numbers,
        })
    }
}

fn main() {
    let cards: Vec<Scratchcard> = include_str!("task4.txt")
        .lines()
        .map(|line| line.parse().expect("invalid card format"))
        .collect();

    let first_star: i32 = cards.iter().map(|card| card.score()).sum();

    println!("{}", first_star);

    let mut cards_count = HashMap::new();

    for card in cards.iter() {
        cards_count.insert(card.id.clone(), 1_u32);
    }

    for card in cards.iter() {
        let win_count: usize = card.win_count().try_into().unwrap();

        for j in 1..=win_count {
            *cards_count.get_mut(&(card.id + j)).unwrap() += cards_count[&card.id];
        }
    }

    let second_star: u32 = cards_count.values().sum();

    println!("{}", second_star);
}
