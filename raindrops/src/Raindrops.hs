module Raindrops (convert) where

convert :: Int -> String
convert n =
  if null res then show n else res
  where
    res = concatMap (sound n) [(3, "Pling"), (5, "Plang"), (7, "Plong")]
    sound x (a, b)
      | mod x a == 0 = b
      | otherwise    = ""

