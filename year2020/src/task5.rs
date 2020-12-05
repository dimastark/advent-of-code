use std::collections::HashSet;

fn main() {
    let seat_ids: HashSet<_> = include_str!("task5.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'F' => "0",
                    'L' => "0",
                    'B' => "1",
                    'R' => "1",
                    _ => "",
                })
                .collect::<String>()
        })
        .map(|bin| isize::from_str_radix(&bin, 2).unwrap())
        .collect();

    let min = *seat_ids.iter().min().unwrap();
    let max = *seat_ids.iter().max().unwrap();

    // First star
    println!("{}", max);

    // Second star
    for i in min..max {
        if !seat_ids.contains(&i) && seat_ids.contains(&(i - 1)) && seat_ids.contains(&(i + 1)) {
            println!("{}", i);
            return;
        }
    }
}
