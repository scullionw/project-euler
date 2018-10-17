#![cfg_attr(feature = "benchmode", feature(test))]

extern crate euler;

fn solve(n: u64) -> u64 {
    // (n - a - b)^2 = a^2 + b^2
    // n^2 = 2na + 2nb - 2ab
    let n_squared = n.pow(2);
    let n_doubled = 2 * n;
    let n_halved = n / 2;

    'net: loop {
        for b in 1..=n_halved {
            for a in 1..b {
                if n_doubled * (a + b) - (2 * a * b)  == n_squared {
                    break 'net a * b * (1000 - a - b);
                }
            }
        }
        unreachable!();
    }
}

const PROBLEM_INPUT: u64 = 1000;

fn main() {
    euler::go(solve, PROBLEM_INPUT);
    euler::bench(solve, PROBLEM_INPUT, 10);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn correct_answer() {
        assert_eq!(solve(PROBLEM_INPUT), 31875000);
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