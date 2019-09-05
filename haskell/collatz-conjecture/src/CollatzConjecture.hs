module CollatzConjecture (collatz) where

collatz :: Integer -> Maybe Integer
collatz n
  | n <= 0 = Nothing
  | otherwise = Just $ count n
  where
    count n
      | n == 1    = 0
      | even n    = 1 + count (n `div` 2)
      | otherwise = 1 + count (n * 3 + 1)
