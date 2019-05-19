#![feature(test)]

use benchtest::benchtest;
use euler::Matrix;

const PROBLEM_DATA: &str = include_str!("data/p011_data.txt");
const PROBLEM_INPUT: usize = 4;

// TODO: try just Vec<Vec>
fn solve(seq: &str, n: usize) -> u64 {
    let data = seq
        .lines()
        .flat_map(|line| line.split_whitespace())
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    Matrix::from_square_slice(&data, 20)
        .upper_diagonals()
        .collect::<Vec<_>>()
        .iter()
        .flat_map(|slice| slice.windows(n))
        .map(|window| window.iter().product())
        .max()
        .unwrap()
}

fn main() {
    println!("{:?}", solve(PROBLEM_DATA, PROBLEM_INPUT));
}

benchtest! {
    problem_solve: solve(PROBLEM_DATA, PROBLEM_INPUT) => 70600674
}
