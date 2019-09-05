module ETL (transform) where

import Data.Map (Map, empty, member, insert, insertWith, foldrWithKey)
import Data.Char (toLower)

-- [(1, "AE"), (2, "IOU")] --> [('a', 1), ('e', 1) .. ]
transform :: Map Int String -> Map Char Int
transform = foldrWithKey f empty
  where
    f _ [] acc = acc
    f k (x:xs) acc = f k xs newAcc
      where
        newAcc
          | member x acc = f k xs (insertWith (+) (toLower x) k acc)
          | otherwise = f k xs (insert (toLower x) k acc)
