module Isogram (isIsogram) where

import qualified Data.Char (toLower)
import qualified Data.List (nub)

isIsogram :: String -> Bool
isIsogram string =
  length (Data.List.nub $ map Data.Char.toLower alphaString) == length alphaString
  where
    alphaString = filter (\x -> x /= ' ' && x /= '-') string
