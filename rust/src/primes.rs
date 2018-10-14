use gcd;
use std::cmp::{min, max};
use std::collections::HashMap;
use fnv::FnvBuildHasher;

pub fn largest_prime_factor(mut n: u64) -> u64 {
    while n % 2 == 0 {
        n /= 2
    }
    if n == 1 {
        return 2;
    }
    let mut divisor = 1;
    while n != 1 {
        divisor += 2;
        while n % divisor == 0 {
            n /= divisor;
        }
    }
    divisor
}

// TODO: Can i make a lazy finder (using Infinity::new())
pub fn primes_under_or_equal_alt(n: u64) -> Vec<u64> {
    let mut candidates: Vec<_> = (2..=n).rev().collect();
    let mut primes = vec![];
    loop {
        match candidates.pop() {
            Some(p) => {
                primes.push(p);
                candidates.retain(|x| x % p != 0);
            },
            None => { break primes }
        }
    }
}

// TODO: Make prime sieve using channels and normally, and with vec retain
pub fn primes_under_or_equal(n: u64) -> Vec<u64> {
    let mut primes = vec![];
    'a: for candidate in 2..=n {
        for divisor in 2..candidate {
            if candidate % divisor == 0 {
                continue 'a;
            }
        }
        primes.push(candidate);
    }
    primes
}

// TODO: Use closures like Duration::span
// TODO: return factors in order
// copied from an old python implementation of mine, unsure if correct
// Recursively factorize non prime results? ex: 45 -> (5, 9) -> (5, 3, 3)
pub fn rho<F>(n: u64, mut x: u64, mut y: u64, func: F) -> (u64, u64)
where
    F: Fn(u64) -> u64
{
    // for _ in 0..100 -> loop forever? return option instead..
    loop {
        let diff = if x > y { x - y } else { y - x };
        let divisor = gcd(diff % n, n);
        if ![1, n].contains(&divisor) {
            break (min(divisor, n / divisor), max(divisor, n / divisor))
        }
        x = func(x) % n;
        y = func(func(y)) % n;
    }
}

// TODO: return factors in order
// TODO: only use primes ?
// copied from an old python implementation of mine, unsure if correct
pub fn p_minus_1(n: u64, mut a: u64) -> (u64, u64) {
    //  for e in 1..10 -> loop forever? return option instead..
    let mut e = 1;
    loop {
        a = a.pow(e) % n;
        let divisor = gcd(a - 1, n);
        if ![1, n].contains(&divisor) {
            break (min(divisor, n / divisor), max(divisor, n / divisor))
        }
        e += 1;
    }
}

// TODO: Add miller rabin primality test
// and the other newer one. Check for other algos used s6app5.

// use fnv hash function for better performance
pub struct PrimeSequence {
    started: bool,
    candidate: u64,
    first_prime_factor: HashMap<u64, u64, FnvBuildHasher>
}

impl PrimeSequence {
    pub fn new() -> PrimeSequence {
        PrimeSequence {
            started: false,
            candidate: 3,
            first_prime_factor: HashMap::default()
        }
    }
}

impl Iterator for PrimeSequence {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        // Return a chain of (2, PrimeSequence)? how? ugly..
        if !self.started {
            self.started = true;
            return Some(2)
        }
        loop {
            let candidate = self.candidate;
            self.candidate += 2;
            match self.first_prime_factor.remove(&candidate) {
                Some(factor) => {
                    let mut next_multiple = factor + candidate;
                    while next_multiple % 2 == 0 || self.first_prime_factor.contains_key(&next_multiple){
                        next_multiple += factor;
                    }
                    self.first_prime_factor.insert(next_multiple, factor);
                },
                None => {
                    self.first_prime_factor.insert(candidate.pow(2), candidate);
                    break Some(candidate);
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_largest_prime_factor() {
        assert_eq!(super::largest_prime_factor(22), 11);
        assert_eq!(super::largest_prime_factor(8), 2);
    }

    #[test]
    fn test_primes_under_or_equal() {
        assert_eq!(super::primes_under_or_equal(20), vec![2,3,5,7,11,13,17,19]);
    }

    #[test]
    fn test_primes_under_or_equal_alt() {
        assert_eq!(super::primes_under_or_equal_alt(20), vec![2,3,5,7,11,13,17,19]);
    }

    #[test]
    fn test_prime_sequence () {
        assert_eq!(super::PrimeSequence::new().take(8).collect::<Vec<u64>>(), vec![2,3,5,7,11,13,17,19]);
    }

    fn poly(x: u64) -> u64 {
        x.pow(2) + 5
    }

    #[test]
    fn test_rho() {
        assert_eq!(super::rho(2867, 1, 1, poly), (47, 61));
    }


    #[test]
    fn test_p_minus_1() {
        assert_eq!(super::p_minus_1(2867, 2), (47, 61));
    }
}