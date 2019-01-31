module Experiments
    (   coupons
        , avgCoupons
        , runCoupons
    )
where

import System.Random

avgCoupons :: RandomGen g => Int -> Int ->  g -> (Float, g)
avgCoupons t numTypes gen = (fromIntegral s / fromIntegral n, r)
    where   (results, r) = runCoupons t numTypes gen
            s = sum results
            n = length results

runCoupons :: RandomGen g => Int -> Int ->  g -> ([Int], g)
runCoupons 0 _ gen = ([], gen)
runCoupons t numTypes gen = (x:remaining, s)
    where   (x, r) = coupons numTypes gen
            (remaining, s) = runCoupons (t - 1) numTypes r

coupons :: RandomGen g => Int -> g -> (Int, g)
coupons numTypes gen = (count, r)
    where   collectedList = replicate numTypes False
            (count, r) = couponStep collectedList 0 gen


couponStep :: RandomGen g => [Bool] -> Int -> g -> (Int, g)
couponStep currList step gen
    | (foldr (&&) True currList) == True    = (step, gen)
    | otherwise                     = couponStep newList (step + 1) r
    where   (newList, r) = tryCoupon currList gen
    
tryCoupon :: RandomGen g => [Bool] -> g -> ([Bool], g)
tryCoupon oldList gen = (newList, r)
    where   n = (length oldList) - 1
            (changedIndex, r) = randomR (0, n) gen
            newList = update changedIndex oldList True

update :: Int -> [a] -> a -> [a]
update n xs y = take n xs ++ [y] ++ drop (n + 1) xs