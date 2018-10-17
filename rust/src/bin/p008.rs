#![cfg_attr(feature = "benchmode", feature(test))]

extern crate euler;

fn solve(seq: &str, n: usize) -> u64 {
    seq .lines()
        .fold(String::new(), |a, b| a + b)
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>()
        .windows(n)
        .map(|x| x.iter().product())
        .max()
        .unwrap()
}

fn solve_fast(seq: &str, n: usize) -> u64 {
    seq .as_bytes()
        .iter()
        .filter(|&&x| x != 92 && x != 110 && x != 10)
        .map(|x| (x - 48) as u64)
        .collect::<Vec<u64>>()
        .windows(n)
        .map(|x| x.iter().product())
        .max()
        .unwrap()
}

const PROBLEM_INPUT: usize = 13;
const PROBLEM_DATA: &str = include_str!("data/p008_data.txt");

fn main() {
    //solve_fast(PROBLEM_DATA, PROBLEM_INPUT);
    euler::go(|n| solve_fast(PROBLEM_DATA, n), PROBLEM_INPUT);
    //euler::bench(solve, PROBLEM_INPUT, 10);
}

#[cfg(test)]
mod tests {
    use super::*;
        #[test]
    fn base_case() {
        assert_eq!(solve(PROBLEM_DATA, 4), 5832);
        assert_eq!(solve_fast(PROBLEM_DATA, 4), 5832);
    }
    #[test]
    fn correct_answer() {
        assert_eq!(solve(PROBLEM_DATA, PROBLEM_INPUT), 23514624000);
        assert_eq!(solve_fast(PROBLEM_DATA, PROBLEM_INPUT), 23514624000);
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
    #[bench]
    fn bench_fast(b: &mut Bencher) {
        b.iter(|| solve_fast(PROBLEM_DATA, PROBLEM_INPUT));
    }
}