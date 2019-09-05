module Acronym (abbreviate) where

import Data.Char (isAlpha, isUpper, toUpper)

abbreviate :: String -> String
abbreviate xs =
  scan xs False
  where
    scan (a:b:tail) lastIsUpper
      | null tail = ""
      | lastIsUpper = scan (b:tail) (isUpper a)
      | isUpper a = a : scan (b:tail) True
      | (not . isAlpha $ a) && isAlpha b = toUpper b : scan tail True
      | otherwise = scan (b:tail) False



