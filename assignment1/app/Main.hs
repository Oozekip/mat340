module Main where

import Runner
import System.Random

main :: IO ()
main = do
    gen <- getStdGen
    i <- selectProblem

    i gen