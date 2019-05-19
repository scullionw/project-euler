#![feature(test)]

use benchtest::benchtest;

const PROBLEM_INPUT: u64 = 1000;

pub fn solve_classic(n: u64) -> u64 {
    // (n - a - b)^2 = a^2 + b^2
    // n^2 = 2na + 2nb - 2ab
    let n_squared = n.pow(2);
    let n_doubled = 2 * n;
    let n_halved = n / 2;

    'net: loop {
        for b in 1..=n_halved {
            for a in 1..b {
                if n_doubled * (a + b) - (2 * a * b) == n_squared {
                    break 'net a * b * (1000 - a - b);
                }
            }
        }
        unreachable!();
    }
}

pub fn solve_fp(n: u64) -> u64 {
    let n_squared = n.pow(2);
    let n_doubled = 2 * n;
    let n_halved = n / 2;

    let (b, a) = (1..=n_halved)
        .flat_map(|b| (1..=b).map(move |a| (b, a)))
        .find(|(b, a)| n_doubled * (a + b) - (2 * a * b) == n_squared)
        .unwrap();

    a * b * (n - a - b)
}

fn main() {
    println!("{:?}", solve_classic(PROBLEM_INPUT));
}

benchtest! {
    problem_solve_classic: solve_classic(PROBLEM_INPUT) => 31875000,
    problem_solve_fp: solve_fp(PROBLEM_INPUT) => 31875000
}
