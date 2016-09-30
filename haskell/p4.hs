main :: IO() -- why is a=100 matched with b=101? doesn't work in python
main = let n = 999 in print $ maximum [a * b | a <- [100..n], b <- [100..n], show (a * b) == (reverse $ show (a * b))]