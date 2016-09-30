import Text.Printf
import Control.Exception
import System.CPUTime

-- add time $ in front of what expression should be timed
-- USE ghc --make -O / ghc --make -O2 to make program fast!

time :: IO t -> IO t
time a = do
    start <- getCPUTime
    v <- a
    end   <- getCPUTime
    let diff = (fromIntegral (end - start)) / (10^12)
    printf "Computation time: %0.3f sec\n" (diff :: Double)
    return v

main::IO ()
main = time $ print $ sum $ filter (\x -> ((x `mod` 3) == 0) || ((x `mod` 5) == 0)) [1..100000000 :: Integer]