use regex::Regex;
use std::collections::{HashMap, HashSet};

fn get_nodes_count(
    cache: &mut HashMap<String, i32>,
    graph: &HashMap<&str, HashSet<(&str, i32)>>,
    color: &str,
) -> i32 {
    if let Some(count) = cache.get(&color.to_owned()) {
        return *count;
    }

    let mut total = 0;
    for (other_color, count) in graph.get(color).unwrap_or(&HashSet::new()) {
        total += count;
        total += count * get_nodes_count(cache, graph, other_color);
    }

    cache.insert(color.to_owned(), total);
    return total;
}

fn main() {
    let parent_bag_re = Regex::new(r"^([a-z ]+) bags contain ").unwrap();
    let nested_bag_re = Regex::new(r"(\d) ([a-z ]+) bags?[,.]").unwrap();

    let bags: Vec<_> = include_str!("task7.txt")
        .lines()
        .map(|line| {
            let parent_color = parent_bag_re
                .captures(line)
                .expect("line should starts with `<color> bags contain`")
                .get(1)
                .expect("line should have parent color")
                .as_str();

            let nested_bags: Vec<(i32, _)> = nested_bag_re
                .captures_iter(line)
                .map(|cap| {
                    (
                        cap.get(1).unwrap().as_str().parse().unwrap(),
                        cap.get(2).unwrap().as_str(),
                    )
                })
                .collect();

            (parent_color, nested_bags)
        })
        .collect();

    let mut graph = HashMap::new();
    let mut reversed_graph = HashMap::new();
    for (parent_color, nested_bags) in bags {
        for (count, nested_color) in nested_bags {
            graph
                .entry(parent_color)
                .or_insert(HashSet::new())
                .insert((nested_color, count));

            reversed_graph
                .entry(nested_color)
                .or_insert(HashSet::new())
                .insert(parent_color);
        }
    }

    let mut queue = vec!["shiny gold"];
    let mut visited = HashSet::new();
    while !queue.is_empty() {
        let color = queue.pop().unwrap();

        visited.insert(color);

        for other in reversed_graph.get(color).unwrap_or(&HashSet::new()) {
            if !visited.contains(other) {
                queue.push(other);
            }
        }
    }

    // First star
    println!("{}", visited.len());

    // Second star
    let mut cache = HashMap::new();
    println!("{}", get_nodes_count(&mut cache, &graph, "shiny gold"));
}
