#![feature(test)]

use benchtest::benchtest;

const PROBLEM_INPUT: u64 = 4_000_000;

pub fn solve_classic(n: u64) -> u64 {
    let mut a = 1;
    let mut b = 2;
    let mut sum = 0;
    while b < n {
        if b % 2 == 0 {
            sum += b;
        }
        let old_a = a;
        a = b;
        b += old_a;
    }
    sum
}

use euler::Fibonnaci;

pub fn solve_fp(n: u64) -> u64 {
    Fibonnaci::new()
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
