#![cfg_attr(feature = "benchmode", feature(test))]

extern crate euler;

fn solve(n: u64) -> u64 {
    euler::largest_prime_factor(n)
}

const PROBLEM_INPUT: u64 = 600_851_475_143;

fn main() {
    euler::go(solve, PROBLEM_INPUT);
    euler::bench(solve, PROBLEM_INPUT, 1000);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base_case() {
        assert_eq!(solve(13_195), 29);
    }
    #[test]
    fn correct_answer() {
        assert_eq!(solve(PROBLEM_INPUT), 6857);
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
