use std::collections::HashMap;

fn count_arrangements(
    joltages: &Vec<i32>,
    from: usize,
    cache: &mut HashMap<usize, usize>,
) -> usize {
    if let Some(result) = cache.get(&from) {
        return *result;
    }

    if joltages.len() - from < 2 {
        cache.insert(from, 1);
        return 1;
    }

    let mut result = 0;

    for i in 1..=3 {
        if let Some(v) = joltages.get(from + i) {
            if joltages[from] >= v - 3 {
                result += count_arrangements(joltages, from + i, cache);
            }
        }
    }

    cache.insert(from, result);

    return result;
}

fn main() {
    let mut joltages: Vec<i32> = include_str!("task10.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    joltages.push(0);
    joltages.push(*joltages.iter().max().unwrap() + 3);
    joltages.sort();

    let mut diffs = HashMap::new();
    for i in 1..joltages.len() {
        let diff = joltages[i] - joltages[i - 1];
        *diffs.entry(diff).or_insert(0) += 1;
    }

    let diff1 = diffs.get(&1).unwrap();
    let diff3 = diffs.get(&3).unwrap();

    // First star
    println!("{}", diff1 * diff3);

    // Second star
    let mut cache = HashMap::new();
    println!("{}", count_arrangements(&joltages, 0, &mut cache));
}
