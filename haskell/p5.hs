evendiv :: [Integer] -> Integer -> Bool
evendiv [] _ = True
evendiv (x:xs) n  = if n `mod` x > 0 then False else evendiv xs n

main :: IO()
main = print $ head $ filter (evendiv [2..20]) [2,4..]       --  foldl (lcm) 1 [1..20]q