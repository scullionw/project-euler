#![feature(test)]

use benchtest::*;
use num_integer::Integer;

const BASE_CASE_INPUT: u32 = 5;
const PROBLEM_INPUT: u32 = 1000;

fn solve(n: u32) -> usize {
    (1..=n).map(|x| spell(x).len()).sum()
}

fn spell(n: u32) -> String {
    assert!(n <= 1000);

    if n == 1000 {
        return "onethousand".to_owned();
    }

    let (hundreds, rest) = n.div_rem(&100);
    let (tens, ones) = rest.div_rem(&10);

    let mut word = String::new();

    if hundreds > 0 {
        word.push_str(uniques(hundreds));
        word.push_str(uniques(100));

        if rest > 0 {
            word.push_str("and");
        } else {
            return word;
        }
    }

    if rest <= 20 {
        word.push_str(uniques(rest));
    } else {
        word.push_str(uniques(tens * 10));
        if ones > 0 {
            word.push_str(uniques(ones));
        }
    }

    word
}

fn uniques(n: u32) -> &'static str {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        100 => "hundred",
        _ => unreachable!(),
    }
}

fn main() {
    println!("{:?}", solve(PROBLEM_INPUT));
}

benchtest! {
    base_case: solve(BASE_CASE_INPUT) => 19,
    problem_solve: solve(PROBLEM_INPUT) => 21124
}
