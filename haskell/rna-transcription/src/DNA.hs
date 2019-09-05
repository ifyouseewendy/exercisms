module DNA (toRNA) where

toRNA :: String -> Maybe String
toRNA xs =
  if all (`elem` ['A', 'C', 'G', 'T']) xs
  then Just (map trans xs)
  else Nothing
  where
    trans :: Char -> Char
    trans c
      | c == 'G' = 'C'
      | c == 'C' = 'G'
      | c == 'T' = 'A'
      | c == 'A' = 'U'

