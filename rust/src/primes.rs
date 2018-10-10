pub fn largest_prime_factor(mut n: u64) -> u64 {
    while n % 2 == 0 {
        n /= 2
    }
    if n == 1 {
        return 2;
    }
    let mut divisor = 1;
    while n != 1 {
        divisor += 2;
        while n % divisor == 0 {
            n /= divisor;
        }
    }
    divisor
}

pub fn primes_under_or_equal(n: u64) -> Vec<u64> {
    let mut primes = vec![];
    'a: for candidate in 2..=n {
        for divisor in 2..candidate {
            if candidate % divisor == 0 {
                continue 'a;
            }
        }
        primes.push(candidate);
    }
    primes
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_largest_prime_factor() {
        assert_eq!(super::largest_prime_factor(22), 11);
        assert_eq!(super::largest_prime_factor(8), 2);
    }

    #[test]
    fn test_primes_under_or_equal() {
        assert_eq!(super::primes_under_or_equal(20), vec![2,3,5,7,11,13,17,19]);
    }
}