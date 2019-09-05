module Diamond (diamond) where

import Data.Char (ord)

diamond :: Char -> Maybe [String]
diamond char | char `elem` ['A' .. 'Z'] = Just (assembleRows char)
             | otherwise                = Nothing

assembleRows :: Char -> [String]
assembleRows 'A'  = ["A"]
assembleRows char = map (assembleRow width) chars
  where
    width = distance char
    chars = ['A' .. char] ++ reverse ['A' .. (pred char)]

distance :: Char -> Int
distance char = ord char - ord 'A'

assembleRow :: Int -> Char -> String
assembleRow width char = infilling ++ content ++ infilling
  where
    currentDistance = distance char
    infilling = replicate (abs $ width - currentDistance) ' '
    content
      | char == 'A' = "A"
      | otherwise = [char] ++ replicate (2 * currentDistance - 1) ' ' ++ [char]
