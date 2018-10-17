#![cfg_attr(feature = "benchmode", feature(test))]

extern crate euler;
// try switching back to if() instead of function ptr
fn solve(n: u64) -> u64 {
    euler::PrimeSequence::new().take_while(|&x| x < n).sum()
}

const PROBLEM_INPUT: u64 = 2_000_000;

fn main() {
    euler::go(solve, PROBLEM_INPUT);
    //euler::bench(solve, PROBLEM_INPUT, 10);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base_case() {
        assert_eq!(solve(10), 17);
    }
    #[test]
    fn correct_answer() {
        assert_eq!(solve(PROBLEM_INPUT), 142913828922);
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