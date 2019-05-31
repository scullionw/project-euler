#![feature(test)]

use benchtest::*;

const PROBLEM_INPUT: u128 = 20;

// How many different ways can we pick 20 rights in 40 moves
fn solve(n: u128) -> u128 {
    n_choose_k(n * 2, n)
}

fn n_choose_k(n: u128, k: u128) -> u128 {
    ((n - k + 1)..=n).product::<u128>() / (1..=k).product::<u128>()
}

fn main() {
    println!("{:?}", solve(PROBLEM_INPUT));
}

benchtest! {
    problem_solve: solve(PROBLEM_INPUT) => 137846528820
}
