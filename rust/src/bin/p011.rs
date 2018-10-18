#![cfg_attr(feature = "benchmode", feature(test))]

extern crate euler;
use euler::Matrix;

fn solve(seq: &str, _n: usize) {
    let data = seq  .lines()
                    .flat_map(|line| line.split_whitespace())
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();

    
    let mat = Matrix::from_square_slice(&data, 20); // abstract out dims
    
    // let piece = [11, 22, 33, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 4, 56, 62, 0];

    // mat[1].copy_from_slice(&piece);

    // println!("{:?}", mat.row(1));

    for row in mat.rows() {
        println!("{:?}", row);
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
    #[test]
    fn correct_answer() {
        assert_eq!(solve(PROBLEM_DATA, PROBLEM_INPUT), 70600674);
    }
}

#[cfg(all(feature = "benchmode", test))]
mod bench {
    use super::*;
    extern crate test;
    use self::test::Bencher;
    
    #[bench]
    fn bench_solve(b: &mut Bencher) {
        b.iter(|| solve(PROBLEM_DATA, PROBLEM_INPUT));
    }
}