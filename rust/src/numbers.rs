use num_integer::Integer;
use std::iter;
use std::mem;
use std::ops::{Add, Mul, Shl};

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

#[derive(Debug, Clone)]
pub struct BigNum {
    digits: Vec<u32>,
}

impl BigNum {
    pub fn new(n: u32) -> Self {
        Self {
            digits: n
                .to_string()
                .chars()
                .rev()
                .map(|x| x.to_digit(10).unwrap())
                .collect(),
        }
    }

    pub fn digit_sum(&self) -> u32 {
        self.digits.iter().sum()
    }
}

impl From<Vec<u32>> for BigNum {
    fn from(item: Vec<u32>) -> Self {
        Self { digits: item }
    }
}

impl Mul<u32> for BigNum {
    type Output = Self;

    fn mul(mut self, rhs: u32) -> Self::Output {
        if rhs == 0 {
            BigNum::new(0)
        } else {
            for _ in 1..rhs {
                self = self.clone() + self;
            }
            self
        }
    }
}

impl Shl<usize> for BigNum {
    type Output = Self;

    fn shl(mut self, rhs: usize) -> BigNum {
        for _ in 0..rhs {
            self = self * 2;
        }
        self
    }
}

impl Add for BigNum {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut a = self.digits.iter();
        let mut b = other.digits.iter();
        let mut new = vec![];
        let mut carry = 0;

        loop {
            match (a.next(), b.next()) {
                (Some(d1), Some(d2)) => {
                    let (quotient, remainder) = (d1 + d2 + carry).div_rem(&10);
                    carry = quotient;
                    new.push(remainder);
                }
                (Some(d1), None) => {
                    let (quotient, remainder) = (d1 + carry).div_rem(&10);
                    carry = quotient;
                    new.push(remainder);
                }
                (None, Some(d2)) => {
                    let (quotient, remainder) = (d2 + carry).div_rem(&10);
                    carry = quotient;
                    new.push(remainder);
                }
                (None, None) => {
                    if carry != 0 {
                        new.push(carry);
                    }
                    break new.into();
                }
            }
        }
    }
}
