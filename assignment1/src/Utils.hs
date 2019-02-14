module Utils (
    enumerate
    , update
    , updateWith
    , chooseN
    , choose
    , dropN
) where

import Data.List
import System.Random

enumerate :: [a] -> [(a, Int)]
enumerate xs = zip xs [0..]

update :: Int -> [a] -> a -> [a]
update n xs y = updateWith n xs (\_ -> y)

updateWith :: Int -> [a] -> (a -> a) -> [a]
updateWith n xs f = take n xs ++ [x'] ++ drop (n + 1) xs
    where   x' = f $ xs !! n

chooseN :: RandomGen g => [a] -> Int -> g -> ([a], g)
chooseN xs 0 gen = ([], gen)
chooseN xs n gen = (y:zs, r)
    where   (y, ys, s) = choose xs gen
            (zs, r) = chooseN ys (n - 1) s

choose :: RandomGen g => [a] -> g -> (a, [a], g)
choose xs gen = (y, ys, r)
    where   (n, r) = randomR (0, (length xs - 1)) gen
            (y, ys) = dropN xs n

dropN :: [a] -> Int -> (a, [a])
dropN (x:xs) 0 = (x, xs)
dropN (x:xs) n = (y, x:ys)
    where (y, ys) = dropN xs (n - 1)
    
