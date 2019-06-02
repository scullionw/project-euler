#![feature(test)]

use benchtest::*;
use std::cmp;

const BASE_CASE_DATA: &str = include_str!("data/p018_base_data.txt");
const PROBLEM_DATA: &str = include_str!("data/p018_data.txt");

fn solve(seq: &str) -> u32 {
    let triangle = Triangle::new(seq);
    triangle.max_chain_sum(0, 0)
}

#[derive(Debug)]
struct Triangle {
    data: Vec<Vec<u32>>,
}

impl Triangle {
    fn new(s: &str) -> Self {
        let mut data = vec![];
        for line in s.lines() {
            let row = line
                .split_whitespace()
                .map(|c| c.parse::<u32>().unwrap())
                .collect();
            data.push(row)
        }
        Self { data }
    }

    fn max_chain_sum(&self, row: usize, col: usize) -> u32 {
        if row + 1 < self.data.len() {
            self.data[row][col]
                + cmp::max(
                    self.max_chain_sum(row + 1, col),
                    self.max_chain_sum(row + 1, col + 1),
                )
        } else {
            self.data[row][col]
        }
    }
}

fn main() {
    println!("{:?}", solve(PROBLEM_DATA));
}

benchtest! {
    base_case: solve(BASE_CASE_DATA) => 23,
    problem_solve: solve(PROBLEM_DATA) => 1074
}
