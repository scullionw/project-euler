#![cfg_attr(feature = "benchmode", feature(test))]

extern crate euler;

fn solve(n: u64) -> u64 {
    let mut candidate = n;
    let divisors = generate_useful_divisors(n);
    'outer: loop {
        for divisor in divisors.iter() {
            if candidate % divisor != 0 {
                candidate += n;
                continue 'outer;
            }
        }
        break candidate;
    }
}

fn generate_useful_divisors(n: u64) -> Vec<u64> {
    let mut confirmed = vec![];

    let mut candidates = (2..n).collect::<Vec<u64>>();
    candidates.retain(|&x| n % x != 0);
    
    while candidates.len() > 0 {
        let last = candidates.pop().unwrap();
        confirmed.push(last);
        candidates.retain(|&x| last % x != 0);
    }
    confirmed
}

const PROBLEM_INPUT: u64 = 20;

fn main() {
    euler::go(solve, PROBLEM_INPUT);
    euler::bench(solve, PROBLEM_INPUT, 10);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base_case() {
        assert_eq!(solve(10), 2520);
    }
    #[test]
    fn correct_answer() {
        assert_eq!(solve(PROBLEM_INPUT), 232792560);
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