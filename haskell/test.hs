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



calc n = [x^2 | x <- [1..n], x^2 `mod` 2 == 0]
calc' n = [ans| x <- [1..n], let ans = x^2, ans `mod` 2 == 0] 

main :: IO()
main = time $ print $ (calc' 1000000) == (calc' 1000000)


