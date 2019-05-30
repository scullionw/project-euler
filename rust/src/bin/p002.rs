#![feature(test)]

use benchtest::benchtest;
use euler::Fibonacci;
use std::mem;

const PROBLEM_INPUT: u64 = 4_000_000;

pub fn solve_classic(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    let mut sum = 0;
    while b < n {
        if b % 2 == 0 {
            sum += b;
        }
        b += mem::replace(&mut a, b);
    }
    sum
}

pub fn solve_fp(n: u64) -> u64 {
    Fibonacci::seq()
        .take_while(|&x| x < n)
        .filter(|&x| x % 2 == 0)
        .sum()
}

fn main() {
    println!("{:?}", solve_classic(PROBLEM_INPUT));
}

benchtest! {
    problem_solve_classic: solve_classic(PROBLEM_INPUT) => 4613732,
    problem_solve_fp: solve_fp(PROBLEM_INPUT) => 4613732
}
