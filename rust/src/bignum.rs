use benchtest::*;
use num_integer::Integer;
use std::ops::{Add, Mul, MulAssign, Shl};
use std::char;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct BigNum {
    digits: Vec<u32>,
}

impl BigNum {
    pub fn new(mut n: u32) -> Self {
        match n {
            x @ 0..=9 => Self { digits: vec![x] },
            _ => {
                let mut digits = vec![];
                loop {
                    let (quotient, remainder) = n.div_rem(&10);
                    digits.push(remainder);
                    if quotient == 0 {
                        break Self { digits };
                    } else {
                        n = quotient;
                    }
                }
            }
        }
    }

    pub fn digit_sum(&self) -> u32 {
        self.digits.iter().sum()
    }

    pub fn factorial(self) -> Self {
        let mut product = BigNum::new(1);
        let mut val = BigNum::new(2);
        loop {
            product = product * val.clone();
            if val == self {
                break product;
            } else {
                val = val + 1;
            }
        }
    }
}

impl From<Vec<u32>> for BigNum {
    fn from(item: Vec<u32>) -> Self {
        Self { digits: item }
    }
}

impl Mul<u32> for BigNum {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        match rhs {
            0 => BigNum::new(0),
            1 => self,
            2 => self.clone() + self,
            _ => self * BigNum::new(rhs),
        }
    }
}

impl MulAssign<usize> for BigNum {
    fn mul_assign(&mut self, rhs: usize) {
        *self = match rhs {
            0 => BigNum::new(0),
            1 => self.clone(),
            2 => self.clone() + self.clone(),
            _ => self.clone() * BigNum::new(rhs as u32),
        };
    }
}

impl fmt::Display for BigNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.digits.iter().rev().map(|&d| char::from_digit(d, 10).unwrap()).collect::<String>())
    }
}

impl Mul<BigNum> for BigNum {
    type Output = Self;

    fn mul(self, rhs: BigNum) -> Self {
        let zero = BigNum::new(0);
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

        temp_nums
            .into_iter()
            .fold(BigNum::new(0), |acc, x| acc + BigNum::from(x))
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
                    break new.into();
                }
            }
        }
    }
}

impl Add<u32> for BigNum {
    type Output = BigNum;

    fn add(self, rhs: u32) -> Self {
        self + BigNum::new(rhs)
    }
}

test! {
    add_1: BigNum::new(22) + BigNum::new(50) => BigNum::new(72),
    add_2: BigNum::new(101) + BigNum::new(2002) => BigNum::new(2103),
    shl_1: BigNum::new(2) << 3 => BigNum::new(16),
    shl_2: BigNum::new(13) << 4 => BigNum::new(208),
    mul_1: BigNum::new(5) * BigNum::new(6) => BigNum::new(30),
    mul_2: BigNum::new(156) * BigNum::new(89) => BigNum::new(13884),
    mulint_1: BigNum::new(5) * 6 => BigNum::new(30),
    mulint_2: BigNum::new(2) * 3 => BigNum::new(6)
}

benchtest! {
    mul_big_int_0: BigNum::new(6453264) * 0 => BigNum::new(0),
    mul_big_big_0: BigNum::new(6453264) * BigNum::new(0) => BigNum::new(0),
    mul_big_int_1: BigNum::new(6453264) * 1 => BigNum::new(6453264),
    mul_big_big_1: BigNum::new(6453264) * BigNum::new(1) => BigNum::new(6453264),
    mul_big_int_2: BigNum::new(6453264) * 2 => BigNum::new(12906528),
    mul_big_big_2: BigNum::new(6453264) * BigNum::new(2) => BigNum::new(12906528),
    mul_big_int_3: BigNum::new(6453264) * 3 => BigNum::new(19359792),
    mul_big_big_3: BigNum::new(6453264) * BigNum::new(3) => BigNum::new(19359792),
    mul_big_int_4: BigNum::new(6453264) * 4 => BigNum::new(25813056),
    mul_big_big_4: BigNum::new(6453264) * BigNum::new(4) => BigNum::new(25813056),
    mul_big_int_n: BigNum::new(6453264) * 37 => BigNum::new(238770768),
    mul_big_big_n: BigNum::new(6453264) * BigNum::new(37) => BigNum::new(238770768),
    rev_mul_big_big_0: BigNum::new(0) * BigNum::new(6453264) => BigNum::new(0),
    rev_mul_big_big_1: BigNum::new(1) * BigNum::new(6453264) => BigNum::new(6453264),
    rev_mul_big_big_2: BigNum::new(2) * BigNum::new(6453264) => BigNum::new(12906528),
    rev_mul_big_big_3: BigNum::new(3) * BigNum::new(6453264) => BigNum::new(19359792),
    rev_mul_big_big_4: BigNum::new(4) * BigNum::new(6453264) => BigNum::new(25813056),
    rev_mul_big_big_n: BigNum::new(37) * BigNum::new(6453264) => BigNum::new(238770768),
    fact_1: BigNum::new(6).factorial() => BigNum::new(720)
}
