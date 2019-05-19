#![feature(test)]

use benchtest::benchtest;

const BASE_CASE_INPUT: u64 = 2;
const PROBLEM_INPUT: u64 = 3;

fn solve(digits: u64) -> u64 {
    let start = 10_u64.pow(digits as u32 - 1);
    let end = 10_u64.pow(digits as u32);
    let mut largest = 0;
    for x in start..end {
        for y in x..end {
            let product = x * y;
            if product > largest {
                let repr = product.to_string();
                if repr == repr.chars().rev().collect::<String>() {
                    largest = product;
                }
            }
        }
    }
    largest
}

fn main() {
    println!("{:?}", solve(PROBLEM_INPUT));
}

benchtest! {
    base_case: solve(BASE_CASE_INPUT) => 9009,
    problem_solve: solve(PROBLEM_INPUT) => 906609
}
