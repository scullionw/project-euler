#![cfg_attr(feature = "benchmode", feature(test))]

extern crate euler;

mod classic {
    pub fn solve(n: u64) -> u64 {
        let mut a = 1;
        let mut b = 2;
        let mut sum = 0;
        while b < n {
            if b % 2 == 0 {
                sum += b;
            }
            let old_a = a;
            a = b;
            b += old_a;
        }
        sum
    }
}

mod fp {
    use euler::Fibonnaci;

    pub fn solve(n: u64) -> u64 {
        Fibonnaci::new()
            .take_while(|&x| x < n)
            .filter(|&x| x % 2 == 0)
            .sum()
    }
}

const PROBLEM_INPUT: u64 = 4_000_000;

fn main() {
    euler::go(classic::solve, PROBLEM_INPUT);
    euler::bench(classic::solve, PROBLEM_INPUT, 1000);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn correct_answer() {
        assert_eq!(classic::solve(PROBLEM_INPUT), 4613732);
        assert_eq!(fp::solve(PROBLEM_INPUT), 4613732);
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
