#![feature(test)]

use benchtest::benchtest;

const BASE_CASE_INPUT: u64 = 10;
const PROBLEM_INPUT: u64 = 100;

pub fn solve_fp(n: u64) -> u64 {
    let sum_of_squares = (1..=n).map(|x| x.pow(2)).sum::<u64>();
    let square_of_sum = (1..=n).sum::<u64>().pow(2);
    square_of_sum - sum_of_squares
}

pub fn solve_classic(n: u64) -> u64 {
    let mut sum = 0;
    let mut sum_of_squares = 0;
    for i in 1..=n {
        sum += i;
        sum_of_squares += i.pow(2);
    }
    sum.pow(2) - sum_of_squares
}

fn main() {
    println!("{:?}", solve_classic(PROBLEM_INPUT));
}

benchtest! {
    base_case_classic: solve_classic(BASE_CASE_INPUT) => 2640,
    base_case_fp: solve_fp(BASE_CASE_INPUT) => 2640,
    problem_solve_classic: solve_classic(PROBLEM_INPUT) => 25164150,
    problem_solve_fp: solve_fp(PROBLEM_INPUT) => 25164150
}
