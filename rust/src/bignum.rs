use benchtest::*;
use num_integer::Integer;
use std::char;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Shl};

type Digit = u8;

/// Toy impl of BigNum, doesn't use intrinsics.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct BigNum {
    digits: Vec<Digit>,
}

// TODO: try using Zero, One, Num (enum)
impl BigNum {
    pub fn digits(&self) -> impl Iterator<Item = u32> + '_ {
        self.digits.iter().rev().map(|d| *d as u32)
    }

    pub fn factorial(self) -> BigNum {
        let mut product = BigNum::from(1);
        let mut val = BigNum::from(2);
        loop {
            product *= &val;
            if val == self {
                break product;
            } else {
                val += 1;
            }
        }
    }

    fn from_validated_digits(digits: Vec<Digit>) -> BigNum {
        BigNum { digits }
    }
}

macro_rules! shl_int_impl {
    ($($t:ty)*) => ($(
        impl Shl<$t> for BigNum {
            type Output = BigNum;
            
            fn shl(mut self, rhs: $t) -> Self::Output {
                for _ in 0..rhs {
                    self *= 2;
                }
                self
            }
        }
    )*)
}

shl_int_impl! { i8 u8 i32 u32 i64 u64 i128 u128 usize }

impl MulAssign<&BigNum> for BigNum {
    #[inline]
    fn mul_assign(&mut self, rhs: &BigNum) {
        *self = &*self * rhs
    }
}

impl MulAssign<BigNum> for BigNum {
    #[inline]
    fn mul_assign(&mut self, rhs: BigNum) {
        *self = &*self * rhs
    }
}


macro_rules! mulassign_int_impl {
    ($($t:ty)*) => ($(
        #[allow(clippy::suspicious_op_assign_impl)]
        impl MulAssign<$t> for BigNum {
            #[inline]
            fn mul_assign(&mut self, rhs: $t) {
                 match rhs {
                    0 => *self = BigNum::from(0),
                    1 => (),
                    2 => *self = &*self + &*self,
                    _ => *self = self.clone() * BigNum::from(rhs),
                };
            }
        }
    )*)
}

mulassign_int_impl! { i8 u8 i32 u32 i64 u64 i128 u128 usize }

macro_rules! mul_int_impl {
    ($($t:ty)*) => ($(
        impl Mul<$t> for BigNum {
            type Output = BigNum;
            #[inline]
            #[allow(clippy::suspicious_arithmetic_impl)]
            fn mul(self, rhs: $t) -> Self::Output {
                match rhs {
                    0 => BigNum::from(0),
                    1 => self,
                    2 => &self + &self,
                    _ => self * BigNum::from(rhs),
                }
            }
        }
        impl Mul<$t> for &BigNum {
            type Output = BigNum;
            #[inline]
            #[allow(clippy::suspicious_arithmetic_impl)]
            fn mul(self, rhs: $t) -> Self::Output {
                match rhs {
                    0 => BigNum::from(0),
                    1 => (*self).clone(),
                    2 => self + self,
                    _ => self * BigNum::from(rhs),
                }
            }
        }
    )*)
}

mul_int_impl! { i8 u8 i32 u32 i64 u64 i128 u128 usize }

impl Mul<BigNum> for BigNum {
    type Output = BigNum;
    #[inline]
    fn mul(self, rhs: BigNum) -> BigNum {
        Mul::mul(&self, &rhs)
    }
}

impl Mul<BigNum> for &BigNum {
    type Output = BigNum;
    #[inline]
    fn mul(self, rhs: BigNum) -> BigNum {
        Mul::mul(self, &rhs)
    }
}

impl Mul<&BigNum> for BigNum {
    type Output = BigNum;
    #[inline]
    fn mul(self, rhs: &BigNum) -> BigNum {
        Mul::mul(&self, rhs)
    }
}

impl Mul<&BigNum> for &BigNum {
    type Output = BigNum;
    #[inline]
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: &BigNum) -> BigNum {
        let zero = BigNum::from(0);
        if self == &zero || rhs == &zero {
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

        temp_nums
            .into_iter()
            .map(BigNum::from_validated_digits)
            .fold(BigNum::from(0), |acc, x| acc + x)
    }
}

macro_rules! addassign_int_impl {
    ($($t:ty)*) => ($(
        impl AddAssign<$t> for BigNum {
            #[inline]
            fn add_assign(&mut self, other: $t) {
                AddAssign::add_assign(self, BigNum::from(other))
            }
        }
    )*)
}

addassign_int_impl! { i8 u8 i32 u32 i64 u64 i128 u128 usize }

impl AddAssign<&BigNum> for BigNum {
    #[inline]
    fn add_assign(&mut self, other: &BigNum) {
        let mut b = other.digits.iter();
        let mut carry = 0;

        for d1 in self.digits.iter_mut() {
            let (quotient, remainder) = if let Some(d2) = b.next() {
                (*d1 + *d2 + carry).div_rem(&10)
            } else {
                (*d1 + carry).div_rem(&10)
            };
            carry = quotient;
            *d1 = remainder;
        }

        for d2 in b {
            let (quotient, remainder) = (*d2 + carry).div_rem(&10);
            carry = quotient;
            self.digits.push(remainder);
        }

        if carry != 0 {
            self.digits.push(carry);
        }
    }
}

impl AddAssign<BigNum> for BigNum {
    #[inline]
    fn add_assign(&mut self, other: BigNum) {
        AddAssign::add_assign(self, &other)
    }
}


impl Add<&BigNum> for BigNum {
    type Output = BigNum;

    #[inline]
    fn add(mut self, other: &BigNum) -> Self::Output {
        let mut b = other.digits.iter();
        let mut carry = 0;

        for d1 in self.digits.iter_mut() {
            let (quotient, remainder) = if let Some(d2) = b.next() {
                (*d1 + *d2 + carry).div_rem(&10)
            } else {
                (*d1 + carry).div_rem(&10)
            };
            carry = quotient;
            *d1 = remainder;
        }

        for d2 in b {
            let (quotient, remainder) = (*d2 + carry).div_rem(&10);
            carry = quotient;
            self.digits.push(remainder);
        }

        if carry != 0 {
            self.digits.push(carry);
        }

        self
    }
}

impl Add<BigNum> for &BigNum {
    type Output = BigNum;

    #[inline]
    fn add(self, other: BigNum) -> BigNum {
        Add::add(other, self)
    }
}

impl Add<BigNum> for BigNum {
    type Output = BigNum;

    #[inline]
    fn add(self, other: BigNum) -> BigNum {
        Add::add(self, &other)
    }
}

impl Add<&BigNum> for &BigNum {
    type Output = BigNum;

    #[inline]
    fn add(self, other: &BigNum) -> BigNum {
        Add::add((*self).clone(), other)
    }
}

macro_rules! add_int_impl {
    ($($t:ty)*) => ($(
        impl Add<$t> for BigNum {
            type Output = BigNum;

            #[inline]
            fn add(self, other: $t) -> BigNum {
                self + BigNum::from(other)
            }
        }

        impl Add<$t> for &BigNum {
            type Output = BigNum;

            #[inline]
            fn add(self, other: $t) -> BigNum {
                self + BigNum::from(other)
            }
        }
    )*)
}

add_int_impl! { i8 u8 i32 u32 i64 u64 i128 u128 usize }

macro_rules! from_impl {
    ($($t:ty)*) => ($(
        impl From<$t> for BigNum  {
            #[inline]
            fn from(mut n: $t) -> BigNum {
                match n {
                    x @ 0..=9 => BigNum { digits: vec![x as Digit] },
                    _ => {
                        let mut digits = vec![];
                        loop {
                            let (quotient, remainder) = n.div_rem(&10);
                            digits.push(remainder as Digit);
                            if quotient == 0 {
                                break BigNum { digits };
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

macro_rules! from_impl_check {
    ($($t:ty)*) => ($(
        impl From<$t> for BigNum  {
            #[inline]
            fn from(mut n: $t) -> BigNum {
                assert!(n >= 0);
                match n {
                    x @ 0..=9 => BigNum { digits: vec![x as Digit] },
                    _ => {
                        let mut digits = vec![];
                        loop {
                            let (quotient, remainder) = n.div_rem(&10);
                            digits.push(remainder as Digit);
                            if quotient == 0 {
                                break BigNum { digits };
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

from_impl_check! { i8 i32 i64 i128 }

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

test! {
    add_1: BigNum::from(22) + BigNum::from(50) => BigNum::from(72),
    add_2: BigNum::from(101) + BigNum::from(2002) => BigNum::from(2103),
    shl_1: BigNum::from(2) << 3 => BigNum::from(16),
    shl_2: BigNum::from(13) << 4 => BigNum::from(208),
    mul_1: BigNum::from(5) * BigNum::from(6) => BigNum::from(30),
    mul_2: BigNum::from(156) * BigNum::from(89) => BigNum::from(13884),
    mulint_1: BigNum::from(5) * 6 => BigNum::from(30),
    mulint_2: BigNum::from(2) * 3 => BigNum::from(6),
    addassign_int_1: { let mut a = BigNum::from(5); a += 6; a } => BigNum::from(11),
    addassign_int_2: { let mut a = BigNum::from(5); a += BigNum::from(6); a } => BigNum::from(11),
    mulassign_int_1: { let mut a = BigNum::from(5); a *= 6; a} => BigNum::from(30),
    mulassign_int_2: { let mut a = BigNum::from(5); a *= BigNum::from(6); a } => BigNum::from(30)
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
