#![cfg_attr(feature = "benchmode", feature(test))]

extern crate euler;

fn solve(n: u64) -> u64 {
    euler::PrimeSequence::new().nth((n - 1) as usize).unwrap()
}

const PROBLEM_INPUT: u64 = 10001;

fn main() {
    euler::go(solve, PROBLEM_INPUT);
    euler::bench(solve, PROBLEM_INPUT, 10);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base_case() {
        assert_eq!(solve(6), 13);
    }
    #[test]
    fn correct_answer() {
        assert_eq!(solve(PROBLEM_INPUT), 104743);
    }
}

#[cfg(all(feature = "benchmode", test))]
mod bench {
    use super::*;
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        b.iter(|| solve(PROBLEM_INPUT));
    }
}
