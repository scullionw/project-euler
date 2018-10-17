#![cfg_attr(feature = "benchmode", feature(test))]

extern crate euler;

#[cfg(feature = "fp")]
use euler::Fibonnaci;

#[cfg(not(feature = "fp"))]
fn solve(n: u64) -> u64 {
    let mut a = 1;
    let mut b = 2;
    let mut sum = 0;
    while b < n {
        if b % 2 == 0 {
            sum += b;
        }
        // Next fibonnaci value (b)
        // Consider switching (and benching)    let (a, b) = (b, a + b);
        let temp_a = a;
        a = b;
        b = temp_a + b;
    }
    sum
}

#[cfg(feature = "fp")]
fn solve(n: u64) -> u64 {
    Fibonnaci::new().take_while(|&x| x < n)
                    .filter(|&x| x % 2 == 0)
                    .sum()
}


const PROBLEM_INPUT: u64 = 4_000_000;

fn main() {
    euler::go(solve, PROBLEM_INPUT);
    euler::bench(solve, PROBLEM_INPUT, 1000);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn correct_answer() {
        assert_eq!(solve(PROBLEM_INPUT), 4613732)
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