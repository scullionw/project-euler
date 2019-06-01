#![feature(test)]

use benchtest::*;
use euler::BigNum;
use num_bigint::ToBigInt;

const BASE_CASE_INPUT: usize = 15;
const PROBLEM_INPUT: usize = 1000;

fn solve(n: usize) -> u32 {
    (BigNum::new(2) << (n - 1)).digit_sum()
}

fn solve_with_dep(n: usize) -> u32 {
    num_traits::pow(2.to_bigint().unwrap(), n)
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
    base_case2: solve_with_dep(BASE_CASE_INPUT) => 26,
    problem_solve2: solve_with_dep(PROBLEM_INPUT) => 1366

}
