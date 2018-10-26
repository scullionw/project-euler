#![cfg_attr(feature = "benchmode", feature(test))]

extern crate euler;

mod fp {
    pub fn solve(n: u64) -> u64 {
        let sum_of_squares = (1..=n).map(|x| x.pow(2)).sum::<u64>();
        let square_of_sum = (1..=n).sum::<u64>().pow(2);
        square_of_sum - sum_of_squares
    }
}

mod classic {
    pub fn solve(n: u64) -> u64 {
        let mut sum = 0;
        let mut sum_of_squares = 0;
        for i in 1..=n {
            sum += i;
            sum_of_squares += i.pow(2);
        }
        sum.pow(2) - sum_of_squares
    }
}

const PROBLEM_INPUT: u64 = 100;

fn main() {
    euler::go(classic::solve, PROBLEM_INPUT);
    euler::bench(classic::solve, PROBLEM_INPUT, 1000);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base_case() {
        assert_eq!(classic::solve(10), 2640);
        assert_eq!(fp::solve(10), 2640);
    }
    #[test]
    fn correct_answer() {
        assert_eq!(classic::solve(PROBLEM_INPUT), 25164150);
        assert_eq!(fp::solve(PROBLEM_INPUT), 25164150);
    }
}

#[cfg(all(feature = "benchmode", test))]
mod bench {
    use super::*;
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_classic(b: &mut Bencher) {
        b.iter(|| classic::solve(PROBLEM_INPUT))
    }
    #[bench]
    fn bench_fp(b: &mut Bencher) {
        b.iter(|| fp::solve(PROBLEM_INPUT))
    }
}
