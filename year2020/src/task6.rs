use std::collections::HashSet;

fn main() {
    let count: usize = include_str!("task6.txt")
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .fold(None, |acc, group| {
                    match acc {
                        None => Some(group),
                        // Some(acc) => Some(acc.union(&group).cloned().collect()) // First star
                        Some(acc) => Some(acc.intersection(&group).cloned().collect()), // Second star
                    }
                })
                .unwrap()
                .len()
        })
        .sum();

    println!("{}", count);
}
