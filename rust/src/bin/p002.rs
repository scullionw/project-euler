extern crate euler;

#[cfg(feature = "fp")]
use euler::Fibonnaci;

#[cfg(not(feature = "fp"))]
fn solve(n: u64) -> u64 {
    let mut a = 1;
    let mut b = 2;
    let mut sum = 0;
    while b < n {
        if b % 2 == 0 {
            sum += b;
        }
        // Next fibonnaci value (b)
        // Consider switching (and benching)    let (a, b) = (b, a + b);
        let temp_a = a;
        a = b;
        b = temp_a + b;
    }
    sum
}

#[cfg(feature = "fp")]
fn solve(n: u64) -> u64 {
    Fibonnaci::new().take_while(|&x| x < n)
                    .filter(|&x| x % 2 == 0)
                    .sum()
}

fn main() {
    euler::go(solve, 4_000_000);
    euler::bench(solve, 4_000_000, 1000);
}

#[cfg(test)]
mod tests {
    #[test]
    fn correct_answer() {
        assert_eq!(super::solve(4_000_000), 4613732)
    }
}