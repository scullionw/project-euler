import Text.Printf
import Control.Exception
import System.CPUTime

-- add time $ in front of what expression should be timed

time :: IO t -> IO t
time a = do
    start <- getCPUTime
    v <- a
    end   <- getCPUTime
    let diff = (fromIntegral (end - start)) / (10^12)
    printf "Computation time: %0.3f sec\n" (diff :: Double)
    return v


evendiv :: Integer -> Bool
evendiv n = sum (map (\x -> n `mod` x) [2..20]) == 0

main :: IO()
main = time $ print $ head $ filter evendiv [1..]