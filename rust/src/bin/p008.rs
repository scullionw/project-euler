#![cfg_attr(feature = "benchmode", feature(test))]

extern crate euler;

fn solve(seq: &str) -> u64 {
    seq .lines()
        .fold(String::new(), |a, b| a + b)
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>()
        .as_slice()
        .windows(13)
        .map(|x| x.iter().product())
        .max()
        .unwrap()
}

const PROBLEM_INPUT: &str = include_str!("data/p008_data.txt");

fn main() {
    euler::go(solve, PROBLEM_INPUT);
    euler::bench(solve, PROBLEM_INPUT, 10);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn correct_answer() {
        assert_eq!(solve(PROBLEM_INPUT), 23514624000);
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