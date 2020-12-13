#[derive(Clone, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

static DIRECTIONS_LOOP: &[Direction] = &[
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

fn simple_rotate(dir: Direction, deg: i32) -> Direction {
    let current_index = DIRECTIONS_LOOP.iter().position(|r| r == &dir).unwrap();
    let next_index = (current_index as i32 + deg / 90 + DIRECTIONS_LOOP.len() as i32) as usize;
    return DIRECTIONS_LOOP[next_index % DIRECTIONS_LOOP.len()].clone();
}

fn rotate(x: &mut i32, y: &mut i32, deg: i32) {
    let rx = *x;
    let ry = *y;

    match deg {
        -90 | 270 => {
            *x = ry;
            *y = -rx;
        }
        -180 | 180 => {
            *x = -rx;
            *y = -ry;
        }
        -270 | 90 => {
            *x = -ry;
            *y = rx;
        }
        _ => {}
    }
}

fn parse_move(line: &str) -> (char, i32) {
    (
        line.chars().nth(0).unwrap(),
        line[1..].parse::<i32>().unwrap(),
    )
}

fn main() {
    let moves: Vec<(char, i32)> = include_str!("task12.txt")
        .lines()
        .map(|line| parse_move(line))
        .collect();

    // First star
    let mut x = 0;
    let mut y = 0;
    let mut d = Direction::East;
    for (c, n) in &moves {
        match c {
            'N' => y += n,
            'E' => x += n,
            'S' => y -= n,
            'W' => x -= n,
            'L' => d = simple_rotate(d, -n),
            'R' => d = simple_rotate(d, *n),
            'F' => match d {
                Direction::North => y += n,
                Direction::East => x += n,
                Direction::South => y -= n,
                Direction::West => x -= n,
            },
            _ => {}
        }
    }

    println!("{}", x.abs() + y.abs());

    // Second star
    let mut x = 0;
    let mut y = 0;
    let mut wx = 10;
    let mut wy = 1;

    for (c, n) in moves {
        match c {
            'N' => wy += n,
            'E' => wx += n,
            'S' => wy -= n,
            'W' => wx -= n,
            'L' => rotate(&mut wx, &mut wy, n),
            'R' => rotate(&mut wx, &mut wy, -n),
            'F' => {
                x += n * wx;
                y += n * wy;
            }
            _ => {}
        }
    }

    println!("{}", x.abs() + y.abs());
}
