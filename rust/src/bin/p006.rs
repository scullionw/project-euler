extern crate euler;

#[cfg(feature = "fp")]
fn solve(n: u64) -> u64 {
    let sum_of_squares = (1..=n).map(|x| x.pow(2)).sum::<u64>();
    let square_of_sum = (1..=n).sum::<u64>().pow(2);
    square_of_sum - sum_of_squares
}


#[cfg(not(feature = "fp"))]
fn solve(n: u64) -> u64 {
    let mut sum = 0;
    let mut sum_of_squares = 0;
    for i in 1..=n {
        sum += i;
        sum_of_squares += i.pow(2);
    }
    sum.pow(2) - sum_of_squares
}

fn main() {
    euler::go(solve, 100);
    euler::bench(solve, 100, 10);
}

#[cfg(test)]
mod tests {
    #[test]
    fn base_case() {
        assert_eq!(super::solve(10), 2640);
    }
    #[test]
    fn correct_answer() {
        assert_eq!(super::solve(100), 25164150);
    }
}