#![cfg_attr(feature = "benchmode", feature(test))]

extern crate euler;

#[cfg(not(feature = "fp"))]
fn solve(n: u64) -> u64 {
    let mut sum = 0;
    for i in 1..n {
        if (i % 3 == 0) || (i % 5 == 0) {
            sum += i;
        } 
    }
    sum
}

#[cfg(feature = "fp")]
fn solve(n: u64) -> u64 {
    (1..n).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

const PROBLEM_INPUT: u64 = 1000;

fn main() {
    euler::go(solve, PROBLEM_INPUT);
    euler::bench(solve, PROBLEM_INPUT, 1000);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base_case() {
        assert_eq!(solve(10), 23);
    }
    #[test]
    fn correct_answer() {
        assert_eq!(solve(PROBLEM_INPUT), 233168);
    }
}

#[cfg(all(feature = "benchmode", test))]
mod bench {
    extern crate test;
    use super::*;
    use self::test::Bencher;

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        b.iter(|| super::solve(PROBLEM_INPUT));
    }
}