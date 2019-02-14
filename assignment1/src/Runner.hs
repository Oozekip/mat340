module Runner(
    selectProblem
    , runProblem
)where

import Data.List
import Data.Functor
import Control.Monad
import Text.Read
import System.Random

import Experiments

readProblem :: RandomGen g => String -> Maybe (g -> IO())
readProblem "1" = Just runCoupons
readProblem "2" = Just playMontyHall
readProblem "3" = Just runMontyHall
readProblem "4" = Just runGamblers
readProblem _ = Nothing

selectProblem :: RandomGen g => IO (g -> IO ())
selectProblem = do
    putStrLn "Pick an experiment"
    putStrLn "1) Coupons"
    putStrLn "2) Monty Hall (interactive)"
    putStrLn "3) Monty Hall"
    putStrLn "4) Gamblers Ruin"
    input <- getLine

    let selected = readProblem input

    case selected of
        Just a -> return a
        Nothing -> do
            putStrLn "Invalid experiment selected\n"
            selectProblem

runProblem :: RandomGen g => Int -> g -> IO ()
runProblem 1 gen = runCoupons gen
runProblem 2 gen = runMontyHall gen
runProblem 3 gen = runGamblers gen
runProblem i _ = putStrLn $ "Invalid problem selected: " ++ show i

readNumber :: Read a => String -> IO a
readNumber promptString = readNumberUntil (\_ -> True) promptString ""

readNumberUntil :: Read a => (a -> Bool) -> String -> String -> IO a
readNumberUntil f prompt rangeErr = do 
    putStrLn prompt
    input <- getLine

    case readMaybe input of
        Just i -> if f i 
                    then return i
                    else do
                        putStrLn rangeErr
                        readNumberUntil f prompt rangeErr
        Nothing -> do
            putStrLn "Input must be a number"
            readNumberUntil f prompt rangeErr

runCoupons :: RandomGen g => g -> IO ()
runCoupons gen = do
    i <- readNumber "How many coupons are there total?"
    t <- readNumber "How many trials should be ran?"

    let (avg, r) = avgCoupons t i gen

    putStrLn $ "Average coupon draws: " ++ show avg 

playMontyHall :: RandomGen g => g -> IO ()
playMontyHall gen = do
    numDoors <- readNumber "How many doors are there?"
    numCars <- readNumberUntil (numDoors >=) "How many cars are there?" "Input must be <= number of doors"
    toOpen <- readNumberUntil ((numDoors - numCars - 1) >=) "How many doors should be opened?" "Input must be <= (number of doors - number of cars - 1)"
    selected <- readNumberUntil (numDoors >=) "Pick a door" "Input must be <= number of doors"

    let (doors, r) = montyHall numDoors numCars gen
        (montyOpen, s) = montyHallOpen doors (selected - 1) toOpen r
        (opened', closed') = partition (\(x, _) -> isOpen x) $ zip montyOpen [1..]
        (_, opened) = unzip opened'
        (_, closed) = unzip closed'
    
    putStrLn $ "Opened doors: " ++ show opened
    selected <- readNumberUntil (\i -> i `elem` closed) ("Select a new closed door: " ++ show closed) "Must select a closed door"
    
    let result = montyOpen !! (selected - 1)

    if isCar result
        then putStrLn "You won a car!"
        else putStrLn "You won a goat!"

runMontyHall :: RandomGen g => g -> IO ()
runMontyHall gen = do
    numDoors <- readNumber "How many doors are there?"
    numCars <- readNumberUntil (numDoors >=) "How many cars are there?" "Input must be <= number of doors"
    toOpen <- readNumberUntil ((numDoors - numCars - 1) >=) "How many doors should be opened?" "Input must be <= (number of doors - number of cars - 1)"
    trials <- readNumber "How many trials should be ran?"

    let (pct, _) = pctMontyHall numDoors numCars toOpen trials gen

    putStrLn $ "Win rate was " ++ show (pct * 100)

runGamblers :: RandomGen g => g -> IO ()
runGamblers gen = do
    a <- readNumber "How much money to start with?"
    b <- readNumber "What is the goal amount of money?"
    p <- readNumberUntil (\i -> 0 <= i && i <= 1) "What is the win percentage (0-1)" "Input must be a range 0-1"
    trials <- readNumber "How many trials should be ran?"

    let (wins, _) = countGamblersRuin a b p trials gen

    putStrLn $ "The player won " ++ show wins ++ " out of " ++ show trials ++ " times."