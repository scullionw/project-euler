#![feature(test)]

use benchtest::*;
use euler::BigNum;
use num_bigint::{BigInt, ToBigInt, BigUint};
use ramp::Int;
use std::ops::{Mul, MulAssign};
use num_traits::identities::One;

const BASE_CASE_INPUT: u32 = 10;
const PROBLEM_INPUT: u32 = 100;

fn solve(n: u32) -> u32 {
    BigNum::new(n).factorial().digit_sum()
}

fn solve_with_dep<T: MulAssign<usize> + OneExt + ToString>(n: usize) -> u32 {
    factorial::<T>(n)
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .sum()
}

fn factorial<T: MulAssign<usize> + OneExt>(n: usize) -> T {
    let mut a = T::one_ext();
    for i in 2..=n {
        a *= i;
    }
    a
}

trait OneExt {
    fn one_ext() -> Self;
}

impl OneExt for BigInt {
    fn one_ext() -> Self {
        BigInt::one()
    }
}

impl OneExt for Int {
    fn one_ext() -> Self {
        Int::from(1)
    }
}

impl OneExt for BigNum {
    fn one_ext() -> Self {
        BigNum::new(1)
    }
}

fn main() {
    println!("{:?}", solve(PROBLEM_INPUT));
}

benchtest! {
    base_case: solve(BASE_CASE_INPUT) => 27,
    problem_solve: solve(PROBLEM_INPUT) => 648,

    base_case2: solve_with_dep::<BigNum>(BASE_CASE_INPUT as usize) => 27,
    problem_solve2: solve_with_dep::<BigNum>(PROBLEM_INPUT as usize) => 648,

    base_case_bigint: solve_with_dep::<BigInt>(BASE_CASE_INPUT as usize) => 27,
    problem_solve_bigint: solve_with_dep::<BigInt>(PROBLEM_INPUT as usize) => 648,

    base_case_ramp: solve_with_dep::<Int>(BASE_CASE_INPUT as usize) => 27,
    problem_solve_ramp: solve_with_dep::<Int>(PROBLEM_INPUT as usize) => 648
}
