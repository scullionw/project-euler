#![feature(test)]

use benchtest::benchtest;

const BASE_CASE_INPUT: u64 = 10;
const PROBLEM_INPUT: u64 = 1000;

fn solve_classic(n: u64) -> u64 {
    let mut sum = 0;
    for i in 1..n {
        if (i % 3 == 0) || (i % 5 == 0) {
            sum += i;
        }
    }
    sum
}

fn solve_fp(n: u64) -> u64 {
    (1..n).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

fn main() {
    println!("{:?}", solve_classic(PROBLEM_INPUT));
}

benchtest! {
    base_case_classic: solve_classic(BASE_CASE_INPUT) => 23,
    base_case_fp: solve_fp(BASE_CASE_INPUT) => 23,
    problem_solve_classic: solve_classic(PROBLEM_INPUT) => 233168,
    problem_solve_fp: solve_fp(PROBLEM_INPUT) => 233168
}
