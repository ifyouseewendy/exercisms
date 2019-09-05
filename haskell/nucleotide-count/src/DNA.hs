module DNA (nucleotideCounts) where

import Data.Map (Map, fromList, adjust)

nucleotideCounts :: String -> Either String (Map Char Int)
nucleotideCounts xs
  | any (`notElem` "ACGT") xs = Left xs
  | otherwise =
      Right (foldr (adjust (+ 1)) counter xs)
        where
          counter = fromList $ map (\t -> (t, 0 :: Int)) "ACGT"
