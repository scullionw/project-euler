#![cfg_attr(feature = "benchmode", feature(test))]

extern crate euler;

mod classic {
    pub fn solve(n: u64) -> u64 {
        // (n - a - b)^2 = a^2 + b^2
        // n^2 = 2na + 2nb - 2ab
        let n_squared = n.pow(2);
        let n_doubled = 2 * n;
        let n_halved = n / 2;

        'net: loop {
            for b in 1..=n_halved {
                for a in 1..b {
                    if n_doubled * (a + b) - (2 * a * b) == n_squared {
                        break 'net a * b * (1000 - a - b);
                    }
                }
            }
            unreachable!();
        }
    }
}

mod fp {
    pub fn solve(n: u64) -> u64 {
        let n_squared = n.pow(2);
        let n_doubled = 2 * n;
        let n_halved = n / 2;

        let (b, a) = (1..=n_halved)
            .flat_map(|b| (1..=b).map(move |a| (b, a)))
            .find(|(b, a)| n_doubled * (a + b) - (2 * a * b) == n_squared)
            .unwrap();

        a * b * (n - a - b)
    }
}

const PROBLEM_INPUT: u64 = 1000;

fn main() {
    euler::go(classic::solve, PROBLEM_INPUT);
    euler::bench(classic::solve, PROBLEM_INPUT, 10);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn correct_answer() {
        assert_eq!(classic::solve(PROBLEM_INPUT), 31875000);
        assert_eq!(fp::solve(PROBLEM_INPUT), 31875000);
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
