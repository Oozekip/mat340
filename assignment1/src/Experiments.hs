module Experiments
    (   coupons
        , avgCoupons
        , isOpen
        , isCar
        , montyHallOpen
        , montyHall
        , pctMontyHall
        , countGamblersRuin
        , gamblersRuin
    )
where

import Utils
import System.Random

data Prize = Car | Goat deriving(Show, Eq)
data Door = Open Prize | Closed Prize deriving(Show, Eq)

isOpen :: Door -> Bool
isOpen (Open _) = True
isOpen _ = False

isCar :: Door -> Bool
isCar (Open Car) = True
isCar (Closed Car) = True
isCar _ = False

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

pctMontyHall :: RandomGen g => Int -> Int -> Int -> Int -> g -> (Float, g)
pctMontyHall d c o n gen = (fromIntegral wins / fromIntegral n, r)
    where (wins, r) = countMontyHall d c o n gen

countMontyHall :: RandomGen g => Int -> Int -> Int -> Int -> g -> (Int, g)
countMontyHall d c o 0 gen = (0, gen)
countMontyHall d c o n gen = if won then (1 + next, r)
                                    else (next, r)
    where   (won, s) = stepMontyHall d c o gen
            (next, r) = countMontyHall d c o (n -  1) s

stepMontyHall :: RandomGen g => Int -> Int -> Int -> g -> (Bool, g)
stepMontyHall d c o gen = (isCar (doors !! switched), r)
    where   (doors, s) = montyHall d c gen
            (selected, t) = randomR (0, length doors - 1) s
            (oDoors, u) = montyHallOpen doors selected o t
            possibleSwitches = [i | (x, i) <- enumerate doors, i /= selected, not $ isOpen x]
            (switched, _, r) = choose possibleSwitches u

montyHall :: RandomGen g => Int -> Int -> g -> ([Door], g)
montyHall numDoors numCars gen = (doors, r)
    where   (carDoors, r) = chooseN [1..numDoors] numCars gen
            doors = map (\i -> if i `elem` carDoors then Closed Car else Closed Goat) [1..numDoors]

montyHallOpen :: RandomGen g => [Door] -> Int -> Int -> g -> ([Door], g)
montyHallOpen doors selected n gen = (opened, r)
    where   possibleDoors = [i | (x, i) <- enumerate doors, x == Closed Goat, i /= selected]
            (toOpen, r) = chooseN possibleDoors n gen
            opened = openDoors doors toOpen
            
openDoors :: [Door] -> [Int] -> [Door]
openDoors xs [] = xs
openDoors xs (y:ys) = openDoors (updateWith y xs openDoor) ys

openDoor :: Door -> Door
openDoor (Closed x) = Open x
openDoor x = x

countGamblersRuin :: RandomGen g => Int -> Int -> Float -> Int -> g -> (Int, g)
countGamblersRuin a b p t gen = sumVictories gen $ take t results
    where results = gamblersRuin a b p gen

playForever :: RandomGen g => Int -> Int -> Float -> g -> [(Bool, g)]
playForever a b p gen = (result, gen'):playForever a b p gen'
    where (result, gen') = gamblersRuin a b p gen

sumVictories :: g -> [(Bool, g)] -> (Int, g)
sumVictories gen results = foldr (\(r, gen) (a, genA) -> if r then ((a + 1), gen) else (a, gen)) results gen

gamblersRuin :: RandomGen g => Int -> Int -> Float -> g -> (Bool, g)
gamblersRuin a b p gen = playUntil a b games
    where games = genGamblersRuin p gen

playUntil :: Int -> Int -> [Int] -> (Bool, g)
playUntil 0 _ ((_, gen):xs)         = (False, gen)
playUntil a b ((x, gen):xs)    = if a >= b
                            then (True, gen)
                            else playUntil (a + x) b xs

genGamblersRuin :: RandomGen g => Float -> g -> [(Int, g)]
genGamblersRuin p gen   = (didWin,gen'):genGamblersRuin p gen'
    where   (x, gen')   = random gen
            didWin      = if x > (1 - p)
                            then 1
                            else -1

gamble :: RandomGen g => Int -> Float -> g -> (Int, g)
gamble a p gen = if result > (1 - p) 
                    then (a + 1, r)
                    else (a - 1, r)
    where (result, r) = random gen