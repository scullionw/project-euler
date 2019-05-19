#![feature(test)]

use benchtest::benchtest;

const BASE_CASE_INPUT: usize = 4;
const PROBLEM_INPUT: usize = 13;
const PROBLEM_DATA: &str = include_str!("data/p008_data.txt");

fn solve(seq: &str, n: usize) -> u64 {
    seq.chars()
        .filter_map(|x| x.to_digit(10).map(u64::from))
        .collect::<Vec<_>>()
        .windows(n)
        .map(|x| x.iter().product())
        .max()
        .unwrap()
}

fn main() {
    println!("{:?}", solve(PROBLEM_DATA, PROBLEM_INPUT));
}

benchtest! {
    base_case: solve(PROBLEM_DATA, BASE_CASE_INPUT) => 5832,
    problem_solve: solve(PROBLEM_DATA, PROBLEM_INPUT) => 23514624000
}
