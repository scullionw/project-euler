#![feature(test)]

use benchtest::benchtest;

const BASE_CASE_INPUT: u64 = 13_195;
const PROBLEM_INPUT: u64 = 600_851_475_143;

fn solve(n: u64) -> u64 {
    euler::largest_prime_factor(n)
}

fn main() {
    println!("{:?}", solve(PROBLEM_INPUT));
}

benchtest! {
    base_case: solve(BASE_CASE_INPUT) => 29,
    problem_solve: solve(PROBLEM_INPUT) => 6857
}
