use benchtest::*;
use num_integer::Integer;
use std::char;
use std::fmt;
use std::mem;
use std::ops::{Add, Mul, MulAssign, Shl};

type Digit = u8;

/// Toy impl of BigNum, doesn't use intrinsics
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct BigNum {
    digits: Vec<Digit>,
}

impl BigNum {
    pub fn digits(&self) -> impl Iterator<Item = u32> + '_ {
        self.digits.iter().rev().map(|d| *d as u32)
    }

    pub fn factorial(self) -> Self {
        let mut product = BigNum::from(1);
        let mut val = BigNum::from(2);
        loop {
            product = product * val.clone();
            if val == self {
                break product;
            } else {
                val = val + 1;
            }
        }
    }

    fn from_validated_digits(digits: Vec<Digit>) -> Self {
        Self { digits }
    }
}

impl Mul<u32> for BigNum {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        match rhs {
            0 => BigNum::from(0),
            1 => self,
            2 => &self + &self,
            _ => self * BigNum::from(rhs),
        }
    }
}

impl MulAssign<usize> for BigNum {
    fn mul_assign(&mut self, rhs: usize) {
        match rhs {
            0 => *self = BigNum::from(0),
            1 => (),
            2 => *self = &*self + &*self,
            _ => *self = self.clone() * BigNum::from(rhs),
        };
    }
}

impl fmt::Display for BigNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.digits
                .iter()
                .rev()
                .map(|&d| char::from_digit(d.into(), 10).unwrap())
                .collect::<String>()
        )
    }
}

impl Mul<BigNum> for BigNum {
    type Output = Self;

    fn mul(self, rhs: BigNum) -> Self {
        let zero = BigNum::from(0);
        if self == zero || rhs == zero {
            return zero;
        }

        let (small, big) = if rhs > self { (self, rhs) } else { (rhs, self) };

        let mut temp_nums = vec![];

        for (index, b) in small.digits.iter().enumerate() {
            let mut temp_num = vec![];
            (0..index).for_each(|_| temp_num.push(0));
            let mut carry = 0;

            for a in big.digits.iter() {
                let (quotient, remainder) = (a * b + carry).div_rem(&10);
                carry = quotient;
                temp_num.push(remainder);
            }

            if carry > 0 {
                temp_num.push(carry);
            }
            temp_nums.push(temp_num);
        }

        temp_nums.into_iter().fold(BigNum::from(0), |acc, x| {
            acc + BigNum::from_validated_digits(x)
        })
    }
}

impl Shl<usize> for BigNum {
    type Output = Self;

    fn shl(mut self, rhs: usize) -> Self {
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
                    break BigNum::from_validated_digits(new);
                }
            }
        }
    }
}

impl Add<&BigNum> for BigNum {
    type Output = Self;

    fn add(self, other: &BigNum) -> Self {
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
                    break BigNum::from_validated_digits(new);
                }
            }
        }
    }
}

impl Add<&BigNum> for &BigNum {
    type Output = BigNum;

    fn add(self, other: &BigNum) -> BigNum {
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
                    break BigNum::from_validated_digits(new);
                }
            }
        }
    }
}

impl Add<u32> for BigNum {
    type Output = BigNum;

    fn add(self, rhs: u32) -> Self {
        self + BigNum::from(rhs)
    }
}

macro_rules! from_impl {
    ($($t:ty)*) => ($(
        impl From<$t> for BigNum  {
            // #[inline]
            fn from(mut n: $t) -> Self {
                match n {
                    x @ 0..=9 => Self { digits: vec![x as Digit] },
                    _ => {
                        let mut digits = vec![];
                        loop {
                            let (quotient, remainder) = n.div_rem(&10);
                            digits.push(remainder as Digit);
                            if quotient == 0 {
                                break Self { digits };
                            } else {
                                n = quotient;
                            }
                        }
                    }
                }
            }
        }
    )*)
}

macro_rules! from_impl_check {
    ($($t:ty)*) => ($(
        impl From<$t> for BigNum  {
            // #[inline]
            fn from(mut n: $t) -> Self {
                assert!(n >= 0);
                match n {
                    x @ 0..=9 => Self { digits: vec![x as Digit] },
                    _ => {
                        let mut digits = vec![];
                        loop {
                            let (quotient, remainder) = n.div_rem(&10);
                            digits.push(remainder as Digit);
                            if quotient == 0 {
                                break Self { digits };
                            } else {
                                n = quotient;
                            }
                        }
                    }
                }
            }
        }
    )*)
}

from_impl! { u8 u32 u64 u128 usize }

from_impl_check! { i8 i32 i64 i128 }

test! {
    add_1: BigNum::from(22) + BigNum::from(50) => BigNum::from(72),
    add_2: BigNum::from(101) + BigNum::from(2002) => BigNum::from(2103),
    shl_1: BigNum::from(2) << 3 => BigNum::from(16),
    shl_2: BigNum::from(13) << 4 => BigNum::from(208),
    mul_1: BigNum::from(5) * BigNum::from(6) => BigNum::from(30),
    mul_2: BigNum::from(156) * BigNum::from(89) => BigNum::from(13884),
    mulint_1: BigNum::from(5) * 6 => BigNum::from(30),
    mulint_2: BigNum::from(2) * 3 => BigNum::from(6)
}

benchtest! {
    mul_big_int_0: BigNum::from(6453264) * 0 => BigNum::from(0),
    mul_big_big_0: BigNum::from(6453264) * BigNum::from(0) => BigNum::from(0),
    mul_big_int_1: BigNum::from(6453264) * 1 => BigNum::from(6453264),
    mul_big_big_1: BigNum::from(6453264) * BigNum::from(1) => BigNum::from(6453264),
    mul_big_int_2: BigNum::from(6453264) * 2 => BigNum::from(12906528),
    mul_big_big_2: BigNum::from(6453264) * BigNum::from(2) => BigNum::from(12906528),
    mul_big_int_3: BigNum::from(6453264) * 3 => BigNum::from(19359792),
    mul_big_big_3: BigNum::from(6453264) * BigNum::from(3) => BigNum::from(19359792),
    mul_big_int_4: BigNum::from(6453264) * 4 => BigNum::from(25813056),
    mul_big_big_4: BigNum::from(6453264) * BigNum::from(4) => BigNum::from(25813056),
    mul_big_int_n: BigNum::from(6453264) * 37 => BigNum::from(238770768),
    mul_big_big_n: BigNum::from(6453264) * BigNum::from(37) => BigNum::from(238770768),
    rev_mul_big_big_0: BigNum::from(0) * BigNum::from(6453264) => BigNum::from(0),
    rev_mul_big_big_1: BigNum::from(1) * BigNum::from(6453264) => BigNum::from(6453264),
    rev_mul_big_big_2: BigNum::from(2) * BigNum::from(6453264) => BigNum::from(12906528),
    rev_mul_big_big_3: BigNum::from(3) * BigNum::from(6453264) => BigNum::from(19359792),
    rev_mul_big_big_4: BigNum::from(4) * BigNum::from(6453264) => BigNum::from(25813056),
    rev_mul_big_big_n: BigNum::from(37) * BigNum::from(6453264) => BigNum::from(238770768),
    fact_1: BigNum::from(6).factorial() => BigNum::from(720)
}
