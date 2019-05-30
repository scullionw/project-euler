use std::mem;
use std::iter;

pub struct Fibonacci {
    prev: u64,
    current: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Self {
            prev: 0,
            current: 1,
        }
    }

    pub fn seq() -> impl Iterator<Item = u64> {
        let mut a = 0;
        let mut b = 1;
        iter::repeat_with(move || {
            b += mem::replace(&mut a, b);
            b
    })
}
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += mem::replace(&mut self.prev, self.current);
        Some(self.current)
    }
}

pub fn div_mod(n: u64, divisor: u64) -> (u64, u64) {
    let quotient = n / divisor;
    (quotient, n - divisor * quotient)
}
