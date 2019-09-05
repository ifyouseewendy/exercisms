module Triangle (TriangleType(..), triangleType) where

data TriangleType = Equilateral
                  | Isosceles
                  | Scalene
                  | Illegal
                  deriving (Eq, Show)

triangleType :: (Num a, Ord a) => a -> a -> a -> TriangleType
triangleType a b c
  | (a + b <= c) || (a + c <= b) || (b + c <= a)  = Illegal
  | and equalSides                                = Equilateral
  | or equalSides                                 = Isosceles
  | otherwise                                     = Scalene
  where
    equalSides = [a == b, a == c, b == c]
