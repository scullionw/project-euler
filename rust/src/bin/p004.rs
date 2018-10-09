extern crate euler;


fn solve(digits: u64) -> u64 {
   let start = 10_u64.pow(digits as u32 - 1);
   let end = 10_u64.pow(digits as u32);
   let mut largest = 0;
   for x in start..end {
       for y in x..end {
            let product = x * y;
            if product > largest {
                let repr = product.to_string();
                if repr == repr.chars().rev().collect::<String>() {
                    largest = product;
                }
            }
       }
   }
   largest
}

fn main() {
    euler::go(solve, 3);
    euler::bench(solve, 3, 10);
}

#[cfg(test)]
mod tests {
    #[test]
    fn base_case() {
        assert_eq!(super::solve(2), 9009);
    }
    #[test]
    fn correct_answer() {
        assert_eq!(super::solve(3), 906609);
    }
}
