#![cfg_attr(feature = "benchmode", feature(test))]

extern crate euler;

mod classic {
    pub fn solve(n: u64) -> u64 {
        let mut sum = 0;
        for i in 1..n {
            if (i % 3 == 0) || (i % 5 == 0) {
                sum += i;
            } 
        }
        sum
    }    
}

mod fp {
    pub fn solve(n: u64) -> u64 {
        (1..n).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
    }
}

const PROBLEM_INPUT: u64 = 1000;

fn main() {
    euler::go(classic::solve, PROBLEM_INPUT);
    euler::bench(classic::solve, PROBLEM_INPUT, 1000);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base_case() {
        assert_eq!(classic::solve(10), 23);
        assert_eq!(fp::solve(10), 23);
    }
    #[test]
    fn correct_answer() {
        assert_eq!(classic::solve(PROBLEM_INPUT), 233168);
        assert_eq!(fp::solve(PROBLEM_INPUT), 233168);
    }
}

#[cfg(all(feature = "benchmode", test))]
mod bench {
    use super::*;
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_classic(b: &mut Bencher) {
        b.iter(|| classic::solve(PROBLEM_INPUT));
    }
    #[bench]
    fn bench_fp(b: &mut Bencher) {
        b.iter(|| fp::solve(PROBLEM_INPUT));
    }
}