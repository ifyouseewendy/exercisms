package diffsquares

func SquareOfSum(n int) int {
  sum := 0
  for i := 1; i <= n; i ++ {
    sum += i
  }
  return sum * sum
}

func SumOfSquares(n int) int {
  sumOfSquare := 0

  for i := 1; i <= n; i ++ {
    sumOfSquare += i * i
  }

  return sumOfSquare
}

func Difference(n int) int {
  return SquareOfSum(n) - SumOfSquares(n)
}

