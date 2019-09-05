module SumOfMultiples (sumOfMultiples) where

import Data.List (union, foldl)

sumOfMultiples :: [Integer] -> Integer -> Integer
sumOfMultiples factors limit =
  sum uniqMultiples
  where
    uniqMultiples = foldl union [] allMultiples
    allMultiples = map (\n -> [n,n+n..limit-1]) factors
