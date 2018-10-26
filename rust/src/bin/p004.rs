#![cfg_attr(feature = "benchmode", feature(test))]

extern crate euler;

fn solve(digits: u64) -> u64 {
    let start = 10_u64.pow(digits as u32 - 1);
    let end = 10_u64.pow(digits as u32);
    let mut largest = 0;
    for x in start..end {
        for y in x..end {
            let product = x * y;
            if product > largest {
                let repr = product.to_string();
                if repr == repr.chars().rev().collect::<String>() {
                    largest = product;
                }
            }
        }
    }
    largest
}

const PROBLEM_INPUT: u64 = 3;

fn main() {
    euler::go(solve, PROBLEM_INPUT);
    euler::bench(solve, PROBLEM_INPUT, 10);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base_case() {
        assert_eq!(solve(2), 9009);
    }
    #[test]
    fn correct_answer() {
        assert_eq!(solve(PROBLEM_INPUT), 906609);
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
