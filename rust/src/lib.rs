extern crate time;
use time::{PreciseTime, Duration};


pub struct Fibonnaci {
    prev: u64,
    current: u64
}

impl Fibonnaci {
    pub fn new() -> Fibonnaci {
        Fibonnaci { prev: 0, current: 1 }
    }
}

impl Iterator for Fibonnaci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.prev + self.current;
        self.prev = self.current;
        self.current = next;
        Some(next)
    }
}

pub fn div_mod(n: u64, divisor: u64) -> (u64, u64) {
    let quotient = n / divisor;
    (quotient, n - divisor * quotient)
}

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

fn measure_execution<T, F>(func: F, arg: T) -> (T, Duration)
where
    T: Copy,
    F: Fn(T) -> T
{
    let begin = PreciseTime::now();
    let answer = func(arg);
    let end = PreciseTime::now();
    (answer, begin.to(end))
}

pub fn go<T, F>(func: F, arg: T)
where
    T: Copy + std::fmt::Display,
    F: Fn(T) -> T
{
    let (answer, duration) = measure_execution(func, arg);
    let mode = if cfg!(feature = "fp") { "FP" } else { "Normal" };
    println!("{}: {}, {:?}", mode, answer, duration);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_largest_prime_factor() {
        assert_eq!(super::largest_prime_factor(22), 11);
        assert_eq!(super::largest_prime_factor(8), 2);
    }
}
