module PerfectNumbers (classify, Classification(..)) where

data Classification = Deficient | Perfect | Abundant deriving (Eq, Show)

classify :: Int -> Maybe Classification
classify num
  | num <= 0           = Nothing
  | aliquot num == num = Just Perfect
  | aliquot num > num  = Just Abundant
  | aliquot num < num  = Just Deficient
  where
    aliquot = sum . factors
    factors n = filter (divisible n) [1..n-1]
    divisible n a = n `mod` a == 0
