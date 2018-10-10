extern crate euler;

fn solve(n: u64) -> u64 {
    euler::largest_prime_factor(n)
}

fn main() {
    euler::go(solve, 600_851_475_143);
    euler::bench(solve, 600_851_475_143, 1000);
}

#[cfg(test)]
mod tests {
    #[test]
    fn base_case() {
        assert_eq!(super::solve(13_195), 29);
    }
    #[test]
    fn correct_answer() {
        assert_eq!(super::solve(600_851_475_143), 6857);
    }
}