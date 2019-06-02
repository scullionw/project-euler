#![feature(test)]

use benchtest::*;
use std::cmp;

const BASE_CASE_DATA: &str = include_str!("data/p018_base_data.txt");
const PROBLEM_DATA: &str = include_str!("data/p018_data.txt");

fn solve(seq: &str) -> u32 {
    let mut data = vec![];
    for line in seq.lines() {
        let row = line
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
        data.push(row)
    }

    max_collapse(&mut data)
}

// Bottom up
fn max_collapse(data: &mut Vec<Vec<u32>>) -> u32 {
    for row in (0..data.len() - 1).rev() {
        for col in 0..data[row].len() {
            data[row][col] += cmp::max(data[row + 1][col], data[row + 1][col + 1])
        }
    }
    data[0][0]
}

// Brute force
fn max_chain_sum(data: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
    if row + 1 < data.len() {
        data[row][col]
            + cmp::max(
                max_chain_sum(data, row + 1, col),
                max_chain_sum(data, row + 1, col + 1),
            )
    } else {
        data[row][col]
    }
}

fn main() {
    println!("{:?}", solve(PROBLEM_DATA));
}

benchtest! {
    base_case: solve(BASE_CASE_DATA) => 23,
    problem_solve: solve(PROBLEM_DATA) => 1074
}
