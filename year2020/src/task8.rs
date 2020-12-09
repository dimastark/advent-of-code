use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

enum Op {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

impl Op {
    fn negate(&self) -> Self {
        match self {
            Op::Acc(arg) => Op::Acc(*arg),
            Op::Jmp(arg) => Op::Nop(*arg),
            Op::Nop(arg) => Op::Jmp(*arg),
        }
    }
}

impl FromStr for Op {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arg: i64 = (&s[4..]).parse()?;

        Ok(match &s[..3] {
            "acc" => Op::Acc(arg),
            "jmp" => Op::Jmp(arg),
            "nop" => Op::Nop(arg),
            other => panic!("invalid op {}", other),
        })
    }
}

fn try_execute(ops: &Vec<Op>) -> (bool, i64) {
    let mut ip = 0;
    let mut counter: i64 = 0;
    let mut executed = HashSet::new();

    while !executed.contains(&ip) {
        executed.insert(ip);

        let mut offset = 1;

        match ops[ip] {
            Op::Acc(arg) => counter += arg,
            Op::Jmp(arg) => offset = arg,
            Op::Nop(_) => (),
        }

        offset += ops.len() as i64;
        offset %= ops.len() as i64;

        if (ip + offset as usize) == ops.len() {
            return (true, counter);
        }

        ip = (ip + offset as usize) % ops.len();
    }

    return (false, counter);
}

fn main() {
    let mut ops = include_str!("task8.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    // First star
    println!("{}", try_execute(&ops).1);

    // Second star
    for i in 0..ops.len() {
        if let Op::Jmp(_) | Op::Nop(_) = ops[i] {
            ops[i] = ops[i].negate();

            let (success, counter) = try_execute(&ops);

            if success {
                println!("{}", counter);
                break;
            }

            ops[i] = ops[i].negate();
        }
    }
}
