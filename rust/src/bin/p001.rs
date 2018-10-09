extern crate euler;


#[cfg(not(feature = "fp"))]
fn solve(n: u64) -> u64 {
    let mut sum = 0;
    for i in 1..n {
        if (i % 3 == 0) || (i % 5 == 0) {
            sum += i;
        } 
    }
    sum
}

#[cfg(feature = "fp")]
fn solve(n: u64) -> u64 {
    (1..n).filter(|x| (x % 3 == 0) || (x % 5 == 0)).sum()
}

fn main() {
    euler::go(solve, 1000);
    euler::bench(solve, 1000, 1000);
}

#[cfg(test)]
mod tests {
    #[test]
    fn base_case() {
        assert_eq!(super::solve(10), 23);
    }
    #[test]
    fn correct_answer() {
        assert_eq!(super::solve(1000), 233168);
    }
}
