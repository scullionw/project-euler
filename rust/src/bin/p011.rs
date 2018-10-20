#![cfg_attr(feature = "benchmode", feature(test))]

extern crate euler;
use euler::Matrix;

fn solve(seq: &str, _n: usize) {
    let data = seq  .lines()
                    .flat_map(|line| line.split_whitespace())
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();

    
    let mat = Matrix::from_square_slice(&data, 20);

    for row in mat.chunk_rows() {
        println!("{:?}", row);
    }

    for diag in mat.upper_diagonals() {
        println!("{:?}", diag);
    }
}

const PROBLEM_DATA: &str = include_str!("data/p011_data.txt");
const PROBLEM_INPUT: usize = 4;

fn main() {
    solve(PROBLEM_DATA, PROBLEM_INPUT);
    //euler::go(|n| solve(PROBLEM_DATA, n), PROBLEM_INPUT);
    //euler::bench(solve, PROBLEM_INPUT, 10);
}

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn correct_answer() {
    //     assert_eq!(solve(PROBLEM_DATA, PROBLEM_INPUT), 70600674);
    // }
}

#[cfg(all(feature = "benchmode", test))]
mod bench {
    use super::*;
    extern crate test;
    use self::test::Bencher;
    
    // #[bench]
    // fn bench_solve(b: &mut Bencher) {
    //     b.iter(|| solve(PROBLEM_DATA, PROBLEM_INPUT));
    // }
}