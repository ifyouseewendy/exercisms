module School (School, add, empty, grade, sorted) where

import Data.Map (Map)
import qualified Data.Map as Map
import Data.List (sort, sortOn)

type School = Map Int [String]

add :: Int -> String -> School -> School
add gradeNum student school =
  Map.insert gradeNum (student : students) school
    where students = grade gradeNum school

empty :: School
empty = Map.empty

grade :: Int -> School -> [String]
grade gradeNum school =
  sort $ Map.findWithDefault [] gradeNum school

sorted :: School -> [(Int, [String])]
sorted school =
  map sortSnd sortedWithGrade
  where
    sortSnd (first, second) = (first, sort second)
    sortedWithGrade = sortOn fst list
    list = Map.toList school
