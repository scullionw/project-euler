#![feature(test)]

use benchtest::*;

const BASE_CASE_INPUT: u64 = 5;
const PROBLEM_INPUT: u64 = 500;

fn solve(n: u64) -> u64 {
    let mut i = 1;
    loop {
        let triangle = triangle_num(i);
        if divisors(triangle) > n {
            break triangle;
        }
        i += 1;
    }
}

fn divisors(mut n: u64) -> u64 {
    let mut current_divisor = 2;
    let mut acc = 1;
    while n > 1 {
        let mut times = 0;
        while n % current_divisor == 0 {
            times += 1;
            n /= current_divisor;
        }
        acc *= times + 1;
        current_divisor += 1;
    }
    acc
}

fn triangle_num(n: u64) -> u64 {
    ((n + 1) * n) / 2
}

fn main() {
    println!("{:?}", solve(PROBLEM_INPUT));
}

benchtest! {
    base_case: solve(BASE_CASE_INPUT) => 28,
    problem_solve: solve(PROBLEM_INPUT) => 76576500
}
