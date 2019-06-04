use benchtest::*;
use std::iter;
use std::mem;

pub fn fibonacci() -> impl Iterator<Item = u64> {
    let mut a = 0;
    let mut b = 1;
    iter::repeat_with(move || {
        let a = mem::replace(&mut a, b);
        b += a;
        a
    })
}

pub fn div_mod(n: u64, divisor: u64) -> (u64, u64) {
    let quotient = n / divisor;
    (quotient, n - divisor * quotient)
}

test! {
    fib: fibonacci().take(10).collect::<Vec<_>>() => {
        vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
    }
}
