module Experiments (
    MontyHall
) where

import System.Random

data Door = Goat | Car

data MontyHall = MontyHall{ 
                            doors :: [Door], 
                            goatDoors :: [Integer]
                            }

-- montyHall :: (Random Integer -> Integer -> Integer) -> (Random, MontyHall)
-- montyHall r numDoors numCars = 