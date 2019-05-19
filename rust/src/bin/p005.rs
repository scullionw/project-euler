#![feature(test)]

use benchtest::benchtest;

const BASE_CASE_INPUT: u64 = 10;
const PROBLEM_INPUT: u64 = 20;

fn solve(n: u64) -> u64 {
    let mut candidate = n;
    let divisors = generate_useful_divisors(n);
    'outer: loop {
        for divisor in &divisors {
            if candidate % divisor != 0 {
                candidate += n;
                continue 'outer;
            }
        }
        break candidate;
    }
}

fn generate_useful_divisors(n: u64) -> Vec<u64> {
    let mut confirmed = vec![];

    let mut candidates = (2..n).collect::<Vec<_>>();
    candidates.retain(|&x| n % x != 0);

    while !candidates.is_empty() {
        let last = candidates.pop().unwrap();
        confirmed.push(last);
        candidates.retain(|&x| last % x != 0);
    }
    confirmed
}

fn main() {
    println!("{:?}", solve(PROBLEM_INPUT));
}

benchtest! {
    base_case: solve(BASE_CASE_INPUT) => 2520,
    problem_solve: solve(PROBLEM_INPUT) => 232792560
}
