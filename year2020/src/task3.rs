fn get_trees_count(m: &Vec<Vec<bool>>, dx: usize, dy: usize) -> i32 {
    let x_size = m[0].len();
    let y_size = m.len();

    let mut trees_count = 0;

    for y in (0..y_size).step_by(dy) {
        let x = (dx * y / dy) % x_size;

        if m[y][x] {
            trees_count += 1;
        }
    }

    return trees_count;
}

fn main() {
    let tree_matrix: Vec<Vec<bool>> = include_str!("task3.txt")
        .split("\n")
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    // First star
    println!("{}", get_trees_count(&tree_matrix, 3, 1));

    // Second star
    let mut result: usize = 1;
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    for (dx, dy) in slopes {
        result *= get_trees_count(&tree_matrix, dx, dy) as usize;
    }

    println!("{}", result);
}
