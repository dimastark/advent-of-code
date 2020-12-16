use itertools::izip;
use num_bigint::{BigInt, ToBigInt};
use num_traits::{One, Zero};
use std::mem::swap;

fn modinv(a: BigInt, m: BigInt) -> BigInt {
    if m == One::one() {
        return One::one();
    }

    let mut a = a;
    let mut m = m.clone();
    let mut x = Zero::zero();
    let mut inv = One::one();

    while a > One::one() {
        inv -= (&a / &m) * &x;
        a = &a % &m;

        swap(&mut a, &mut m);
        swap(&mut x, &mut inv);
    }

    if inv < Zero::zero() {
        inv += m;
    }

    return inv;
}

fn main() {
    // First star
    {
        let mut lines = include_str!("task13.txt").lines();
        let timestamp: u64 = lines
            .next()
            .expect("input shouldn't be empty")
            .parse()
            .expect("first line should be an uint64");
        let ids: Vec<u64> = lines
            .next()
            .expect("input should contains second line")
            .split(",")
            .filter_map(|item| item.parse().ok())
            .collect();
        let (next_bus_id, time_to_wait) = ids
            .iter()
            .map(|id| (id, id - timestamp % id))
            .min_by(|a, b| a.1.cmp(&b.1))
            .expect("answer not found");

        println!("{}", time_to_wait * next_bus_id);
    }

    // Second star
    {
        let ids: Vec<(u64, u64)> = include_str!("task13.txt")
            .lines()
            .skip(1)
            .next()
            .expect("input should contains second line")
            .split(",")
            .enumerate()
            .filter_map(|(i, item)| {
                if item == "x" {
                    None
                } else {
                    Some((i as u64, item.parse().unwrap()))
                }
            })
            .collect();

        // Chinese Remainder Theorem
        let m = ids.iter().fold(1, |m, (_, id)| m * id);
        let ais = ids.iter().map(|(i, id)| id - (i % id));
        let mis = ids.iter().map(|(_, id)| *id);

        let mut result: BigInt = Zero::zero();
        for (ai, mi) in izip!(ais, mis) {
            let mm = (m / mi).to_bigint().unwrap();
            let mi = mi.to_bigint().unwrap();

            result += ai * &mm * modinv(mm, mi);
            result %= m;
        }

        println!("{}", result);
    }
}
