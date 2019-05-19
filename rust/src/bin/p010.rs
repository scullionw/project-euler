#![feature(test)]

use benchtest::benchtest;

const BASE_CASE_INPUT: u64 = 10;
const PROBLEM_INPUT: u64 = 2_000_000;

// try switching back to if() instead of function ptr
fn solve(n: u64) -> u64 {
    euler::PrimeSequence::new().take_while(|&x| x < n).sum()
}

fn main() {
    println!("{:?}", solve(PROBLEM_INPUT));
}

benchtest! {
    base_case: solve(BASE_CASE_INPUT) => 17,
    problem_solve: solve(PROBLEM_INPUT) => 142913828922
}
