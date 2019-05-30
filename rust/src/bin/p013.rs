#![feature(test)]

use benchtest::*;

const PROBLEM_DATA: &str = include_str!("data/p013_data.txt");

fn solve(seq: &str) -> u64 {
    let mut sum = 0;
    for line in seq.lines() {
        sum += line[..11].parse::<u64>().unwrap();
    }
    sum.to_string()[..10].parse::<u64>().unwrap()
}

fn main() {
    println!("{:?}", solve(PROBLEM_DATA));
}

benchtest! {
    problem_solve: solve(PROBLEM_DATA) => 5537376230
}
