primes :: [Integer]
primes = sieve [2..]
    where
        sieve (p:xs) = p:sieve[x | x <- xs, x `mod` p > 0]

findMaxPrimeFactor :: Integer -> Integer
findMaxPrimeFactor n = findMaxPrimeFactor' n primes

findMaxPrimeFactor' 1 (x:xs) = x -- What type is this?
findMaxPrimeFactor' n (x:xs) | n `mod` x == 0 = findMaxPrimeFactor' (n `div` x) (x:xs)
                             | otherwise      = findMaxPrimeFactor' n xs
main :: IO()
main = print $ findMaxPrimeFactor 600851475143
