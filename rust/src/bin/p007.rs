extern crate euler;


fn solve(n: u64) -> u64 {
    euler::PrimeSequence::new().nth((n - 1) as usize).unwrap()
}

fn main() {
    euler::go(solve, 10001);
    euler::bench(solve, 10001, 10);
}

#[cfg(test)]
mod tests {
    #[test]
    fn base_case() {
        assert_eq!(super::solve(6), 13);
    }
    #[test]
    fn correct_answer() {
        assert_eq!(super::solve(10001), 104743);
    }
}