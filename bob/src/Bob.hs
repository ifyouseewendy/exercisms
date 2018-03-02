module Bob (responseFor) where

import Data.Char (isSpace, isUpper, isAlpha)

responseFor :: String -> String
responseFor xs
  | isEmpty                     = "Fine. Be that way!"
  | isYelling && isQuestioning  = "Calm down, I know what I'm doing!"
  | isYelling                   = "Whoa, chill out!"
  | isQuestioning               = "Sure."
  | otherwise                   = "Whatever."
  where
    xs' = filter (not . isSpace) xs
    letters = filter isAlpha xs'
    isEmpty = null xs'
    isYelling = all isUpper letters && any isUpper letters
    isQuestioning = last xs' == '?'
