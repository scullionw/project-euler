#![cfg_attr(feature = "benchmode", feature(test))]

extern crate euler;

fn solve(seq: &str, n: usize) -> u64 {
    seq.chars()
        .filter_map(|x| x.to_digit(10).map(u64::from))
        .collect::<Vec<_>>()
        .windows(n)
        .map(|x| x.iter().product())
        .max()
        .unwrap()
}

const PROBLEM_INPUT: usize = 13;
const PROBLEM_DATA: &str = include_str!("data/p008_data.txt");

fn main() {
    euler::go(|n| solve(PROBLEM_DATA, n), PROBLEM_INPUT);
    euler::bench(|n| solve(PROBLEM_DATA, n), PROBLEM_INPUT, 10);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base_case() {
        assert_eq!(solve(PROBLEM_DATA, 4), 5832);
    }
    #[test]
    fn correct_answer() {
        assert_eq!(solve(PROBLEM_DATA, PROBLEM_INPUT), 23514624000);
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
