use itertools::all;
use std::cmp::max;
use std::str::FromStr;

struct Sack {
    blue: i32,
    green: i32,
    red: i32,
}

impl Sack {
    fn contains(&self, other: &Sack) -> bool {
        other.blue <= self.blue && other.green <= self.green && other.red <= self.red
    }
}

impl FromStr for Sack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;

        for ball_str in s.split(", ") {
            let separator = ball_str
                .find(" ")
                .ok_or(format!("missing separator: '{}'", ball_str))?;

            let (count_str, color) = ball_str.split_at(separator);

            let count: i32 = count_str
                .parse()
                .ok()
                .ok_or(format!("invalid count: '{}'", count_str))?;

            match &color[1..] {
                "blue" => blue = count,
                "green" => green = count,
                "red" => red = count,
                color => return Err(format!("unknown color: '{}'", color)),
            }
        }

        Ok(Sack { blue, green, red })
    }
}

struct Game {
    id: i32,
    iterations: Vec<Sack>,
}

impl Game {
    fn min_possible_sack(&self) -> Sack {
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;

        for iteration in &self.iterations {
            blue = max(blue, iteration.blue);
            green = max(green, iteration.green);
            red = max(red, iteration.red);
        }

        Sack { blue, green, red }
    }

    fn possible(&self, sack: &Sack) -> bool {
        all(&self.iterations, |other| sack.contains(&other))
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id_from = s.find(" ").ok_or(format!("missing space: '{}'", s))? + 1;
        let id_to = s.find(":").ok_or(format!("missing colon: '{}'", s))?;
        let id: i32 = s[id_from..id_to]
            .parse()
            .ok()
            .ok_or(format!("invalid id: '{}'", &s[id_from..id_to]))?;

        let (_, part2) = s.split_at(id_to + 2);
        let iterations_parts = part2.split("; ");
        let mut iterations = vec![];

        for iteration_str in iterations_parts {
            iterations.push(
                iteration_str
                    .parse()
                    .ok()
                    .ok_or(format!("invalid iteration: '{}'", iteration_str))?,
            );
        }

        Ok(Game { id, iterations })
    }
}

fn main() {
    let sack = &Sack {
        blue: 14,
        green: 13,
        red: 12,
    };

    let games: Vec<Game> = include_str!("task2.txt")
        .lines()
        .map(|line| line.parse::<Game>().expect("invalid input"))
        .collect();

    let first_star: i32 = games
        .iter()
        .filter(|game| game.possible(sack))
        .map(|game| game.id)
        .sum();

    let second_star: i32 = games
        .iter()
        .map(|game| game.min_possible_sack())
        .map(|sack| sack.blue * sack.green * sack.red)
        .sum();

    println!("{}", first_star);
    println!("{}", second_star);
}
