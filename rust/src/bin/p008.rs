extern crate euler;


fn solve(seq: &str) -> u64 {
    seq .lines()
        .fold(String::new(), |a, b| a + b)
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>()
        .as_slice()
        .windows(13)
        .map(|x| x.iter().product())
        .max()
        .unwrap()
}

fn main() {
    let data = include_str!("data/p008_data.txt");
    euler::go(solve, data);
    euler::bench(solve, data, 10);
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn correct_answer() {
        let data = include_str!("data/p008_data.txt");
        assert_eq!(super::solve(data), 23514624000);
    }
}