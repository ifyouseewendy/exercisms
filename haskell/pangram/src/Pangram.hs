module Pangram (isPangram) where

import Data.List (nubBy)
import Data.Char (isAlpha, toLower)

isPangram :: String -> Bool
isPangram text =
  length uniqLetters == 26
  where
    uniqLetters = nubBy lowerEqual letters
    lowerEqual a b = toLower a == toLower b
    letters = filter isAlpha text
