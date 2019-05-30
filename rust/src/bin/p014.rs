#![feature(test)]

use benchtest::*;

const PROBLEM_INPUT: u64 = 1_000_000;

fn solve(upto: u64) -> u64 {
    (2..upto)
        .map(|n| (n, collatz(n).count()))
        .max_by_key(|&(_, count)| count)
        .unwrap()
        .0
}

fn collatz(n: u64) -> impl Iterator<Item = u64> {
    iter::successors(Some(n), |&n| match n {
        1 => None,
        _ if n % 2 == 0 => Some(n / 2),
        _ => Some(3 * n + 1),
    })
}

fn main() {
    println!("{:?}", solve(PROBLEM_INPUT));
}

benchtest! {
    problem_solve: solve(PROBLEM_INPUT) => 837799
}
