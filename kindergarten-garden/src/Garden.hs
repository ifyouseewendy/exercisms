module Garden
    ( Plant (..)
    , garden
    , lookupPlants
    ) where

import qualified Data.Map.Strict as Map
import           Data.Map.Strict (Map, (!))
import Data.List.Split (chunksOf)

data Plant = Clover
           | Grass
           | Radishes
           | Violets
           deriving (Eq, Show)

type Garden = Map String [Plant]

-- garden ["Alice", "Bob"] "RRGG\nRRGG"
-- > fromList [("Alice", [Radishes, Radishes, Radishes, Radishes]), ("Bob", [Grass, Grass, Grass, Grass])]
garden :: [String] -> String -> Garden
garden students plants =
  Map.fromList $ zip students (preprocess plants)

lookupPlants :: String -> Garden -> [Plant]
lookupPlants student garden =
  garden ! student

-- "AABB\nAABB"
-- [[Alfa, Alfa, Alfa, Alfa], [Beta, Beta, Beta, Beta]]
preprocess :: String -> [[Plant]]
preprocess plants =
  map toPlant zippedChunks
  where
    zippedChunks = zip firstRowInChunk secondRowInChunk -- [("AA", "AA"), ("BB", "BB")]
    [firstRowInChunk, secondRowInChunk] = map (chunksOf 2) $ lines plants -- [["AA", "BB"], ["AA", "BB"]]
    toPlant (a, b) = map dict (a ++ b)
    dict 'C' = Clover
    dict 'G' = Grass
    dict 'R' = Radishes
    dict 'V' = Violets
