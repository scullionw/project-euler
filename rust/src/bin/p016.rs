#![feature(test)]

use benchtest::*;
use euler::BigNum;
use num_bigint::ToBigInt;
use ramp::Int;

const BASE_CASE_INPUT: usize = 15;
const PROBLEM_INPUT: usize = 1000;

fn solve(n: usize) -> u32 {
    (BigNum::new(2) << (n - 1)).digit_sum()
}

fn solve_with_bigint(n: usize) -> u32 {
    num_traits::pow(2.to_bigint().unwrap(), n)
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .sum()
}

fn solve_with_ramp(n: usize) -> u32 {
    Int::from(2)
        .pow(n)
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .sum()
}

fn main() {
    println!("{:?}", solve(PROBLEM_INPUT));
}

benchtest! {
    base_case: solve(BASE_CASE_INPUT) => 26,
    problem_solve: solve(PROBLEM_INPUT) => 1366,
    base_case_bigint: solve_with_bigint(BASE_CASE_INPUT) => 26,
    problem_solve_bigint: solve_with_bigint(PROBLEM_INPUT) => 1366,
    base_case_ramp: solve_with_ramp(BASE_CASE_INPUT) => 26,
    problem_solve_ramp: solve_with_ramp(PROBLEM_INPUT) => 1366
}
