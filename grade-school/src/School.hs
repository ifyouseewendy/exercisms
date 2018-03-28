module School (School, add, empty, grade, sorted) where

import Data.Map (Map)
import qualified Data.Map as Map
import Data.Maybe (fromJust, isNothing)
import Data.List (sort, sortOn)

type School = Map Int [String]

add :: Int -> String -> School -> School
add gradeNum student school =
  Map.insert gradeNum (student : students) school
    where students = grade gradeNum school

empty :: School
empty = Map.empty

grade :: Int -> School -> [String]
grade gradeNum school
  | isNothing v = []
  | otherwise = sort (fromJust v)
    where v = (Map.!?) school gradeNum

sorted :: School -> [(Int, [String])]
sorted school =
  map (\t -> (fst t, sort (snd t))) (sortOn fst list)
  where list = Map.toList school
