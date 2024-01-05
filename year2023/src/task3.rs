struct Gear {
    numbers: Vec<i32>,
}

struct Schema {
    adjacent: String,
    number: i32,
}

fn wrap_engine_schematic(field: &mut Vec<Vec<char>>) {
    let line_len = field[0].len() + 2;

    field.insert(0, vec!['.'; line_len]);

    for line in field.iter_mut() {
        line.insert(0, '.');
        line.push('.');
    }

    field.push(vec!['.'; line_len]);
}

fn parse_engine_schematic(data: &str) -> Vec<Schema> {
    let mut field: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    wrap_engine_schematic(&mut field);

    let mut i = 0;
    let mut j = 0;

    let mut engine = vec![];

    while j < field.len() {
        while i < field[j].len() {
            if !field[j][i].is_ascii_digit() {
                i += 1;
                continue;
            }

            let mut end = i;

            while field[j][end].is_ascii_digit() {
                end += 1;
            }

            let prev_j = j.checked_sub(1).unwrap();
            let prev_i = i.checked_sub(1).unwrap();

            let top: String = field[prev_j][prev_i..end + 1].iter().collect();
            let left: String = field[j][prev_i..i].iter().collect();
            let value: String = field[j][i..end].iter().collect();
            let right: String = field[j][end..end + 1].iter().collect();
            let bottom: String = field[j + 1][prev_i..end + 1].iter().collect();

            let adjacent = top + &left + &right + &bottom;
            let number: i32 = i32::from_str_radix(value.as_str(), 10).unwrap();

            engine.push(Schema { adjacent, number });

            i = end;
        }

        j += 1;
        i = 0;
    }

    engine
}

fn parse_line_numbers(line: &Vec<char>, index: usize) -> Vec<i32> {
    let mut start = index - 1;
    let mut end = index + 1;

    while start > 0 && line[start].is_ascii_digit() {
        start -= 1;
    }

    while end < line.len() && line[end].is_ascii_digit() {
        end += 1;
    }

    let mut i = start;
    let mut result = vec![];

    while i < end {
        if !line[i].is_ascii_digit() {
            i += 1;
            continue;
        }

        let mut count = 0;

        while line[i + count].is_ascii_digit() {
            count += 1;
        }

        let value: String = line[i..i + count].iter().collect();
        let number: i32 = i32::from_str_radix(value.as_str(), 10).unwrap();

        result.push(number);

        i += count;
    }

    result
}

fn parse_gears_schematic(data: &str) -> Vec<Gear> {
    let mut field: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    wrap_engine_schematic(&mut field);

    let mut i = 0;
    let mut j = 0;

    let mut gears = vec![];

    while j < field.len() {
        while i < field[j].len() {
            if field[j][i] != '*' {
                i += 1;
                continue;
            }

            let prev_j = j.checked_sub(1).unwrap();
            let next_j = j.checked_add(1).unwrap();

            let mut numbers = vec![];
            
            numbers.extend(parse_line_numbers(&field[prev_j], i));
            numbers.extend(parse_line_numbers(&field[j], i));
            numbers.extend(parse_line_numbers(&field[next_j], i));

            gears.push(Gear { numbers });

            i += 1;
        }

        j += 1;
        i = 0;
    }

    gears
}

fn main() {
    let data = include_str!("task3.txt");

    let first_star: i32 = parse_engine_schematic(data)
        .iter()
        .filter(|schema| {
            schema
                .adjacent
                .chars()
                .any(|c| c != '.' && !c.is_ascii_digit())
        })
        .map(|schema| schema.number)
        .sum();

    println!("{}", first_star);

    let second_star: i32 = parse_gears_schematic(data)
        .iter()
        .filter(|gear| gear.numbers.len() == 2)
        .map(|gear| gear.numbers[0] * gear.numbers[1])
        .sum();

    println!("{}", second_star);
}
