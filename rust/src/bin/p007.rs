#![feature(test)]

use benchtest::benchtest;

const BASE_CASE_INPUT: u64 = 6;
const PROBLEM_INPUT: u64 = 10001;

fn solve(n: u64) -> u64 {
    euler::PrimeSequence::new().nth((n - 1) as usize).unwrap()
}

fn main() {
    println!("{:?}", solve(PROBLEM_INPUT));
}

benchtest! {
    base_case: solve(BASE_CASE_INPUT) => 13,
    problem_solve: solve(PROBLEM_INPUT) => 104743
}
