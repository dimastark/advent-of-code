// First star
// const VISION_DISTANCE: i32 = 1;
// const OCCUPATE_TOLERANCE: i32 = 4;

// Second star
const VISION_DISTANCE: i32 = 1000;
const OCCUPATE_TOLERANCE: i32 = 5;

#[derive(Clone, PartialEq)]
enum Cell {
    Empty,
    Occuped,
    Floor,
}

type State = Vec<Vec<Cell>>;

#[allow(dead_code)]
fn print_state(state: &State) {
    let width = state[0].len();
    let height = state.len();

    for y in 0..height {
        for x in 0..width {
            print!(
                "{}",
                match state[y][x] {
                    Cell::Empty => 'L',
                    Cell::Occuped => '#',
                    Cell::Floor => '.',
                }
            );
        }
        println!();
    }
}

fn calculate_next_state(state: &State) -> State {
    let width = state[0].len();
    let height = state.len();

    let mut next_state = vec![vec![Cell::Floor; width]; height];

    for y in 0..height {
        for x in 0..width {
            if state[y][x] == Cell::Floor {
                continue;
            }

            let mut count = 0;

            for dy in -1..=1_i32 {
                for dx in -1..=1_i32 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let mut m = 1;
                    let mut cy = y as i32 + dy;
                    let mut cx = x as i32 + dx;

                    while 0 <= cy
                        && cy < height as i32
                        && 0 <= cx
                        && cx < width as i32
                        && m <= VISION_DISTANCE
                    {
                        if state[cy as usize][cx as usize] == Cell::Empty {
                            break;
                        }

                        if state[cy as usize][cx as usize] == Cell::Occuped {
                            count += 1;
                            break;
                        }

                        m += 1;
                        cy = y as i32 + m * dy;
                        cx = x as i32 + m * dx;
                    }
                }
            }

            next_state[y][x] = match &state[y][x] {
                Cell::Empty => {
                    if count == 0 {
                        Cell::Occuped
                    } else {
                        Cell::Empty
                    }
                }
                Cell::Occuped => {
                    if count >= OCCUPATE_TOLERANCE {
                        Cell::Empty
                    } else {
                        Cell::Occuped
                    }
                }
                Cell::Floor => Cell::Floor,
            };
        }
    }

    return next_state;
}

fn get_occuped_count(state: &State) -> usize {
    let width = state[0].len();
    let height = state.len();

    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            if state[y][x] == Cell::Occuped {
                count += 1;
            }
        }
    }

    return count;
}

fn main() {
    let mut state = include_str!("task11.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'L' => Cell::Empty,
                    '#' => Cell::Occuped,
                    _ => Cell::Floor,
                })
                .collect()
        })
        .collect();

    let mut occuped_count = get_occuped_count(&state);

    loop {
        state = calculate_next_state(&state);

        let next_state_occuped_count = get_occuped_count(&state);

        if occuped_count == next_state_occuped_count {
            break;
        }

        occuped_count = next_state_occuped_count;
    }

    println!("{}", occuped_count);
}
