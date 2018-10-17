#![cfg_attr(feature = "benchmode", feature(test))]

extern crate euler;

#[cfg(feature = "fp")]
fn solve(n: u64) -> u64 {
    let sum_of_squares = (1..=n).map(|x| x.pow(2)).sum::<u64>();
    let square_of_sum = (1..=n).sum::<u64>().pow(2);
    square_of_sum - sum_of_squares
}


#[cfg(not(feature = "fp"))]
fn solve(n: u64) -> u64 {
    let mut sum = 0;
    let mut sum_of_squares = 0;
    for i in 1..=n {
        sum += i;
        sum_of_squares += i.pow(2);
    }
    sum.pow(2) - sum_of_squares
}

pub const PROBLEM_INPUT: u64 = 100;

fn main() {
    euler::go(solve, PROBLEM_INPUT);
    euler::bench(solve, PROBLEM_INPUT, 1000);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base_case() {
        assert_eq!(solve(10), 2640);
    }
    #[test]
    fn correct_answer() {
        assert_eq!(solve(PROBLEM_INPUT), 25164150);
    }
}

#[cfg(all(feature = "benchmode", test))]
mod bench {
    use super::*;
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        b.iter(|| solve(PROBLEM_INPUT))
    }
}